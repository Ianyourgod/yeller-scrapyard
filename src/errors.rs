#![allow(dead_code)]

use std::io::Write;

#[derive(Debug, Clone)]
pub struct Error {
    pub kind: ErrorKind,
    pub line: usize,
}

impl Error {
    pub fn new(kind: ErrorKind, line: usize) -> Self {
        Self { kind, line }
    }

    pub fn report(&self, input: &str) {
        let line = input.lines().nth(self.line - 1).unwrap();
        let line_number = format!("{} | ", self.line);
        let final_line = format!("{}{}", line_number, line);
        return self.kind.report(&final_line);
    }
}

#[derive(Debug, Clone)]
pub enum ErrorKind {
    UnexpectedToken {
        expected: String,
        found: String,
    },
    UnexpectedEOF,
    WrongFunshunCount {
        expected: u32,
        found: u32,
    },
    UnexpectedChar(char),
}

impl ErrorKind {
    pub fn report(&self, line: &str) {
        let text = self.to_speech();
        eprintln!("Error: {}", text);
        eprintln!("{}", line);
    }

    pub fn to_speech(&self) -> String {
        let text = match self {
            Self::UnexpectedToken { expected, found } => {
                format!("You dumbass, you wrote {}, when I wanted {}", found, expected)
            }
            Self::UnexpectedEOF => {
                "Why the hell is there an EOF here".to_string()
            }
            Self::WrongFunshunCount { expected, found } => {
                let expected = *expected;
                let found = *found;

                format!("Do you want your code to be unreadable? Why did you mark your function as number {} instead of {}?", found, expected)
            }
            Self::UnexpectedChar(c) => {
                let c = *c;

                format!("What the hell is a '{}', why would you type that. You must hate everyone huh.", c)
            }
        };

        // call "python3 speech.py" with the error message

        std::process::Command::new("python3")
            .arg("speech.py")
            .stdin(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to run speech.py")
            .stdin
            .unwrap()
            .write_all(text.as_bytes())
            .expect("Failed to write to stdin of speech.py");

        text
    }
}