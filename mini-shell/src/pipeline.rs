use crate::redirect::{redirect_stdin, redirect_stdout, split_input, split_redirect};
use std::ffi::CString;
fn exec_words(words: &[&str]) -> ! {
    let c_words: Vec<CString> = words.iter().map(|w| CString::new(*w).unwrap()).collect();
    let mut argv: Vec<*const libc::c_char> = c_words.iter().map(|w| w.as_ptr()).collect();
    argv.push(std::ptr::null());
    unsafe { libc::execvp(c_words[0].as_ptr(), argv.as_ptr()) };
    eprintln!("exec failed: {}", std::io::Error::last_os_error());
    unsafe { libc::_exit(127) };
}

pub fn run_pipeline(cmds: &[&[&str]]) {
    let n = cmds.len();
    let mut cmds: Vec<&[&str]> = cmds.to_vec();

    let (first, infile) = match split_input(cmds[0]) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("mysh: {}", e);
            return;
        }
    };
    cmds[0] = first;
    let (last, outfile, append) = match split_redirect(cmds[n - 1]) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("mysh: {}", e);
            return;
        }
    };
    cmds[n - 1] = last;
    let mut prev_read: i32 = -1;
    let mut pids: Vec<i32> = Vec::new();

    for i in 0..n {
        let is_last = i == n - 1;
        let mut fds = [-1i32; 2];
        if !is_last && unsafe { libc::pipe(fds.as_mut_ptr()) } == -1 {
            eprintln!("pipe failed: {}", std::io::Error::last_os_error());
            break;
        }
        let pid = unsafe { libc::fork() };
        if pid == 0 {
            if prev_read != -1 {
                unsafe {
                    libc::dup2(prev_read, 0);
                    libc::close(prev_read);
                }
            }
            if !is_last {
                unsafe {
                    libc::close(fds[0]);
                    libc::dup2(fds[1], 1);
                    libc::close(fds[1]);
                }
            }
            if i == 0
                && let Some(name) = infile
            {
                redirect_stdin(name);
            }
            if is_last && let Some(name) = outfile {
                redirect_stdout(name, append);
            }
            exec_words(cmds[i]);
        }
        if prev_read != -1 {
            unsafe {
                libc::close(prev_read);
            }
        }
        prev_read = -1;
        if !is_last {
            unsafe {
                libc::close(fds[1]);
            }
            prev_read = fds[0];
        }
        pids.push(pid);
    }
    if prev_read != -1 {
        unsafe {
            libc::close(prev_read);
        }
    }
    for pid in pids {
        let mut status = 0;
        unsafe {
            libc::waitpid(pid, &mut status, 0);
        }
    }
}
