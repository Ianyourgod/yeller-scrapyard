#![feature(box_patterns)]

use rand::Rng;

mod formatting;
mod lexer;
mod parser;
mod semantic_analysis;
mod ir;
mod llvm_gen;

mod errors;

fn compile(input: &str, output_file: &str) -> Result<(), errors::Error> {
    // 1/5 chance to fail
    if rand::rng().random_range(0..5) == 0 {
        return Err(errors::Error::new(errors::ErrorKind::RandomChance, usize::MAX));
    }

    formatting::formatting_check(input)?;

    let mut parser = parser::Parser::new(input)?;
    let program = parser.parse_program()?;

    let (program, symbol_table) = semantic_analysis::analyze(program)?;

    //println!("{:#?}", program);

    let mut ir_generator = ir::IRGenerator::new(symbol_table);
    let program = ir_generator.generate_ir(program)?;

    //println!("{:#?}", program);

    let context = llvm_gen::LLVMGenerator::create_context();
    let llvm_gen = llvm_gen::LLVMGenerator::new(&context, &ir_generator.symbol_table);
    llvm_gen.generate(program, output_file);

    Ok(())
}

fn main() {
    // read args
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input> <output>", args[0]);
        std::process::exit(1);
    }

    let input = std::fs::read_to_string(&args[1]).expect("Failed to read input file");
    match compile(&input, &args[2]) {
        Ok(_) => println!("Compilation successful"),
        Err(e) => e.report(&input),
    }
}
