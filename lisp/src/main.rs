use nom::{
    branch::alt,
    character::complete::{alpha1, char, digit1, multispace0, multispace1},
    combinator::{cut, map, map_res},
    multi::many0,
    sequence::{delimited, preceded, terminated, tuple},
    Parser,
};
use nom_supreme::{
    error::ErrorTree,
    tag::complete::tag,
    ParserExt,
};

// Helpers
type IResult<'a, T, U> = nom::IResult<T, U, ErrorTree<&'a str>>;

fn sexp<'a, O1, F>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O1>
where
    F: Parser<&'a str, O1, ErrorTree<&'a str>>,
{
    delimited(
        char('('),
        preceded(multispace0, inner),
        cut(preceded(multispace0, char(')'))),
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
    map(
        alt((
            map(tag("+"), |_| BuiltIn::Plus),
            map(tag("-"), |_| BuiltIn::Minus),
            map(tag("*"), |_| BuiltIn::Times),
            map(tag("/"), |_| BuiltIn::Divide),
            map(tag("="), |_| BuiltIn::Equal),
            map(tag("not"), |_| BuiltIn::Not),
        ))
        .context("operator"),
        |built_in| Atom::BuiltIn(built_in),
    )(input)
}

fn parse_boolean(input: &str) -> IResult<&str, Atom> {
    alt((
        map(tag("#t"), |_| Atom::Boolean(true)),
        map(tag("#f"), |_| Atom::Boolean(false)),
    ))
    .context("boolean")
    .parse(input)
}

fn parse_keyword(input: &str) -> IResult<&str, Atom> {
    map(
        preceded(tag(":"), cut(alpha1)).context("keyword"),
        |keyword: &str| Atom::Keyword(keyword.to_string()),
    )(input)
}

fn parse_number(input: &str) -> IResult<&str, Atom> {
    alt((
        map_res(digit1, |digits: &str| {
            digits.parse::<i32>().map(Atom::Number)
        }),
        map_res(preceded(tag("-"), digit1), |digits: &str| {
            digits.parse::<i32>().map(|it| Atom::Number(it * -1))
        }),
    ))
    .context("number")
    .parse(input)
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
    sexp(map(
        preceded(
            terminated(tag("if"), multispace1),
            cut(tuple((parse_expr, parse_expr))),
        ),
        |(predicate, then)| Expr::If(Box::new(predicate), Box::new(then)),
    ))(input)
}

fn parse_if_else(input: &str) -> IResult<&str, Expr> {
    sexp(map(
        preceded(
            terminated(tag("if"), multispace1),
            cut(tuple((parse_expr, parse_expr, parse_expr))),
        ),
        |(predicate, then, otherwise)| {
            Expr::IfElse(Box::new(predicate), Box::new(then), Box::new(otherwise))
        },
    ))(input)
}

fn parse_quote(input: &str) -> IResult<&str, Expr> {
    map(preceded(tag("'"), cut(sexp(many0(parse_expr)))), |exprs| {
        Expr::Quote(exprs)
    })(input)
}

fn parse_expr(input: &str) -> IResult<&str, Expr> {
    preceded(
        multispace0,
        alt((
            parse_constant,
            parse_application,
            parse_if_else,
            parse_if,
            parse_quote,
        )),
    )(input)
}

fn main() {
    dbg!(parse_expr("((^ (= (+ 3 (/ 9 3)) (* 2 3)) * /) 456 123)"));
}
