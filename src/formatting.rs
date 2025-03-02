use crate::errors;

pub fn formatting_check(input: &str) -> Result<(), errors::Error> { // true if successful, false if failed
    if !check_for_tabs(input) {
        return Err(errors::Error::new(errors::ErrorKind::Tabbing, usize::MAX));
    }

    if !check_for_empties(input) {
        return Err(errors::Error::new(errors::ErrorKind::ExtraLine, usize::MAX));
    }

    Ok(())
}

fn check_for_tabs(input: &str) -> bool {
    !input.lines().any(|line| if let Some(c) = line.chars().nth(0) { c.is_whitespace() } else { false })
}

fn check_for_empties(input: &str) -> bool {
    !input.lines().any(|line| line.is_empty())
}