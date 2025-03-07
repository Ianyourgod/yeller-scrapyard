#![allow(dead_code)]

#[derive(Debug, Clone)]
pub struct Program {
    pub functions: Vec<FunctionDefinition>,
}

#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    pub name: String,
    pub params: Vec<(String, Type)>,
    pub return_type: Type,
    pub body: Option<Block>,
    pub line_started: usize,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub items: Vec<BlockItem>,
    pub line_started: usize,
}

#[derive(Debug, Clone)]
pub enum BlockItem {
    Statement(Statement),
    Declaration(Declaration),
}

#[derive(Debug, Clone)]
pub struct Declaration {
    pub name: String,
    pub ty: Type,
    pub value: Expression,
    pub line_started: usize,
}

#[derive(Debug, Clone)]
pub struct Statement {
    pub kind: StatementKind,
    pub line_started: usize,
}

#[derive(Debug, Clone)]
pub enum StatementKind {
    Return(Expression),
    Block(Block),
    Expression(Expression),
    If(Expression, Box<Statement>, Option<Box<Statement>>),
    While(Expression, Box<Statement>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub line_started: usize,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionKind {
    Number(u64),
    Binary(Binop, Box<Expression>, Box<Expression>),
    Variable(String),
    Assign(Box<Expression>, Box<Expression>),
    IsZero(Box<Expression>),
    FunctionCall(String, Vec<Expression>),
    AddressOf(Box<Expression>),
    Dereference(Box<Expression>),
    Subscript(Box<Expression>, Box<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Binop {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    I32,
    Pointer(Box<Type>),
    Function(Vec<Type>, Box<Type>),
}