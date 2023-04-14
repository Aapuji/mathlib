use std::{sync::Arc, fmt::Display, collections::HashMap};
use num_complex::Complex64;

use crate::expr::{Expr, EvalError, EvalResult};

/** Independent variable, unknown  */
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Var {
    name: String,
}

impl Var {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VAR[{}]", self.name)
    }
}

impl Expr for Var {
    fn is_variant_on(&self, var: Arc<Var>) -> bool {
        self == var.as_ref()
    }
    fn eval(&self, var_values: &HashMap<&Var, Complex64>) -> EvalResult {
        var_values.get(&self)
            .map(|v| Ok(v.clone()))
            .unwrap_or(Err(EvalError::VarMissing { name: self.name.clone() }))
    }
}

#[cfg(test)]
mod test {
    use super::Var;

    #[test]
    fn var_comparison() {
        let a = Var::new("a");
        let b = Var::new("b");
        let a2 = Var::new("a");

        assert_eq!(a, a);
        assert_eq!(a, a2);
        assert_ne!(a, b);
    }
}