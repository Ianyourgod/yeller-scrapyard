mod lexer;
mod parser;
mod ir;
mod llvm_gen;

mod errors;

fn compile(input: &str, output_file: &str) -> Result<(), errors::Error> {
    let mut parser = parser::Parser::new(input)?;
    let program = parser.parse_program()?;
    let mut ir_generator = ir::IRGenerator::new();
    let program = ir_generator.generate_ir(program)?;
    let context = llvm_gen::LLVMGenerator::create_context();
    let llvm_gen = llvm_gen::LLVMGenerator::new(&context);
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
