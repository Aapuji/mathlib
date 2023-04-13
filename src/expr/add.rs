use std::{sync::Arc, fmt::Display};

use crate::var::Var;

use super::{ArcExpr, Expr};


impl std::ops::Add for &ArcExpr {
    type Output = ArcExpr;
    fn add(self, rhs: Self) -> Self::Output {
        ArcExpr(Arc::new(ExprAdd::new(vec![self.0.clone(),rhs.0.clone()])))
    }
}

#[derive(Debug)]
pub struct ExprAdd {
    children: Vec<Arc<dyn Expr>>,
}

impl ExprAdd {
    pub fn new(children: Vec<Arc<dyn Expr>>) -> Self {
        Self {
            children,
        }
    }
}

impl Display for ExprAdd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TODO")
    }
}

impl Expr for ExprAdd {
    fn is_variant_on(&self, var: Arc<Var>) -> bool {
        self.children[..]
            .into_iter()
            .map(|v| v.is_variant_on(var.clone()))
            .reduce(|a, b| a || b)
            .unwrap_or(false)
    }
}