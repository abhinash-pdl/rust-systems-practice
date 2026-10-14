use std::ffi::CString;

fn main() {
    let mut failed = false;
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        if let Err(e) = copy_to_stdout(0) {
            eprintln!("stdin: {}", e);
            failed = true;
        }
    } else {
        for path in &args {
            if let Err(e) = cat_file(&path) {
                eprintln!("{}: {}", path, e);
                failed = true;
            }
        }
    }
    if failed {
        std::process::exit(1);
    }
}
fn write_all(fd: i32, buf: &[u8]) -> Result<(), std::io::Error> {
    let mut done = 0;
    while done < buf.len() {
        let w = unsafe {
            libc::write(
                fd,
                buf.as_ptr().add(done) as *const libc::c_void,
                buf.len() - done,
            )
        };
        if w < 0 {
            let err =std::io::Error::last_os_error();
            if err.kind() == std::io::ErrorKind::Interrupted{
                continue;
            }
            return Err(err);

        }
        done += w as usize;
    }
    Ok(())
}

fn cat_file(path: &str) -> Result<(), std::io::Error> {
    let c_path = CString::new(path).unwrap();
    let fd = unsafe { libc::open(c_path.as_ptr(), libc::O_RDONLY) };
    if fd < 0 {
        return Err(std::io::Error::last_os_error());
    }
    let result = copy_to_stdout(fd);
    unsafe {
        libc::close(fd);
    }
    result
}

fn copy_to_stdout(fd: i32) -> Result<(), std::io::Error> {
    let mut buf = [0u8; 4096];
    loop {
        let n = unsafe { libc::read(fd, buf.as_mut_ptr() as *mut libc::c_void, buf.len()) };
        if n == 0 {
            return Ok(());
        }
        if n < 0 {
            let err = std::io::Error::last_os_error();
            if err.kind() == std::io::ErrorKind::Interrupted{
                continue;
            }
            return Err(err);
        }
        write_all(1, &buf[..n as usize])?;
    }
}
