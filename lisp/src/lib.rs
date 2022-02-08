#![feature(if_let_guard)]

mod eval;
mod parse;

pub use eval::Context;
pub use parse::parse;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BuiltIn {
    Plus,
    Minus,
    Times,
    Divide,
    Equal,
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
            Self::Not => write!(f, "not"),
            Self::GreaterEqual => write!(f, ">="),
            Self::Greater => write!(f, ">"),
            Self::LessEqual => write!(f, "<="),
            Self::Less => write!(f, "<"),
            Self::And => write!(f, "and"),
            Self::Or => write!(f, "or"),
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
                    write!(f, "#t")
                } else {
                    write!(f, "#f")
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
    Application(Box<Expr>, Vec<Expr>),
    /// (if predicate do-this)
    If(Box<Expr>, Box<Expr>),
    /// (if predicate do-this otherwise-do-this)
    IfElse(Box<Expr>, Box<Expr>, Box<Expr>),
    /// '(3 (if (+ 3 3) 4 5) 7)
    Quote(Vec<Expr>),
    /// (define red 123)
    Define(Atom, Box<Expr>),
    /// (lambda (x y z) (+ x y z))
    Lambda(Vec<Expr>, Box<Expr>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Constant(atom) => write!(f, "{}", atom),
            Self::Application(head, tail) => {
                write!(f, "({}", head)?;
                for expr in tail {
                    write!(f, " {}", expr)?;
                }
                write!(f, ")")
            }
            Self::If(predicate, then) => {
                write!(f, "(if {} {})", predicate, then)
            }
            Self::IfElse(predicate, then, otherwise) => {
                write!(f, "(if {} {} {})", predicate, then, otherwise)
            }
            Self::Quote(expr) => {
                write!(f, "'(")?;
                for (i, expr) in expr.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", expr)?;
                }
                write!(f, ")")
            }
            Self::Define(name, body) => write!(f, "(define {} {})", name, body),
            Self::Lambda(args, body) => {
                write!(f, "(lambda (")?;
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

pub fn parse_and_eval<'a>(
    input: &'a str,
    context: &mut eval::Context,
) -> Result<Vec<Expr>, nom_supreme::error::ErrorTree<&'a str>> {
    parse::parse(input).map(|items| {
        items
            .into_iter()
            .filter_map(|it| context.eval(it))
            .collect::<Vec<_>>()
    })
}
