use std::ffi::CString;

fn main() {
    let mut failed = false;
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        match count_fd(0) {
            Ok(c) => println!(" {:>7} {:7} {:>7}", c.lines, c.words, c.bytes),
            Err(e) => {
                eprintln!(" stdin: {}", e);
                failed = true;
            }
        }
    } else {
        for path in &args {
            match count_path(path) {
                Ok(c) => println!(" {} {} {} {}", c.lines, c.words, c.bytes, path),
                Err(e) => {
                    eprintln!(" {}: {}", path, e);
                    failed = true;
                }
            }
        }
    }
    if failed {
        std::process::exit(1);
    }
}
fn count_path(path: &str) -> Result<Counts, std::io::Error> {
    let c_path = CString::new(path).unwrap();
    let fd = unsafe { libc::open(c_path.as_ptr(), libc::O_RDONLY) };
    if fd < 0 {
        return Err(std::io::Error::last_os_error());
    }
    let result = count_fd(fd);
    unsafe {
        libc::close(fd);
    }
    result
}

struct Counts {
    lines: usize,
    words: usize,
    bytes: usize,
}

fn count_fd(fd: i32) -> Result<Counts, std::io::Error> {
    let mut counts = Counts {
        lines: 0,
        words: 0,
        bytes: 0,
    };
    let mut buf = [0u8; 4096];
    let mut in_word = false;
    loop {
        let n = unsafe { libc::read(fd, buf.as_mut_ptr() as *mut libc::c_void, buf.len()) };
        if n == 0 {
            return Ok(counts);
        }
        if n < 0 {
            let err = std::io::Error::last_os_error();
            if err.kind() == std::io::ErrorKind::Interrupted {
                continue;
            }
            return Err(err);
        }
        let chunks = &buf[..n as usize];
        counts.bytes += chunks.len();

        for &b in chunks {
            if b == b'\n' {
                counts.lines += 1;
            }
            if matches!(b, b' ' | b'\n' | b'\t' | b'\r') {
                in_word = false;
            } else if !in_word {
                counts.words += 1;
                in_word = true;
            }
        }
    }
}
