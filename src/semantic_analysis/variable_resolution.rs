use std::collections::HashMap;
use crate::parser::nodes;
use crate::errors;

#[derive(Debug, Clone)]
pub struct VarMapEntry {
    #[allow(dead_code)]
    pub ty: nodes::Type,
}

pub struct Analyzer {
    pub var_map: HashMap<String, VarMapEntry>,
    pub variables_this_function: u32,
}

impl Analyzer {
    pub fn new() -> Self {
        Self {
            var_map: HashMap::new(),
            variables_this_function: 0,
        }
    }

    pub fn analyze_program(&mut self, mut program: nodes::Program) -> Result<nodes::Program, errors::Error> {
        program.functions.iter().map(|function| {
            self.preanalyze_function(function)
        }).collect::<Result<Vec<_>, _>>()?;
        
        let new_functions = program.functions.into_iter().map(|function| self.analyze_function(function)).collect::<Result<Vec<_>, _>>()?;

        program.functions = new_functions;

        Ok(program)
    }

    fn preanalyze_function(&mut self, function: &nodes::FunctionDefinition) -> Result<(), errors::Error> {
        self.var_map.insert(function.name.clone(), VarMapEntry { ty: nodes::Type::Function(function.params.iter().map(|(_, ty)| ty.clone()).collect(), Box::new(function.return_type.clone())) });

        if function.name.len() > 4 {
            return Err(errors::Error::new(errors::ErrorKind::LongFuncName(function.name.clone()), function.line_started));
        }

        Ok(())
    }

    fn analyze_function(&mut self, function: nodes::FunctionDefinition) -> Result<nodes::FunctionDefinition, errors::Error> {
        self.variables_this_function = 0;

        for (name, ty) in &function.params {
            self.var_map.insert(name.clone(), VarMapEntry { ty: ty.clone() });
            self.variables_this_function += 1;
        }

        let new_block = self.analyze_block(function.body)?;

        if self.variables_this_function == 1 {
            return Err(errors::Error::new(errors::ErrorKind::LonelyVariable, function.line_started));
        }

        if self.variables_this_function >= 10 {
            return Err(errors::Error::new(errors::ErrorKind::PackedFunc(self.variables_this_function), function.line_started));
        }

        let function = nodes::FunctionDefinition {
            name: function.name,
            params: function.params,
            return_type: function.return_type,
            body: new_block,
            line_started: function.line_started
        };

        Ok(function)
    }

    fn analyze_block(&mut self, block: nodes::Block) -> Result<nodes::Block, errors::Error> {
        let mut new_items = Vec::new();

        for item in block.items {
            match item {
                nodes::BlockItem::Statement(statement) => {
                    new_items.push(nodes::BlockItem::Statement(self.analyze_statement(statement)?));
                }
                nodes::BlockItem::Declaration(declaration) => {
                    new_items.push(nodes::BlockItem::Declaration(self.analyze_declaration(declaration)?));
                }
            }
        }

        Ok(nodes::Block { items: new_items, line_started: block.line_started })
    }

    fn analyze_declaration(&mut self, declaration: nodes::Declaration) -> Result<nodes::Declaration, errors::Error> {
        if self.var_map.contains_key(&declaration.name) {
            return Err(errors::Error::new(errors::ErrorKind::VariableAlreadyDeclared(declaration.name), declaration.line_started));
        }

        if declaration.name.len() < 7 {
            return Err(errors::Error::new(errors::ErrorKind::ShortVarName(declaration.name), declaration.line_started));
        }

        // analyze the expression
        let new_expression = self.analyze_expression(declaration.value)?;

        self.var_map.insert(declaration.name.clone(), VarMapEntry { ty: declaration.ty.clone() });

        self.variables_this_function += 1;

        Ok(nodes::Declaration {
            name: declaration.name,
            ty: declaration.ty,
            value: new_expression,
            line_started: declaration.line_started,
        })
    }

