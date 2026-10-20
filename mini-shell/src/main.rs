use std::io::{self, Write};
mod builtins;
mod command;
mod pipeline;
mod redirect;
fn main() {
    unsafe {
        libc::signal(libc::SIGINT, libc::SIG_IGN);
    }
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
        if builtins::handle(&words) {
            continue;
        }
        let cmds: Vec<&[&str]> = words.split(|w| *w == "|").collect();
        if cmds.len() > 1 {
            if cmds.iter().any(|c| c.is_empty()) {
                eprintln!("mysh: syntax error near |");
                continue;
            }
            pipeline::run_pipeline(&cmds);
        } else {
            command::run_command(&words);
        }
    }
}
