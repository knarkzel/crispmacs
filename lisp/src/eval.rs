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

fn booleans(tail: &[Expr]) -> Option<impl Iterator<Item = bool>> {
    tail.iter()
        .map(expr_to_boolean)
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

pub fn eval(expr: Expr) -> Option<Expr> {
    match expr {
        Expr::Constant(_) | Expr::Quote(_) => Some(expr),
        Expr::Application(head, tail) => {
            let head = eval(*head)?;
            let tail = tail.into_iter().map(eval).collect::<Option<Vec<_>>>()?;
            match head {
                Expr::Constant(Atom::BuiltIn(built_in)) => match built_in {
                    BuiltIn::Greater => logic!(tail => a > b),
                    BuiltIn::Less => logic!(tail => a < b),
                    BuiltIn::GreaterEqual => logic!(tail => a >= b),
                    BuiltIn::LessEqual => logic!(tail => a <= b),
                    BuiltIn::Plus => number_to_expr(numbers(&tail)?.sum()),
                    BuiltIn::Minus => number_to_expr(numbers(&tail)?.fold(0, |a, b| a - b)),
                    BuiltIn::Times => number_to_expr(numbers(&tail)?.product()),
                    BuiltIn::Equal => boolean_to_expr(tail.windows(2).all(|it| it[0] == it[1])),
                    BuiltIn::And => boolean_to_expr(booleans(&tail)?.all(|it| it)),
                    BuiltIn::Or => boolean_to_expr(booleans(&tail)?.any(|it| it)),
                    BuiltIn::Divide => {
                        if let Some(Some(car)) = car(&tail).map(expr_to_number) {
                            number_to_expr(numbers(cdr(&tail)?)?.fold(car, |a, b| a / b))
                        } else {
                            None
                        }
                    }
                    BuiltIn::Not => {
                        if tail.len() == 1 {
                            boolean_to_expr(!expr_to_boolean(car(&tail)?)?)
                        } else {
                            None
                        }
                    }
                    _ => None,
                },
                _ => None,
            }
        }
        _ => None,
    }
}
