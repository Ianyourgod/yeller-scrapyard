use crate::parser::nodes;
use crate::errors;

mod variable_resolution;
pub mod typecheck;

pub fn analyze(program: nodes::Program) -> Result<(nodes::Program, typecheck::SymbolTable), errors::Error> {
    let mut analyzer = variable_resolution::Analyzer::new();
    let program = analyzer.analyze_program(program)?;
    let mut typechecker = typecheck::TypeChecker::new();
    let program = typechecker.typecheck_program(program)?;

    Ok((program, typechecker.symbol_table))
}