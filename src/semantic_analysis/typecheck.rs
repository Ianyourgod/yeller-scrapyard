use std::collections::HashMap;

use crate::parser::nodes;
use crate::errors;

#[derive(Debug, Clone)]
pub struct STEntry {
    pub ty: nodes::Type,
}

pub struct SymbolTable {
    symbols: HashMap<String, STEntry>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }

    pub fn insert_raw(&mut self, name: String, ty: nodes::Type) {
        self.symbols.insert(name, STEntry { ty });
    }

    pub fn insert(&mut self, name: String, entry: STEntry) {
        self.symbols.insert(name, entry);
    }

    pub fn get(&self, name: &str) -> Option<&STEntry> {
        self.symbols.get(name)
    }
}

pub struct TypeChecker {
    pub symbol_table: SymbolTable,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            symbol_table: SymbolTable::new(),
        }
    }

    pub fn typecheck_program(&mut self, program: nodes::Program) -> Result<nodes::Program, errors::Error> {
        for function in &program.functions {
            self.preadd_functions(function)?;
        }

        let new_functions = program.functions.into_iter().map(|function| self.typecheck_function(function)).collect::<Result<Vec<_>, _>>()?;

        Ok(nodes::Program { functions: new_functions })
    }

    fn preadd_functions(&mut self, function: &nodes::FunctionDefinition) -> Result<(), errors::Error> {
        self.symbol_table.insert_raw(function.name.clone(), nodes::Type::Function(function.params.iter().map(|(_, ty)| ty.clone()).collect(), Box::new(function.return_type.clone())));

        Ok(())
    }

    fn typecheck_function(&mut self, function: nodes::FunctionDefinition) -> Result<nodes::FunctionDefinition, errors::Error> {
        for (name, ty) in &function.params {
            self.symbol_table.insert(name.clone(), STEntry { ty: ty.clone() });
        }

        let new_block = if let Some(body) = function.body { Some(self.typecheck_block(body)?) } else {None};

        Ok(nodes::FunctionDefinition {
            name: function.name,
            params: function.params,
            return_type: function.return_type,
            body: new_block,
            line_started: function.line_started
        })
    }

    fn typecheck_block(&mut self, block: nodes::Block) -> Result<nodes::Block, errors::Error> {
        let new_items = block.items.into_iter().map(|item| self.typecheck_block_item(item)).collect::<Result<Vec<_>, _>>()?;

        Ok(nodes::Block { items: new_items, line_started: block.line_started })
    }

    fn typecheck_block_item(&mut self, item: nodes::BlockItem) -> Result<nodes::BlockItem, errors::Error> {
        match item {
            nodes::BlockItem::Statement(statement) => {
                let new_statement = self.typecheck_statement(statement)?;
                Ok(nodes::BlockItem::Statement(new_statement))
            }
            nodes::BlockItem::Declaration(declaration) => {
                let new_declaration = self.typecheck_declaration(declaration)?;
                Ok(nodes::BlockItem::Declaration(new_declaration))
            }
        }
    }

    fn typecheck_declaration(&mut self, declaration: nodes::Declaration) -> Result<nodes::Declaration, errors::Error> {
        let new_value = self.typecheck_and_convert(declaration.value)?;

        // we let llvm catch our type errors because im lazy

        self.symbol_table.insert(declaration.name.clone(), STEntry { ty: declaration.ty.clone() });

        Ok(nodes::Declaration { name: declaration.name, ty: declaration.ty, value: new_value, line_started: declaration.line_started })
    }

    fn typecheck_statement(&mut self, statement: nodes::Statement) -> Result<nodes::Statement, errors::Error> {
        match statement.kind {
            nodes::StatementKind::Return(expression) => {
                let new_expression = self.typecheck_and_convert(expression)?;
                Ok(nodes::Statement { kind: nodes::StatementKind::Return(new_expression), line_started: statement.line_started })
            }
            nodes::StatementKind::Block(block) => {
                let new_block = self.typecheck_block(block)?;
                Ok(nodes::Statement { kind: nodes::StatementKind::Block(new_block), line_started: statement.line_started })
            }
            nodes::StatementKind::Expression(expression) => {
                let new_expression = self.typecheck_and_convert(expression)?;
                Ok(nodes::Statement { kind: nodes::StatementKind::Expression(new_expression), line_started: statement.line_started })
            }
            nodes::StatementKind::If(condition, then_block, else_block) => {
                let new_condition = self.typecheck_and_convert(condition)?;
                let new_then_block = self.typecheck_statement(*then_block)?;
                let new_else_block = match else_block {
                    Some(else_block) => Some(Box::new(self.typecheck_statement(*else_block)?)),
                    None => None,
                };

                Ok(nodes::Statement { kind: nodes::StatementKind::If(new_condition, Box::new(new_then_block), new_else_block), line_started: statement.line_started })
            }
            nodes::StatementKind::While(condition, block) => {
                let new_condition = self.typecheck_and_convert(condition)?;
                let new_block = self.typecheck_statement(*block)?;

                Ok(nodes::Statement { kind: nodes::StatementKind::While(new_condition, Box::new(new_block)), line_started: statement.line_started })
            }
        }
    }

    fn typecheck_expression(&mut self, expression: nodes::Expression) -> Result<nodes::Expression, errors::Error> {
        match expression.kind {
            nodes::ExpressionKind::Number(_) => Ok(expression),
            nodes::ExpressionKind::Binary(op, left, right) => {
                let new_left = self.typecheck_and_convert(*left)?;
                let new_right = self.typecheck_and_convert(*right)?;

                if new_left.ty != new_right.ty {
                    return Err(errors::Error::new(errors::ErrorKind::TypeError, expression.line_started));
                }

                let ty = new_left.ty.clone();

                Ok(nodes::Expression { kind: nodes::ExpressionKind::Binary(op, Box::new(new_left), Box::new(new_right)), line_started: expression.line_started, ty })
            }
            nodes::ExpressionKind::Variable(ref name) => {
                if let Some(entry) = self.symbol_table.get(name) {
                    if let nodes::Type::Function(_, _) = entry.ty {
                        Err(errors::Error::new(errors::ErrorKind::TypeError, expression.line_started))
                    } else {
                        Ok(nodes::Expression { kind: nodes::ExpressionKind::Variable(name.clone()), line_started: expression.line_started, ty: entry.ty.clone() })
                    }
                } else {
                    unreachable!()
                }
            }
            nodes::ExpressionKind::FunctionCall(name, args) => {
                if let Some(entry) = self.symbol_table.get(&name) {
                    if let nodes::Type::Function(params, return_type) = &entry.ty {
                        if args.len() != params.len() {
                            return Err(errors::Error::new(errors::ErrorKind::TypeError, expression.line_started));
                        }

                        let params = params.iter().cloned().collect::<Vec<_>>();
                        let return_type = return_type.clone();

                        let new_args = args.into_iter().map(|arg| self.typecheck_and_convert(arg)).collect::<Result<Vec<_>, _>>()?;

                        for (arg, param) in new_args.iter().zip(params.iter()) {
                            if arg.ty != *param {
                                return Err(errors::Error::new(errors::ErrorKind::TypeError, expression.line_started));
                            }
                        }

                        Ok(nodes::Expression { kind: nodes::ExpressionKind::FunctionCall(name, new_args), line_started: expression.line_started, ty: *return_type })
                    } else {
                        Err(errors::Error::new(errors::ErrorKind::TypeError, expression.line_started))
                    }
                } else {
                    unreachable!()
                }
            }
            nodes::ExpressionKind::Assign(left, right) => {
                let new_left = self.typecheck_and_convert(*left)?;
                let new_right = self.typecheck_and_convert(*right)?;

                if new_left.ty != new_right.ty {
                    return Err(errors::Error::new(errors::ErrorKind::TypeError, expression.line_started));
                }

                if !self.is_lvalue(&new_left) {
                    return Err(errors::Error::new(errors::ErrorKind::InvalidAssignmentTarget, expression.line_started));
                }

                let ty = new_left.ty.clone();

                Ok(nodes::Expression { kind: nodes::ExpressionKind::Assign(Box::new(new_left), Box::new(new_right)), line_started: expression.line_started, ty })
            }
            nodes::ExpressionKind::IsZero(expr) => {
                let new_expr = self.typecheck_and_convert(*expr)?;

                if !self.is_arithmetic(&new_expr.ty) {
                    return Err(errors::Error::new(errors::ErrorKind::TypeError, expression.line_started));
                }

                let ty = new_expr.ty.clone();

                Ok(nodes::Expression { kind: nodes::ExpressionKind::IsZero(Box::new(new_expr)), line_started: expression.line_started, ty })
            }
            nodes::ExpressionKind::Dereference(inner) => {
                let new_inner = self.typecheck_and_convert(*inner)?;

                match &new_inner.ty {
                    nodes::Type::Pointer(inner_ty) => Ok({
                        let ty = *inner_ty.clone();
                        nodes::Expression { kind: nodes::ExpressionKind::Dereference(Box::new(new_inner)), line_started: expression.line_started, ty }
                    }),
                    _ => Err(errors::Error::new(errors::ErrorKind::TypeError, expression.line_started))
                }
            }
            nodes::ExpressionKind::AddressOf(inner) => {
                if !self.is_lvalue(&*inner) {
                    return Err(errors::Error::new(errors::ErrorKind::TypeError, expression.line_started));
                }

                let new_inner = self.typecheck_expression(*inner)?;
                let ty = nodes::Type::Pointer(Box::new(new_inner.ty.clone()));
                
                Ok(nodes::Expression { kind: nodes::ExpressionKind::AddressOf(Box::new(new_inner)), line_started: expression.line_started, ty })
            }
            nodes::ExpressionKind::Subscript(array, index) => {
                let new_array = self.typecheck_expression(*array)?;
                let new_index = self.typecheck_and_convert(*index)?;

                if !self.is_lvalue(&new_array) {
                    return Err(errors::Error::new(errors::ErrorKind::TypeError, expression.line_started));
                }

                match &new_array.ty {
                    nodes::Type::Pointer(inner_ty) => {
                        if !self.is_arithmetic(&new_index.ty) {
                            return Err(errors::Error::new(errors::ErrorKind::TypeError, expression.line_started));
                        }

                        Ok({
                            let ty = *inner_ty.clone();
                            nodes::Expression { kind: nodes::ExpressionKind::Subscript(Box::new(new_array), Box::new(new_index)), line_started: expression.line_started, ty }
                        })
                    }
                    _ => Err(errors::Error::new(errors::ErrorKind::TypeError, expression.line_started)),
                }
            }
        }
    }

    fn typecheck_and_convert(&mut self, expression: nodes::Expression) -> Result<nodes::Expression, errors::Error> {
        let new_expression = self.typecheck_expression(expression)?;

        Ok(new_expression)
    }

    fn is_arithmetic(&self, ty: &nodes::Type) -> bool {
        match ty {
            nodes::Type::I32 => true,
            _ => false,
        }
    }

    fn is_lvalue(&self, expression: &nodes::Expression) -> bool {
        match expression.kind {
            nodes::ExpressionKind::Variable(_) => true,
            nodes::ExpressionKind::Dereference(_) => true,
            nodes::ExpressionKind::Subscript(_, _) => true,
            _ => false,
        }
    }
}