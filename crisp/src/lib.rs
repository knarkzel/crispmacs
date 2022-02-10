#![feature(if_let_guard)]

mod eval;
mod parse;

use anyhow::bail;
pub use eval::Context;
use fehler::throws;
pub use parse::parse;
use std::fmt::Display;
type Error = anyhow::Error;
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
    Number(i32),
    Keyword(String),
    Boolean(bool),
    BuiltIn(BuiltIn),
    Symbol(String),
}

impl Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(number) => write!(f, "{}", number),
            Self::Keyword(keyword) => write!(f, ":{}", keyword),
            Self::Boolean(boolean) => {
                if *boolean {
                    write!(f, "true")
                } else {
                    write!(f, "false")
                }
            }
            Self::BuiltIn(built_in) => write!(f, "{}", built_in),
            Self::Symbol(symbol) => write!(f, "{}", symbol),
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
    Let(Atom, Box<Expr>),
    /// (fn (x y z) (+ x y z))
    Function(Vec<Expr>, Box<Expr>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Constant(atom) => write!(f, "{}", atom),
            Self::Call(head, tail) => {
                write!(f, "({}", head)?;
                for expr in tail {
                    write!(f, " {}", expr)?;
                }
                write!(f, ")")
            }
            Self::If(predicate, then, otherwise) => {
                write!(f, "(if {} {} {:?})", predicate, then, otherwise)
            }
            Self::Quote(expr) => {
                write!(f, "(")?;
                for (i, expr) in expr.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", expr)?;
                }
                write!(f, ")")
            }
            Self::Let(name, body) => write!(f, "(let {} {})", name, body),
            Self::Function(args, body) => {
                write!(f, "(fn (")?;
                for (i, expr) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", expr)?;
                }
                write!(f, ") {})", body)
            }
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
