use std::ffi::CString;
pub fn handle(words: &[&str]) -> bool {
    if words[0] == "cd" {
        let target = match words.get(1) {
            Some(dir) => dir.to_string(),
            None => std::env::var("HOME").unwrap_or_else(|_| "/".to_string()),
        };
        let c_target = CString::new(target).unwrap();
        let status = unsafe { libc::chdir(c_target.as_ptr()) };
        if status == -1 {
            eprintln!("cd: {}", std::io::Error::last_os_error());
        }
        return true;
    }
    if words[0] == "exit" {
        let code = words
            .get(1)
            .and_then(|c| c.parse::<i32>().ok())
            .unwrap_or(0);
        std::process::exit(code);
    }
    false
}
