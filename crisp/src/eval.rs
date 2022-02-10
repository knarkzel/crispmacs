use crate::*;
use std::collections::HashMap;

// Eval helpers
#[throws]
fn expr_to_number(expr: &Expr) -> i32 {
    match expr {
        Expr::Constant(Atom::Number(it)) => *it,
        _ => bail!("Invalid number passed: {}", expr),
    }
}

fn number_to_expr(number: i32) -> Expr {
    Expr::Constant(Atom::Number(number))
}

#[throws]
fn expr_to_boolean(expr: &Expr) -> bool {
    match expr {
        Expr::Constant(Atom::Boolean(it)) => *it,
        _ => bail!("Invalid boolean passed: {expr}"),
    }
}

fn boolean_to_expr(boolean: bool) -> Expr {
    Expr::Constant(Atom::Boolean(boolean))
}

#[throws]
fn numbers(tail: &[Expr]) -> impl Iterator<Item = i32> {
    tail.iter()
        .map(expr_to_number)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
}

#[throws]
fn booleans(tail: &[Expr]) -> impl Iterator<Item = bool> {
    tail.iter()
        .map(expr_to_boolean)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
}

fn car<T>(tail: &[T]) -> Option<&T> {
    tail.first()
}

fn cdr<T>(tail: &[T]) -> Option<&[T]> {
    match tail.len() > 0 {
        true => Some(&tail[1..]),
        false => None,
    }
}

// Curry lambda by recursively translating each variable into its parameter
#[throws]
fn curry(expr: Expr, left: &[Expr], right: &[Expr], marked: &mut [bool]) -> Expr {
    let mut single = |expr| -> Result<Expr, Error> { curry(expr, left, right, marked) };
    match expr {
        // Handle variable
        symbol if let Some(index) = left.iter().position(|it| *it == symbol) => {
            marked[index] = true;
            match right.get(index) {
                Some(parameter) => parameter.clone(),
                None => bail!("Index out of bounds"),
            }
        },
        // Handle other forms
        Expr::Call(head, tail) => {
            let head = Box::new(single(*head)?);
            let tail = tail.into_iter().map(single).collect::<Result<Vec<_>, _>>()?;
            Expr::Call(head, tail)
        },
        Expr::If(predicate, then, otherwise) => Expr::If(Box::new(single(*predicate)?), Box::new(single(*then)?), otherwise.and_then(|it| single(*it).ok().map(Box::new))),
        it => it,
    }
}

// Macros
macro_rules! logic {
	($tail:ident => $a:ident $op:tt $b:ident) => {
		boolean_to_expr($tail.windows(2).all(|it| match (&it[0], &it[1]) {
            (Expr::Constant(Atom::Number($a)), Expr::Constant(Atom::Number($b))) => {
                $a $op $b
            }
            _ => false,
        }))
	};
}

// Context
#[derive(Default)]
pub struct Context {
    symbols: HashMap<String, Expr>,
}

impl Context {
    pub fn eval(&mut self, expr: Expr) -> Result<Expr, Error> {
        match expr {
            Expr::Constant(Atom::Symbol(symbol)) => match self.symbols.get(&symbol) {
                Some(expr) => Ok(expr.clone()),
                None => bail!("Invalid variable: {symbol}"),
            },
            Expr::Constant(_) | Expr::Quote(_) => Ok(expr),
            Expr::Let(Atom::Symbol(symbol), expr) => {
                let expr = self.eval(*expr)?;
                self.symbols.insert(symbol.clone(), expr);
                bail!("Declared variable: {symbol}")
            }
            Expr::If(predicate, then, otherwise) => {
                let predicate = self.eval(*predicate)?;
                if expr_to_boolean(&predicate)? {
                    self.eval(*then)
                } else if let Some(branch) = otherwise {
                    self.eval(*branch)
                } else {
                    bail!("No branches of predicate ran: {predicate}")
                }
            }
            Expr::Call(head, tail) => {
                let head = self.eval(*head)?;
                let tail = tail
                    .into_iter()
                    .map(|it| self.eval(it))
                    .collect::<Result<Vec<_>, _>>()?;
                match head {
                    Expr::Function(args, expr) => {
                        let mut marked = (0..args.len()).map(|_| false).collect::<Vec<_>>();
                        let expr = curry(*expr, &args, &tail, &mut marked)?;
                        let args = args
                            .into_iter()
                            .zip(marked.into_iter())
                            .filter_map(|(it, marked)| if marked { None } else { Some(it) })
                            .collect::<Vec<_>>();
                        match args.len() == 0 {
                            true => self.eval(expr),
                            false => Ok(Expr::Function(args, Box::new(expr))),
                        }
                    }
                    Expr::Constant(Atom::BuiltIn(built_in)) => Ok(match built_in {
                        BuiltIn::Greater => logic!(tail => a > b),
                        BuiltIn::Less => logic!(tail => a < b),
                        BuiltIn::GreaterEqual => logic!(tail => a >= b),
                        BuiltIn::LessEqual => logic!(tail => a <= b),
                        BuiltIn::Plus => number_to_expr(numbers(&tail)?.sum()),
                        BuiltIn::Minus => match car(&tail).map(expr_to_number) {
                            Some(Ok(car)) => {
                                number_to_expr(numbers(cdr(&tail).unwrap_or_default())?.fold(car, |a, b| a - b))
                            }
                            _ => bail!("No car found for minus"),
                        },
                        BuiltIn::Times => number_to_expr(numbers(&tail)?.product()),
                        BuiltIn::Equal => boolean_to_expr(tail.windows(2).all(|it| it[0] == it[1])),
                        BuiltIn::NotEqual => {
                            boolean_to_expr(tail.windows(2).all(|it| it[0] != it[1]))
                        }
                        BuiltIn::And => boolean_to_expr(booleans(&tail)?.all(|it| it)),
                        BuiltIn::Or => boolean_to_expr(booleans(&tail)?.any(|it| it)),
                        BuiltIn::Divide => match car(&tail).map(expr_to_number) {
                            Some(Ok(car)) => {
                                number_to_expr(numbers(cdr(&tail).unwrap_or_default())?.fold(car, |a, b| a / b))
                            }
                            _ => bail!("No car found for divide"),
                        },
                        BuiltIn::Not => match (tail.len() == 1, car(&tail)) {
                            (true, Some(car)) => boolean_to_expr(!expr_to_boolean(car)?),
                            _ => bail!("! expects 1 parameter only"),
                        },
                    }),
                    it => Ok(it),
                }
            }
            it => Ok(it),
        }
    }
}
