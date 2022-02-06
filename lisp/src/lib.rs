#![feature(array_windows)]

mod parse;
mod eval;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BuiltIn {
    Plus,
    Minus,
    Times,
    Divide,
    Equal,
    Not,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Atom {
    Number(i32),
    Keyword(String),
    Boolean(bool),
    BuiltIn(BuiltIn),
}

// Expressions
#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Constant(Atom),
    /// (func-name arg1 arg2)
    Application(Box<Expr>, Vec<Expr>),
    /// (if predicate do-this)
    If(Box<Expr>, Box<Expr>),
    /// (if predicate do-this otherwise-do-this)
    IfElse(Box<Expr>, Box<Expr>, Box<Expr>),
    /// '(3 (if (+ 3 3) 4 5) 7)
    Quote(Vec<Expr>),
}

pub fn parse_and_eval(input: &str) -> Result<Vec<Expr>, nom_supreme::error::ErrorTree<&str>> {
    parse::parse(input).map(|items| items.into_iter().filter_map(eval::eval).collect::<Vec<_>>())
}
