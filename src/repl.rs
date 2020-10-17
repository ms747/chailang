use std::io::{Read, Result, Stdin, Stdout, Write};

use crate::lexer::Lexer;

const PROMPT: &str = ">> ";

pub struct Repl {
    stdin: Stdin,
    stdout: Stdout,
}

impl Repl {
    pub fn new(stdin: Stdin, stdout: Stdout) -> Self {
        Repl { stdin, stdout }
    }

    pub fn start(&mut self) -> std::io::Result<()> {
        let mut stdout_handle = self.stdout.lock();
        let mut buffer = String::new();
        let mut lexer = Lexer::new("".into());
        loop {
            stdout_handle.write(PROMPT.as_bytes())?;
            stdout_handle.flush()?;
            self.stdin.read_line(&mut buffer)?;
            lexer.initial_state(&buffer);
            stdout_handle.write_fmt(format_args!("{:#?}\n", lexer.tokens()))?;
            buffer.clear();
        }
        Ok(())
    }
}
