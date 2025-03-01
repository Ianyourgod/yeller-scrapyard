pub mod definition;

use crate::parser::nodes;
use crate::errors;

pub struct IRGenerator {
    tmp_counter: u64,
}

impl IRGenerator {
    pub fn new() -> Self {
        Self { tmp_counter: 0 }
    }

    pub fn generate_ir(&mut self, program: nodes::Program) -> Result<definition::Program, errors::Error> {
        let mut functions = Vec::new();
        
        for function in program.functions {
            functions.push(self.generate_function(function)?);
        }

        Ok(definition::Program { functions })
    }

    fn generate_function(&mut self, function: nodes::FunctionDefinition) -> Result<definition::Function, errors::Error> {
        let mut body = Vec::new();

        self.generate_block(function.body, &mut body)?;

        Ok(definition::Function {
            name: function.name,
            params: function.params,
            return_type: function.return_type,
            body,
        })
    }

    fn generate_block(&mut self, block: nodes::Block, body: &mut Vec<definition::Instruction>) -> Result<(), errors::Error> {
        for item in block.items {
            match item {
                nodes::BlockItem::Statement(statement) => {
                    self.generate_statement(statement, body)?;
                }
                nodes::BlockItem::Declaration(declaration) => {
                    self.generate_declaration(declaration, body)?;
                }
            }
        }

        Ok(())
    }

    fn generate_statement(&mut self, statement: nodes::Statement, body: &mut Vec<definition::Instruction>) -> Result<(), errors::Error> {
        match statement.kind {
            nodes::StatementKind::Return(expression) => {
                let val = self.generate_expression(expression, body)?;
                body.push(definition::Instruction::Return(val));
            }
            nodes::StatementKind::Block(block) => {
                self.generate_block(block, body)?;
            }
            nodes::StatementKind::Expression(expression) => {
                self.generate_expression(expression, body)?;
            }
            nodes::StatementKind::If(val, block, else_block) => {
                let val = self.generate_expression(val, body)?;
                let label = self.new_tmp();
                let end_label = self.new_tmp();

                body.push(definition::Instruction::JumpIfZero(val, label.clone()));
                self.generate_statement(*block, body)?;
                body.push(definition::Instruction::Jump(end_label.clone()));
                body.push(definition::Instruction::Label(label));
                if let Some(else_block) = else_block {
                    self.generate_statement(*else_block, body)?;
                }
                body.push(definition::Instruction::Label(end_label));
            }
        }

        Ok(())
    }

    fn generate_declaration(&mut self, declaration: nodes::Declaration, body: &mut Vec<definition::Instruction>) -> Result<(), errors::Error> {
        let val = self.generate_expression(declaration.value, body)?;
        body.push(definition::Instruction::Copy {
            src: val,
            dst: definition::Val::Var(declaration.name),
        });

        Ok(())
    }

    fn generate_expression(&mut self, expression: nodes::Expression, body: &mut Vec<definition::Instruction>) -> Result<definition::Val, errors::Error> {
        match expression.kind {
            nodes::ExpressionKind::Number(n) => Ok(definition::Val::Number(n)),
            nodes::ExpressionKind::Binary(op, left, right) => {
                let left = self.generate_expression(*left, body)?;
                let right = self.generate_expression(*right, body)?;

                let dst = definition::Val::Var(self.new_tmp());
                let kind = match op {
                    nodes::Binop::Add => definition::Binop::Add,
                    nodes::Binop::Sub => definition::Binop::Sub,
                    nodes::Binop::Mul => definition::Binop::Mul,
                    nodes::Binop::Div => definition::Binop::Div,
                    nodes::Binop::Mod => definition::Binop::Mod,
                };

                let instr = definition::Instruction::Binary {
                    op: kind,
                    src1: left,
                    src2: right,
                    dst: dst.clone(),
                };

                body.push(instr);

                Ok(dst)
            }
            nodes::ExpressionKind::Variable(name) => Ok(definition::Val::Var(name)),
        }
    }

    fn new_tmp(&mut self) -> String {
        let name = format!(".tmp.{}", self.tmp_counter);
        self.tmp_counter += 1;
        name
    }
}