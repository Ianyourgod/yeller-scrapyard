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
        let final_line = if self.line < input.lines().count() {
            let line = input.lines().nth(self.line - 1).unwrap();
            let line_number = format!("{} | ", self.line);
            format!("{}{}", line_number, line)
        } else {
            format!("{} | LALALALALA I CAN'T HEAR YOU", self.line)
        };
        return self.kind.report(&final_line);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    UnexpectedToken {
        expected: String,
        found: String,
    },
    UnexpectedEOF,
    WrongfunctionCount {
        expected: u64,
        found: u64,
    },
    LonelyVariable,
    VariableAlreadyDeclared(String),
    PackedFunc(u32),
    UnexpectedChar(char),
    VariableNotDeclared(String),
    RandomChance,
    Tabbing,
    ExtraLine,
    ShortVarName(String),
    LongFuncName(String),
    InvalidAssignmentTarget,
    TypeError,
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
            Self::WrongfunctionCount { expected, found } => {
                let expected = *expected;
                let found = *found;

                format!("Do you want your code to be unreadable? Why did you mark your function as number {} instead of {}?", found, expected)
            }
            Self::UnexpectedChar(c) => {
                let c = *c;

                format!("What the hell is a '{}', why would you type that. You must hate everyone huh.", c)
            }
            Self::LonelyVariable => {
                "Wow, you must really hate your program. Leaving your variables so lonely? Be better! Don't let a variable be alone or else you're a bad parent/programmer.".to_string()
            }
            Self::PackedFunc(amount) => {
                format!("Woah woah woah. Slow your horses buddy! You can't have {} whole variables in just one function. That's basically a party! Calm it down a bit!", amount)
            }
            Self::VariableAlreadyDeclared(name) => {
                format!("Hey man, I heard you like variables, but wow! You can't have two whole variables in your program named {}. Someone might get confused!", name)
            }
            Self::VariableNotDeclared(name) => {
                format!("Buddy... this is meant to be a bad compiler... how are you making these kinds of mistakes... {} doesn't exist buddy...", name)
            }
            Self::RandomChance => {
                "I, as the compiler, have decided that I hate you and your code. I shall now fail. Goodbye.".to_string()
            }
            Self::ExtraLine => {
                "Why is one of your lines empty? Are you trying to waste space? Do better!".to_string()
            }
            Self::Tabbing => {
                "Why do you have spacing before your line? Stop wasting peoples time by making them have to move their eyes to the start of the actually useful stuff!".to_string()
            }
            Self::ShortVarName(name) => {
                format!("My sir, thou shalt not name a vaariii-able with less than 7 characters! It is quite simply vulgur and unpleasant to the eyes! Please, m'lord, pick a better name for thy variable than {}!", name)
            }
            Self::LongFuncName(name) => {
                format!("\"{}\" br u nt skspr 💔", name)
            }
            Self::InvalidAssignmentTarget => {
                "Bro WHAT are you trying to assign to 💔".to_string()
            }
            Self::TypeError => {
                "Dude how did you manage to get a fucking type error in this bullshit language".to_string()
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

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::compile;

    fn test_error(file: &str, expected_error: ErrorKind) {
        let input = std::fs::read_to_string(file).expect("Failed to read input file");
        match compile(&input, "____doesnt______mattttter____") {
            Ok(_) => panic!("Compilation should have failed!"),
            Err(e) => {
                if let ErrorKind::RandomChance = e.kind {
                    return;
                }
                assert_eq!(e.kind, expected_error);
            },
        }
    }

    #[test]
    fn test_extra_line() {
        test_error("error_examples/extra_line.yl", ErrorKind::ExtraLine);
    }

    #[test]
    fn test_lonely_variable() {
        test_error("error_examples/lonely_variable.yl", ErrorKind::LonelyVariable);
    }

    #[test]
    fn test_long_func_name() {
        test_error("error_examples/long_func_name.yl", ErrorKind::LongFuncName("fiver".to_string()));
    }

    #[test]
    fn test_packed_func() {
        test_error("error_examples/packed_function.yl", ErrorKind::PackedFunc(11));
    }

    #[test]
    fn test_short_var_name() {
        test_error("error_examples/short_var_name.yl", ErrorKind::ShortVarName("var".to_string()));
    }

    #[test]
    fn test_tabbing() {
        test_error("error_examples/tabbing.yl", ErrorKind::Tabbing);
    }

    #[test]
    fn test_unexpected_char() {
        test_error("error_examples/unexpected_char.yl", ErrorKind::UnexpectedChar('@'));
    }

    #[test]
    fn test_unexpected_eof() {
        test_error("error_examples/unexpected_eof.yl", ErrorKind::UnexpectedEOF);
    }

    #[test]
    fn test_unexpected_token() {
        test_error("error_examples/unexpected_token.yl", ErrorKind::UnexpectedToken {
            expected: "the".to_string(),
            found: "i".to_string(),
        });
    }

    #[test]
    fn test_variable_already_declared() {
        test_error("error_examples/var_already_declared.yl", ErrorKind::VariableAlreadyDeclared("varrrriable".to_string()));
    }

    #[test]
    fn test_variable_not_declared() {
        test_error("error_examples/var_not_declared.yl", ErrorKind::VariableNotDeclared("varrrriable".to_string()));
    }

    #[test]
    fn test_wrong_function_count() {
        test_error("error_examples/wrong_fn_count.yl", ErrorKind::WrongfunctionCount {
            expected: 1,
            found: 2,
        });
    }

    #[test]
    fn test_invalid_assignment_target() {
        test_error("error_examples/invalid_assign_target.yl", ErrorKind::InvalidAssignmentTarget);
    }
}