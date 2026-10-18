use std::ffi::CString;
pub fn split_redirect<'a>(
    words: &'a [&'a str],
) -> Result<(&'a [&'a str], Option<&'a str>), String> {
    match words.iter().position(|w| *w == ">") {
        None => Ok((words, None)),
        Some(pos) => {
            if pos == 0 {
                return Err("Missing command before >".to_string());
            }
            match words.get(pos + 1) {
                Some(file) => Ok((&words[..pos], Some(*file))),
                None => Err("Missing file name after >".to_string()),
            }
        }
    }
}

pub fn redirect_stdout(name: &str) {
    let c_name = CString::new(name).unwrap();
    let fd = unsafe {
        libc::open(
            c_name.as_ptr(),
            libc::O_WRONLY | libc::O_CREAT | libc::O_TRUNC,
            0o644,
        )
    };
    if fd == -1 {
        eprintln!("mysh: {}: {}", name, std::io::Error::last_os_error());
        unsafe { libc::_exit(1) };
    }
    unsafe {
        libc::dup2(fd, 1);
        libc::close(fd);
    }
}
