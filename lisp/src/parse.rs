use crate::*;
use nom::{
    branch::alt,
    character::complete::{alpha1, alphanumeric1, char, digit1, multispace0, multispace1},
    combinator::{cut, map, map_res},
    multi::{many0, many1},
    sequence::{delimited, preceded, terminated, tuple},
    Parser,
};
use nom_supreme::{error::ErrorTree, final_parser::final_parser, tag::complete::tag, ParserExt};

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

fn ws<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: Fn(&'a str) -> IResult<&'a str, O>,
{
    delimited(multispace0, inner, multispace0)
}

// Atoms
fn parse_built_in(input: &str) -> IResult<&str, Atom> {
    map(
        alt((
            map(tag("+"), |_| BuiltIn::Plus),
            map(tag("-"), |_| BuiltIn::Minus),
            map(tag("*"), |_| BuiltIn::Times),
            map(tag("/"), |_| BuiltIn::Divide),
            map(tag("="), |_| BuiltIn::Equal),
            map(tag(">="), |_| BuiltIn::GreaterEqual),
            map(tag("<="), |_| BuiltIn::LessEqual),
            map(tag(">"), |_| BuiltIn::Greater),
            map(tag("<"), |_| BuiltIn::Less),
            map(tag("not"), |_| BuiltIn::Not),
            map(tag("and"), |_| BuiltIn::And),
            map(tag("or"), |_| BuiltIn::Or),
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

fn parse_symbol(input: &str) -> IResult<&str, Atom> {
    map(alphanumeric1, |symbol: &str| {
        Atom::Symbol(symbol.to_string())
    })(input)
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
    alt((
        parse_number,
        parse_boolean,
        parse_built_in,
        parse_keyword,
        parse_symbol,
    ))(input)
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
        preceded(ws(tag("if")), cut(tuple((parse_expr, parse_expr)))),
        |(predicate, then)| Expr::If(Box::new(predicate), Box::new(then)),
    ))(input)
}

fn parse_if_else(input: &str) -> IResult<&str, Expr> {
    sexp(map(
        preceded(
            ws(tag("if")),
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

fn parse_define(input: &str) -> IResult<&str, Expr> {
    sexp(map(
        preceded(ws(tag("define")), cut(tuple((parse_symbol, parse_expr)))),
        |(name, expr)| Expr::Define(name, Box::new(expr)),
    ))(input)
}

fn parse_lambda(input: &str) -> IResult<&str, Expr> {
    sexp(map(
        preceded(
            ws(tag("lambda")),
            cut(tuple((sexp(many0(ws(parse_symbol))), parse_expr))),
        ),
        |(args, expr)| {
            Expr::Lambda(
                args.into_iter().map(|it| Expr::Constant(it)).collect(),
                Box::new(expr),
            )
        },
    ))(input)
}

fn parse_expr(input: &str) -> IResult<&str, Expr> {
    preceded(
        multispace0,
        alt((
            parse_if_else,
            parse_if,
            parse_define,
            parse_lambda,
            parse_quote,
            parse_constant,
            parse_application,
        )),
    )(input)
}

pub fn parse(input: &str) -> Result<Vec<Expr>, ErrorTree<&str>> {
    final_parser(many1(delimited(multispace0, parse_expr, multispace0)))(input)
}
