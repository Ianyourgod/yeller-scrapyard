pub mod definition;

use crate::parser::nodes;
use crate::errors;
use crate::semantic_analysis::typecheck::{STEntry, SymbolTable};

pub struct IRGenerator {
    tmp_counter: u64,
    pub symbol_table: SymbolTable,
}

impl IRGenerator {
    pub fn new(symbol_table: SymbolTable) -> Self {
        Self { tmp_counter: 0, symbol_table }
    }

    pub fn generate_ir(&mut self, program: nodes::Program) -> Result<definition::Program, errors::Error> {
        let mut functions = Vec::new();
        
        for function in program.functions {
            let function = self.generate_function(function)?;
            if let Some(function) = function {
                functions.push(function);
            }
        }

        Ok(definition::Program { functions })
    }

    fn generate_function(&mut self, function: nodes::FunctionDefinition) -> Result<Option<definition::Function>, errors::Error> {
        let mut body = Vec::new();

        let block = if let Some(body) = function.body {
            body
        } else {
            return Ok(None);
        };

        self.generate_block(block, &mut body)?;

        Ok(Some(definition::Function {
            name: function.name,
            params: function.params,
            return_type: function.return_type,
            body,
        }))
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

                body.push(definition::Instruction::JumpIfZero(val, label.clone()));
                self.generate_statement(*block, body)?;
                body.push(definition::Instruction::Label(label));

                if let Some(block) = else_block {
                    let end_label = self.new_tmp();
                    body.push(definition::Instruction::Jump(end_label.clone()));
                    self.generate_statement(*block, body)?;
                    body.push(definition::Instruction::Label(end_label));
                }
            }
            nodes::StatementKind::While(val, block) => {
                let label = self.new_tmp();
                let end_label = self.new_tmp();

                body.push(definition::Instruction::Label(label.clone()));
                let val = self.generate_expression(val, body)?;
                body.push(definition::Instruction::JumpIfZero(val, end_label.clone()));
                self.generate_statement(*block, body)?;
                body.push(definition::Instruction::Jump(label));
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

                let dst = self.new_tmp_var(expression.ty.clone());
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
            nodes::ExpressionKind::Assign(left, right) => {
                let right = self.generate_expression(*right, body)?;
                let left = match left.kind {
                    nodes::ExpressionKind::Variable(name) => {
                        let var = definition::Val::Var(name);
                        body.push(definition::Instruction::Copy {
                            src: right,
                            dst: var.clone(),
                        });
                        var
                    }
                    nodes::ExpressionKind::Dereference(expr) => {
                        let addr = self.generate_expression(*expr, body)?;
                        body.push(definition::Instruction::Store(right, addr.clone()));
                        addr
                    }
                    nodes::ExpressionKind::Subscript(expr, index) => {
                        let ty = match &expr.ty {
                            nodes::Type::Pointer(ty) => (**ty).clone(),
                            _ => unreachable!(),
                        };
                        let expr = self.generate_expression(*expr, body)?;
                        let index = self.generate_expression(*index, body)?;

                        let addr = self.new_tmp_var(nodes::Type::Pointer(Box::new(ty)));
                        body.push(definition::Instruction::AddPtr {
                            ptr: expr,
                            index,
                            dst: addr.clone(),
                        });
                        body.push(definition::Instruction::Store(right, addr.clone()));
                        addr
                    }
                    _ => unreachable!(),
                };

                Ok(left)
            }
            nodes::ExpressionKind::IsZero(expr) => {
                let val = self.generate_expression(*expr, body)?;
                let dst = self.new_tmp_var(expression.ty.clone());

                body.push(definition::Instruction::Binary {
                    op: definition::Binop::Equal,
                    src1: val,
                    src2: definition::Val::Number(0),
                    dst: dst.clone(),
                });

                Ok(dst)
            }
            nodes::ExpressionKind::FunctionCall(name, args) => {
                let args = args.into_iter().map(|arg| self.generate_expression(arg, body)).collect::<Result<Vec<_>, _>>()?;
                let dst = self.new_tmp_var(expression.ty.clone());

                body.push(definition::Instruction::FunctionCall(name, args, dst.clone()));

                Ok(dst)
            }
            nodes::ExpressionKind::AddressOf(expr) => {
                let val = self.generate_expression(*expr, body)?;
                let dst = self.new_tmp_var(expression.ty.clone());

                body.push(definition::Instruction::GetAddress(val, dst.clone()));

                Ok(dst)
            }
            nodes::ExpressionKind::Dereference(expr) => {
                let val = self.generate_expression(*expr, body)?;
                let dst = self.new_tmp_var(expression.ty.clone());

                body.push(definition::Instruction::Load(val, dst.clone()));

                Ok(dst)
            }
            nodes::ExpressionKind::Subscript(expr, index) => {
                let expr = self.generate_expression(*expr, body)?;
                let index = self.generate_expression(*index, body)?;
                let dst = self.new_tmp_var(expression.ty.clone());

                body.push(definition::Instruction::AddPtr {
                    ptr: expr,
                    index,
                    dst: dst.clone(),
                });

                Ok(dst)
            }
            nodes::ExpressionKind::Variable(name) => Ok(definition::Val::Var(name)),
        }
    }

    fn new_tmp_var(&mut self, ty: nodes::Type) -> definition::Val {
        let name = self.new_tmp();
        self.symbol_table.insert(name.clone(), STEntry { ty });
        definition::Val::Var(name)
    }

    fn new_tmp(&mut self) -> String {
        let name = format!(".tmp.{}", self.tmp_counter);
        self.tmp_counter += 1;
        name
    }
}