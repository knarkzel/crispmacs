use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, digit1, multispace0, multispace1},
    combinator::{cut, map, map_res, opt},
    error::{context, VerboseError},
    multi::many0,
    sequence::{delimited, preceded, terminated, tuple},
    Parser,
};

// Helpers
type IResult<'a, T, U> = nom::IResult<T, U, nom::error::VerboseError<&'a str>>;

fn sexp<'a, O1, F>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O1>
where
    F: Parser<&'a str, O1, VerboseError<&'a str>>,
{
    delimited(
        char('('),
        preceded(multispace0, inner),
        context("closing paren", cut(preceded(multispace0, char(')')))),
    )
}

// Atoms
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

fn parse_built_in(input: &str) -> IResult<&str, Atom> {
    let (rest, output) =
        alt((tag("+"), tag("-"), tag("*"), tag("/"), tag("="), tag("not")))(input)?;
    let built_in = match output {
        "+" => BuiltIn::Plus,
        "-" => BuiltIn::Minus,
        "*" => BuiltIn::Times,
        "/" => BuiltIn::Divide,
        "=" => BuiltIn::Equal,
        "not" => BuiltIn::Not,
        _ => unreachable!(),
    };
    Ok((rest, Atom::BuiltIn(built_in)))
}

fn parse_boolean(input: &str) -> IResult<&str, Atom> {
    alt((
        map(tag("#t"), |_| Atom::Boolean(true)),
        map(tag("#f"), |_| Atom::Boolean(false)),
    ))(input)
}

fn parse_keyword(input: &str) -> IResult<&str, Atom> {
    map(
        context("keyword", preceded(tag(":"), cut(alpha1))),
        |keyword: &str| Atom::Keyword(keyword.to_string()),
    )(input)
}

fn parse_number(input: &str) -> IResult<&str, Atom> {
    alt((
        map_res(digit1, |digits: &str| digits.parse::<i32>().map(Atom::Number)),
        map(preceded(tag("-"), digit1), |digits: &str| {
            Atom::Number(-1 * digits.parse::<i32>().unwrap())
        }),
    ))(input)
}

fn parse_atom(input: &str) -> IResult<&str, Atom> {
    alt((parse_number, parse_boolean, parse_built_in, parse_keyword))(input)
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

impl Expr {
    fn number(&self) -> Option<i32> {
        match self {
            Expr::Constant(Atom::Number(it)) => Some(*it),
            _ => None,
        }
    }
    
    fn boolean(&self) -> Option<bool> {
        match self {
            Expr::Constant(Atom::Boolean(it)) => Some(*it),
            _ => None,
        }
    }
}

fn parse_constant(input: &str) -> IResult<&str, Expr> {
    map(parse_atom, |atom| Expr::Constant(atom))(input)
}

fn parse_application(input: &str) -> IResult<&str, Expr> {
    let inner = map(tuple((parse_expr, many0(parse_expr))), |(head, tail)| {
        Expr::Application(Box::new(head), tail)
    });
    sexp(inner)(input)
}

fn parse_if(input: &str) -> IResult<&str, Expr> {
    let inner = context(
        "if expression",
        map(
            preceded(
                terminated(tag("if"), multispace1),
                cut(tuple((parse_expr, parse_expr, opt(parse_expr)))),
            ),
            |(pred, true_branch, maybe_false_branch)| {
                if let Some(false_branch) = maybe_false_branch {
                    Expr::IfElse(
                        Box::new(pred),
                        Box::new(true_branch),
                        Box::new(false_branch),
                    )
                } else {
                    Expr::If(Box::new(pred), Box::new(true_branch))
                }
            },
        ),
    );
    sexp(inner)(input)
}

fn parse_quote(input: &str) -> IResult<&str, Expr> {
    map(
        context("quote", preceded(tag("'"), cut(sexp(many0(parse_expr))))),
        |exprs| Expr::Quote(exprs),
    )(input)
}

fn parse_expr(input: &str) -> IResult<&str, Expr> {
    preceded(
        multispace0,
        alt((parse_constant, parse_application, parse_if, parse_quote)),
    )(input)
}

fn main() {
    dbg!(parse_expr("((if (= (+ 3 (/ 9 3)) (* 2 3)) * /) 456 123)"));
}
