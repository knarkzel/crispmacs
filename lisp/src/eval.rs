use crate::*;

// Eval helpers
fn expr_to_number(expr: &Expr) -> Option<i32> {
    match expr {
        Expr::Constant(Atom::Number(it)) => Some(*it),
        _ => None,
    }
}

fn number_to_expr(number: i32) -> Option<Expr> {
    Some(Expr::Constant(Atom::Number(number)))
}

fn expr_to_boolean(expr: &Expr) -> Option<bool> {
    match expr {
        Expr::Constant(Atom::Boolean(it)) => Some(*it),
        _ => None,
    }
}

fn boolean_to_expr(boolean: bool) -> Option<Expr> {
    Some(Expr::Constant(Atom::Boolean(boolean)))
}

fn numbers(tail: &[Expr]) -> Option<impl Iterator<Item = i32>> {
    tail.iter()
        .map(expr_to_number)
        .collect::<Option<Vec<_>>>()
        .map(|it| it.into_iter())
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

pub fn eval(expr: Expr) -> Option<Expr> {
    match expr {
        Expr::Constant(_) | Expr::Quote(_) => Some(expr),
        Expr::Application(head, tail) => {
            let head = eval(*head)?;
            let tail = tail.into_iter().map(eval).collect::<Option<Vec<_>>>()?;
            match head {
                Expr::Constant(Atom::BuiltIn(BuiltIn::Plus)) => {
                    number_to_expr(numbers(&tail)?.sum())
                }
                Expr::Constant(Atom::BuiltIn(BuiltIn::Minus)) => {
                    number_to_expr(numbers(&tail)?.fold(0, |a, b| a - b))
                }
                Expr::Constant(Atom::BuiltIn(BuiltIn::Times)) => {
                    number_to_expr(numbers(&tail)?.product())
                }
                Expr::Constant(Atom::BuiltIn(BuiltIn::Divide)) => {
                    if let Some(Some(car)) = car(&tail).map(expr_to_number) {
                        number_to_expr(numbers(cdr(&tail)?)?.fold(car, |a, b| a / b))
                    } else {
                        None
                    }
                }
                Expr::Constant(Atom::BuiltIn(BuiltIn::Equal)) => {
                    boolean_to_expr(tail.array_windows::<2>().all(|[a, b]| a == b))
                }
                Expr::Constant(Atom::BuiltIn(BuiltIn::Not)) => {
                    if tail.len() == 1 {
                        boolean_to_expr(!expr_to_boolean(car(&tail)?)?)
                    } else {
                        None
                    }
                }
                _ => None,
            }
        }
        _ => None,
    }
}
