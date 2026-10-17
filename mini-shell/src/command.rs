use std::ffi::CString;
pub fn run_command(words: &[&str]) {
    let c_words: Vec<CString> = words.iter().map(|w| CString::new(*w).unwrap()).collect();
    let mut argv: Vec<*const libc::c_char> = c_words.iter().map(|w| w.as_ptr()).collect();
    argv.push(std::ptr::null());

    let pid = unsafe { libc::fork() };

    if pid < 0 {
        eprintln!("Error: fork() failed");
    } else if pid == 0 {
        let s = unsafe { libc::execvp(c_words[0].as_ptr(), argv.as_ptr()) };
        if s == -1 {
            eprintln!("exec failed: {}", std::io::Error::last_os_error());
            unsafe { libc::_exit(127) };
        }
    } else {
        let mut status: i32 = 0;
        unsafe { libc::waitpid(pid, &mut status, 0) };
        if libc::WIFEXITED(status) {
            let code = libc::WEXITSTATUS(status);
            if code != 0 {
                eprintln!("Exit status: {}", code);
            }
        } else if libc::WIFSIGNALED(status) {
            eprintln!("[Killed by signal {}]", libc::WTERMSIG(status));
        }
    }
}
