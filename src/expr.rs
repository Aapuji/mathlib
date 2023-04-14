use std::{sync::Arc, fmt::{Debug, Display, self}, collections::HashMap};
use num_complex::Complex64;

use super::var::Var;

pub mod add;
pub mod mul;
pub mod constant;

pub trait Expr : Debug + Display {
    fn is_variant_on(&self, var: Arc<Var>) -> bool;
    fn eval(&self, var_values: &HashMap<&Var, Complex64>) -> EvalResult;
}

#[derive(Debug)]
pub enum EvalError {
    VarMissing { name: String },
}
impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::VarMissing { name } => write!(f,"Missing variable '{name}' in eval."),
        }
    }
}

pub type EvalResult = Result<Complex64, EvalError>;

pub struct ArcExpr(pub Arc<dyn Expr>);
impl Clone for ArcExpr {
    fn clone(&self) -> Self {
        ArcExpr(self.0.clone())
    }
}

#[macro_export]
macro_rules! arcify_expr {
    ($expr: expr) => {{
        ArcExpr(Arc::new($expr))
    }};
}
