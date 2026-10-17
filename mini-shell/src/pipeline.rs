use std::ffi::CString;
fn exec_words(words: &[&str]) -> ! {
    let c_words: Vec<CString> = words.iter().map(|w| CString::new(*w).unwrap()).collect();
    let mut argv: Vec<*const libc::c_char> = c_words.iter().map(|w| w.as_ptr()).collect();
    argv.push(std::ptr::null());
    unsafe { libc::execvp(c_words[0].as_ptr(), argv.as_ptr()) };
    eprintln!("exec failed: {}", std::io::Error::last_os_error());
    unsafe { libc::_exit(127) };
}

pub fn run_pipeline(left: &[&str], right: &[&str]) {
    let mut fds = [0i32; 2];
    if unsafe { libc::pipe(fds.as_mut_ptr()) } == -1 {
        eprintln!("pipe failed: {}", std::io::Error::last_os_error());
        return;
    }
    let (read_end, write_end) = (fds[0], fds[1]);
    let pid1 = unsafe { libc::fork() };
    if pid1 == 0 {
        unsafe {
            libc::close(read_end);
            libc::dup2(write_end, 1);
            libc::close(write_end);
        }
        exec_words(left);
    }
    let pid2 = unsafe { libc::fork() };
    if pid2 == 0 {
        unsafe {
            libc::close(write_end);
            libc::dup2(read_end, 0);
            libc::close(read_end);
        }

        exec_words(right);
    }

    unsafe {
        libc::close(read_end);
        libc::close(write_end);
        let mut status: i32 = 0;
        libc::waitpid(pid1, &mut status, 0);
        libc::waitpid(pid2, &mut status, 0);
    }
}
