use std::collections::HashSet;

use crate::ir::definition;
use crate::semantic_analysis::typecheck::SymbolTable;

pub struct Collector<'a> {
    pub variables: HashSet<(String, definition::Type)>,
    pub frontend_symbol_table: &'a SymbolTable,
}

impl<'a> Collector<'a> {
    pub fn new(frontend_symbol_table: &'a SymbolTable) -> Self {
        Self { variables: HashSet::new(), frontend_symbol_table }
    }

    pub fn collect_function(&mut self, function: &definition::Function) {
        for instruction in &function.body {
            self.collect_instruction(instruction);
        }
    }

    fn collect_instruction(&mut self, instruction: &definition::Instruction) {
        match instruction {
            definition::Instruction::Return(val) => {
                self.collect_val(val);
            }
            definition::Instruction::Binary { src1, src2, dst, .. } => {
                self.collect_val(src1);
                self.collect_val(src2);
                self.collect_val(dst);
            }
            definition::Instruction::Copy { src, dst } => {
                self.collect_val(src);
                self.collect_val(dst);
            }
            definition::Instruction::JumpIfZero(val, ..) => {
                self.collect_val(val);
            }
            definition::Instruction::JumpIfNotZero(val, ..) => {
                self.collect_val(val);
            }
            definition::Instruction::FunctionCall(_, args, dst) => {
                for arg in args {
                    self.collect_val(arg);
                }
                self.collect_val(dst);
            }
            definition::Instruction::GetAddress(_, val) => {
                self.collect_val(val);
            }
            definition::Instruction::Store(val1, val2) => {
                self.collect_val(val1);
                self.collect_val(val2);
            }
            definition::Instruction::Load(val1, val2) => {
                self.collect_val(val1);
                self.collect_val(val2);
            }
            definition::Instruction::AddPtr { ptr, index, dst, .. } => {
                self.collect_val(ptr);
                self.collect_val(index);
                self.collect_val(dst);
            }
            definition::Instruction::Jump(_) |
            definition::Instruction::Label(_) => {}
        }
    }

    fn collect_val(&mut self, val: &definition::Val) {
        match val {
            definition::Val::Var(name) => {
                let ty = self.frontend_symbol_table.get(name).unwrap().ty.clone();

                self.variables.insert((name.clone(), ty));
            }
            _ => {}
        }
    }
}