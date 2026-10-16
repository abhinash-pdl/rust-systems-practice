use std::ffi::CString;

fn main() {
    let mut failed = false;
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (opts, files) = match parse_args(args) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("mywc: {}", e);
            std::process::exit(1);
        }
    };

    if files.is_empty() {
        match count_fd(0) {
            Ok(c) => print_counts(&opts, &c, None),
            Err(e) => {
                eprintln!(" stdin: {}", e);
                failed = true;
            }
        }
    } else {
        for path in &files {
            match count_path(path) {
                Ok(c) => print_counts(&opts, &c, Some(path)),
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

struct Opts {
    lines: bool,
    words: bool,
    bytes: bool,
}
fn parse_args(args: Vec<String>) -> Result<(Opts, Vec<String>), String> {
    let mut opts = Opts {
        lines: false,
        words: false,
        bytes: false,
    };
    let mut files = Vec::new();

    for arg in args {
        if arg.starts_with('-') && arg.len() > 1 {
            for ch in arg[1..].chars() {
                match ch {
                    'l' => opts.lines = true,
                    'w' => opts.words = true,
                    'c' => opts.bytes = true,
                    _ => return Err(format!("Invalid argument: {}", ch)),
                }
            }
        } else {
            files.push(arg);
        }
    }
    if !opts.lines && !opts.words && !opts.bytes {
        opts.lines = true;
        opts.words = true;
        opts.bytes = true;
    }
    Ok((opts, files))
}

fn print_counts(opts: &Opts, c: &Counts, name: Option<&str>) {
    let mut parts: Vec<String> = Vec::new();
    if opts.lines {
        parts.push(format!("{} ", c.lines));
    }
    if opts.words {
        parts.push(format!("{} ", c.words));
    }
    if opts.bytes {
        parts.push(format!("{} ", c.bytes));
    }
    let mut line = parts.join(" ");
    if let Some(n) = name {
        line.push(' ');
        line.push_str(n);
    }
    println!("{}", line);
}
