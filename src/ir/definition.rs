#![allow(dead_code)]

pub use crate::parser::nodes::Type;

#[derive(Debug, Clone)]
pub struct Program {
    pub functions: Vec<Function>,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<(String, Type)>,
    pub return_type: Type,
    pub body: Vec<Instruction>,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Copy {
        src: Val,
        dst: Val,
    },
    Binary {
        op: Binop,
        src1: Val,
        src2: Val,
        dst: Val,
    },
    Return(Val),
    Label(String),
    Jump(String),
    JumpIfZero(Val, String),
    JumpIfNotZero(Val, String),
}

#[derive(Debug, Clone)]
pub enum Binop {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Debug, Clone)]
pub enum Val {
    Var(String),
    Number(u64),
}