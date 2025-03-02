use crate::parser::nodes;
use crate::errors;

mod variable_resolution;

pub fn analyze(program: nodes::Program) -> Result<nodes::Program, errors::Error> {
    let mut analyzer = variable_resolution::Analyzer::new();
    analyzer.analyze_program(program)
}