    fn analyze_statement(&mut self, statement: nodes::Statement) -> Result<nodes::Statement, errors::Error> {
        match statement.kind {
            nodes::StatementKind::Return(expression) => {
                let new_expression = self.analyze_expression(expression)?;

                Ok(nodes::Statement {
                    kind: nodes::StatementKind::Return(new_expression),
                    line_started: statement.line_started,
                })
            }
            nodes::StatementKind::Block(block) => {
                let new_block = self.analyze_block(block)?;

                Ok(nodes::Statement {
                    kind: nodes::StatementKind::Block(new_block),
                    line_started: statement.line_started,
                })
            }
            nodes::StatementKind::Expression(expression) => {
                let new_expression = self.analyze_expression(expression)?;

                Ok(nodes::Statement {
                    kind: nodes::StatementKind::Expression(new_expression),
                    line_started: statement.line_started,
                })
            }
            nodes::StatementKind::If(val, block, else_block) => {
                let new_val = self.analyze_expression(val)?;
                let new_block = self.analyze_statement(*block)?;
                let new_else_block = else_block.map(|block| self.analyze_statement(*block)).transpose()?;

                Ok(nodes::Statement {
                    kind: nodes::StatementKind::If(new_val, Box::new(new_block), new_else_block.map(Box::new)),
                    line_started: statement.line_started,
                })
            }
            nodes::StatementKind::While(val, block) => {
                let new_val = self.analyze_expression(val)?;
                let new_block = self.analyze_statement(*block)?;

                Ok(nodes::Statement {
                    kind: nodes::StatementKind::While(new_val, Box::new(new_block)),
                    line_started: statement.line_started,
                })
            }
        }
    }

    fn analyze_expression(&mut self, expression: nodes::Expression) -> Result<nodes::Expression, errors::Error> {
        match expression.kind {
            nodes::ExpressionKind::Number(_) => Ok(expression),
            nodes::ExpressionKind::Binary(op, left, right) => {
                let new_left = self.analyze_expression(*left)?;
                let new_right = self.analyze_expression(*right)?;

                Ok(nodes::Expression {
                    kind: nodes::ExpressionKind::Binary(op, Box::new(new_left), Box::new(new_right)),
                    line_started: expression.line_started,
                    ty: expression.ty,
                })
            }
            nodes::ExpressionKind::Variable(name) => {
                if !self.var_map.contains_key(&name) {
                    return Err(errors::Error::new(errors::ErrorKind::VariableNotDeclared(name), expression.line_started));
                }

                Ok(nodes::Expression {
                    kind: nodes::ExpressionKind::Variable(name),
                    line_started: expression.line_started,
                    ty: expression.ty,
                })
            }
            nodes::ExpressionKind::Assign(left, right) => {
                let new_left = self.analyze_expression(*left)?;
                let new_right = self.analyze_expression(*right)?;

                match new_left.kind {
                    nodes::ExpressionKind::Variable(ref name) => {
                        if !self.var_map.contains_key(name) {
                            return Err(errors::Error::new(errors::ErrorKind::VariableNotDeclared(name.clone()), expression.line_started));
                        }
                    }
                    _ => return Err(errors::Error::new(errors::ErrorKind::InvalidAssignmentTarget, expression.line_started)),
                }

                Ok(nodes::Expression {
                    kind: nodes::ExpressionKind::Assign(Box::new(new_left), Box::new(new_right)),
                    line_started: expression.line_started,
                    ty: expression.ty,
                })
            }
            nodes::ExpressionKind::IsZero(expr) => {
                let new_expr = self.analyze_expression(*expr)?;

                Ok(nodes::Expression {
                    kind: nodes::ExpressionKind::IsZero(Box::new(new_expr)),
                    line_started: expression.line_started,
                    ty: expression.ty,
                })
            }
            nodes::ExpressionKind::FunctionCall(name, args) => {
                if !self.var_map.contains_key(&name) {
                    return Err(errors::Error::new(errors::ErrorKind::VariableNotDeclared(name), expression.line_started));
                }

                let new_args = args.into_iter().map(|arg| self.analyze_expression(arg)).collect::<Result<Vec<_>, _>>()?;

                Ok(nodes::Expression {
                    kind: nodes::ExpressionKind::FunctionCall(name, new_args),
                    line_started: expression.line_started,
                    ty: expression.ty,
                })
            }
        }
    }
}