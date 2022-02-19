use crate::*;

// In-built functions for data types
pub fn std(head: &Expr, tail: &[Expr]) -> Result<Expr, Error> {
    match head {
        Expr::Constant(Atom::Symbol(name)) => match (name.as_str(), tail) {
            ("car", [Expr::Quote(items)]) => match items.len() {
                0 => Ok(Expr::Nil),
                _ => Ok(items[0].clone()),
            },
            ("car", [Expr::Nil]) => Ok(Expr::Nil),
            ("cdr", [Expr::Quote(items)]) => match items.len() {
                0 | 1 => Ok(Expr::Nil),
                2 => Ok(items[1].clone()),
                _ => Ok(Expr::Quote(items[1..].to_vec())),
            },
            ("cdr", [Expr::Nil]) => Ok(Expr::Nil),
            _ => bail!("No valid functions found"),
        },
        _ => bail!("No valid functions found"),
    }
}
