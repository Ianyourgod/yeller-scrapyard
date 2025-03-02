use std::collections::HashSet;

use crate::ir::definition;

pub struct Collector {
    pub variables: HashSet<(String, definition::Type)>,
}

impl Collector {
    pub fn new() -> Self {
        Self { variables: HashSet::new() }
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
            definition::Instruction::Jump(_) |
            definition::Instruction::Label(_) => {}
        }
    }

    fn collect_val(&mut self, val: &definition::Val) {
        match val {
            definition::Val::Var(name) => {
                let ty = definition::Type::I32;

                self.variables.insert((name.clone(), ty));
            }
            _ => {}
        }
    }
}