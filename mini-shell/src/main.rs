use std::io::{self, Write};
mod builtins;
mod command;
mod pipeline;
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
        if builtins::handle(&words) {
            continue;
        }
        if let Some(pos) = words.iter().position(|w| *w == "|") {
            let (left, right) = (&words[..pos], &words[pos + 1..]);

            if left.is_empty() || right.is_empty() {
                eprintln!("mysh: syntax error near | ");
                continue;
            }
            pipeline::run_pipeline(left, right);
        } else {
            command::run_command(&words);
        }
    }
}
