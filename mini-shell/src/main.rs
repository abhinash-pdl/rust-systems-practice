use std::ffi::CString;
use std::io::{self, Write};

fn main() {
    loop {
        print!("mysh> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        let n = io::stdin().read_line(&mut line).unwrap();
        if n == 0 {
            break;
        }

        let words: Vec<&str> = line.split_whitespace().collect();

        if words.is_empty() {
            continue;
        }
        if words[0] =="cd"{
            let target = match words.get(1){
                Some(dir) => dir.to_string(),
                None => std::env::var("HOME").unwrap_or_else(|_| "/".to_string())
            };
            let c_target = CString::new(target).unwrap();
            let status = unsafe {libc::chdir(c_target.as_ptr())};
            if status == -1{
                eprintln!("cd : {}",io::Error::last_os_error());
            }
            continue;
        }
        if words[0] == "exit"{
            let code = words.get(1).and_then(|c| c.parse::<i32>().ok()).unwrap_or(0);
            std::process::exit(code);
        }

        run_command(&words);
    }
}

fn run_command(words: &[&str]) {
    let c_words: Vec<CString> = words.iter().map(|w| CString::new(*w).unwrap()).collect();
    let mut argv: Vec<*const libc::c_char> = c_words.iter().map(|w| w.as_ptr()).collect();
    argv.push(std::ptr::null());

    let pid = unsafe { libc::fork() };

    if pid < 0 {
        eprintln!("Error: fork() failed");
    } else if pid == 0 {
        let s = unsafe { libc::execvp(c_words[0].as_ptr(), argv.as_ptr()) };
        if s == -1 {
            eprintln!("exec failed: {}", io::Error::last_os_error());
            unsafe { libc::_exit(127) };
        }
    } else {
        let mut status: i32 = 0;
        unsafe { libc::waitpid(pid, &mut status, 0) };
        if libc::WIFEXITED(status) {
            let code = libc::WEXITSTATUS(status);
            if code !=0 {
                eprintln!("Exit status: {}", code);
            }
        }else if libc::WIFSIGNALED(status) {
            eprintln!("[Killed by signal {}]", libc::WTERMSIG(status));
        }
    }

}
