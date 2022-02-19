#![feature(if_let_guard)]

mod eval;
mod parse;
mod core;

use anyhow::bail;
pub use eval::Context;
pub use fehler::throws;
pub use parse::parse;
use std::fmt::Display;
use num::bigint::BigInt;
pub type Error = anyhow::Error;
use beau_collector::BeauCollector as _;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BuiltIn {
    Plus,
    Minus,
    Times,
    Divide,
    Equal,
    NotEqual,
    Not,
    GreaterEqual,
    Greater,
    LessEqual,
    Less,
    And,
    Or,
}

impl Display for BuiltIn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Times => write!(f, "*"),
            Self::Divide => write!(f, "/"),
            Self::Equal => write!(f, "="),
            Self::NotEqual => write!(f, "!="),
            Self::Not => write!(f, "!"),
            Self::GreaterEqual => write!(f, ">="),
            Self::Greater => write!(f, ">"),
            Self::LessEqual => write!(f, "<="),
            Self::Less => write!(f, "<"),
            Self::And => write!(f, "&&"),
            Self::Or => write!(f, "||"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Atom {
    Number(BigInt),
    Float(f64),
    Keyword(String),
    BuiltIn(BuiltIn),
    Symbol(String),
    String(String),
    Char(char),
}

impl Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(number) => write!(f, "{number}"),
            Self::Float(float) => write!(f, "{float}"),
            Self::Keyword(keyword) => write!(f, ":{keyword}"),
            Self::BuiltIn(built_in) => write!(f, "{built_in}"),
            Self::Symbol(symbol) => write!(f, "{symbol}"),
            Self::String(string) => write!(f, "\"{string}\""),
            Self::Char(letter) => write!(f, "'{letter}'"),
        }
    }
}

// Expressions
#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Constant(Atom),
    /// (func-name arg1 arg2 arg3 ...)
    Call(Box<Expr>, Vec<Expr>),
    /// (if predicate then otherwise)
    If(Box<Expr>, Box<Expr>, Option<Box<Expr>>),
    /// '(3 (if (+ 3 3) 4 5) 7)
    Quote(Vec<Expr>),
    /// (let red 123)
    Let(Vec<(Atom, Box<Expr>)>),
    /// (fn (x y z) (+ x y z))
    Function(Vec<Expr>, Box<Expr>),
    /// nil
    Nil,
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Constant(atom) => write!(f, "{atom}"),
            Self::Call(head, tail) => {
                write!(f, "({head}")?;
                for expr in tail {
                    write!(f, " {expr}")?;
                }
                write!(f, ")")
            }
            Self::If(predicate, then, otherwise) => {
                match otherwise {
                    Some(it) => write!(f, "(if {predicate} {then} {it})"),
                    None => write!(f, "(if {predicate} {then})")
                }
            }
            Self::Quote(expr) => match expr.len() {
                0 => write!(f, ""),
                1 => write!(f, "{}", expr[0]),
                _ => {
                    write!(f, "(")?;
                    for (i, expr) in expr.iter().enumerate() {
                        if i > 0 {
                            write!(f, " ")?;
                        }
                        write!(f, "{expr}")?;
                    }
                    write!(f, ")")
                }
            },
            Self::Let(items) => {
                write!(f, "(let ")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{} {}", item.0, item.1)?;
                }
                write!(f, ")")
            }
            Self::Function(args, body) => {
                write!(f, "(fn (")?;
                for (i, expr) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{expr}")?;
                }
                write!(f, ") {body})")
            }
            Self::Nil => write!(f, "nil"),
        }
    }
}

// Collect all errors using https://github.com/tarquin-the-brave/beau-collector/
pub fn parse_and_eval<'a>(
    input: &'a str,
    context: &mut eval::Context,
) -> Result<Result<Vec<Expr>, anyhow::Error>, nom_supreme::error::ErrorTree<&'a str>> {
    parse::parse(input).map(|items| {
        items
            .into_iter()
            .map(|it| context.eval(it))
            .bcollect::<Vec<_>>()
    })
}

