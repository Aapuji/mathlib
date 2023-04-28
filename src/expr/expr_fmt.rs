use std::fmt;

use super::Expr;

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Sum(terms) => {
                let mut k: Vec<String> = vec![];
                for term in terms {
                    k.push(format!("{}", term));
                }
                write!(f, "({})", k.join(" + "))
            },
            Expr::Product(terms) => {
                let mut k: Vec<String> = vec![];
                for term in terms {
                    k.push(format!("{}", term));
                }
                write!(f, "({})", k.join(" * "))
            },
            Expr::Var(var) => write!(f, "VAR[{}]", var.get_name()),
            Expr::Const(num) => write!(f, "CONST[{}]", num),
            Expr::Function(_, _) => todo!(),
        }
    }
}