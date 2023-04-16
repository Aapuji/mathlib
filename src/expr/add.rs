use std::{fmt::Display, sync::Arc, collections::HashMap};
use num_complex::Complex64;

use crate::var::Var;
use super::{ArcExpr, Expr};

impl std::ops::Add for ArcExpr {
    type Output = ArcExpr;

    fn add(self, rhs: Self) -> Self::Output {
        ArcExpr(Arc::new(AddExpr::new(vec![self.0, rhs.0])))
    }
}

#[derive(Debug)]
pub struct AddExpr {
    operands: Vec<Arc<dyn Expr>>,
}

impl AddExpr {
    pub fn new(operands: Vec<Arc<dyn Expr>>) -> Self {
        Self { operands }
    }
}

impl Display for AddExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        for (i, arcexpr) in self.operands[..].into_iter().enumerate() {
            if i > 0 {
                write!(f, " + ")?;
            }
            write!(f, "{}", arcexpr)?;
        }
        write!(f, ")")?;

        Ok(())
    }
}

impl Expr for AddExpr {
    fn is_variant_on(&self, var: Arc<Var>) -> bool {
        self.operands[..]
            .into_iter()
            .map(|v| v.is_variant_on(var.clone()))
            .reduce(|a, b| a || b)
            .unwrap_or(false)
    }
    fn eval(&self, var_values: &HashMap<&Var, Complex64>) -> super::EvalResult {
        let mut sum = Complex64::new(0.0, 0.0);
        for term in &self.operands {
            sum += term.eval(var_values)?
        }
        Ok(sum)
    }
}
