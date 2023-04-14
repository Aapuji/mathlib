use std::{fmt::Display, sync::Arc};
use num_complex::Complex64;

use crate::var::Var;
use super::{ArcExpr, Expr};

impl std::ops::Add for ArcExpr {
    type Output = ArcExpr;

    fn add(self, rhs: Self) -> Self::Output {
        ArcExpr(Arc::new(ExprAdd::new(vec![self.0, rhs.0])))
    }
}

#[derive(Debug)]
pub struct ExprAdd {
    children: Vec<Arc<dyn Expr>>,
}

impl ExprAdd {
    pub fn new(children: Vec<Arc<dyn Expr>>) -> Self {
        Self { children }
    }
}

impl Display for ExprAdd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        for (i,arcexpr) in self.children[..].into_iter().enumerate() {
            if i > 0 {
                write!(f, " + ")?;
            }
            write!(f, "{}", arcexpr)?;
        }
        write!(f, ")")?;

        Ok(())
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
    fn eval(&self, var_values: &std::collections::HashMap<&Var, num_complex::Complex64>) -> super::EvalResult {
        let mut sum = Complex64::new(0.0, 0.0);
        for term in &self.children {
            sum += term.eval(var_values)?
        }
        Ok(sum)
    }
}
