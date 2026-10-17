use std::ffi::CString;

fn main() {
    let mut fds = [0i32; 2];
    if unsafe { libc::pipe(fds.as_mut_ptr()) } == -1 {
        eprintln!("pipe failed {}", std::io::Error::last_os_error());
        return;
    }
    let read_end = fds[0];
    let write_end = fds[1];

    let pid1 = unsafe { libc::fork() };

    if pid1 == 0 {
        unsafe {
            libc::close(read_end);
            libc::dup2(write_end, 1);
            libc::close(write_end);
        }
        let prog = CString::new("ls").unwrap();
        let argv = [prog.as_ptr(), std::ptr::null()];
        unsafe {
            libc::execvp(prog.as_ptr(), argv.as_ptr());
            libc::_exit(127);
        }
    }
    let pid2 = unsafe { libc::fork() };
    if pid2 == 0 {
        unsafe {
            libc::close(write_end);
            libc::dup2(read_end, 0);
            libc::close(read_end);
        }
        let prog = CString::new("wc").unwrap();
        let arg = CString::new("-l").unwrap();
        let argv = [prog.as_ptr(), arg.as_ptr(), std::ptr::null()];
        unsafe {
            libc::execvp(prog.as_ptr(), argv.as_ptr());
            libc::_exit(127);
        }
    }
    unsafe {
        libc::close(read_end);
        libc::close(write_end);
        let mut status = 0;
        libc::waitpid(pid1, &mut status, 0);
        libc::waitpid(pid2, &mut status, 0);
    }
}
