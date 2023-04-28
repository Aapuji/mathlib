use std::{sync::Arc, collections::HashMap, ops::{Add, Mul, Sub}, fmt::{Display, Formatter, self}};
use num_complex::Complex64;

use crate::{var::Var, num::Num, function::FuncDef};

mod expr_fmt;

#[derive(Debug)]
pub enum EvalError {
    VarMissing { name: String },
    FnArgCountMismatch { },
}

impl Display for EvalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::VarMissing { name } => write!(f, "Missing variable '{name}' in eval."),
            Self::FnArgCountMismatch {  } => write!(f, "Incorrect number of function arguments.")
        }
    }
}
pub type EvalResult = Result<Complex64, EvalError>;


#[derive(Debug, Clone)]
pub enum Expr {
    Sum(Vec<Expr>),
    Product(Vec<Expr>),
    Var(Arc<Var>),
    Const(Num),
    Function(Arc<dyn FuncDef>, Vec<Expr>),
}

impl Expr {
    pub fn eval(&self, var_values: &HashMap<&Var, Complex64>) -> EvalResult {
        match self {
            Self::Sum(terms) => {
                let mut sum = Complex64::new(0.0, 0.0);

                for term in terms {
                    sum += term.eval(var_values)?
                }

                Ok(sum)
            }

            Self::Product(terms) => {
                let mut product = Complex64::new(1.0, 0.0);

                for term in terms {
                    product *= term.eval(var_values)?
                }

                Ok(product)
            }

            Self::Var(var) => {
                Ok(var_values.get(var.as_ref())
                    .ok_or(EvalError::VarMissing { name: var.get_name() })?
                    .clone()
                )
            }

            Self::Const(num) => {
                Ok(num.eval_float())
            }

            Self::Function(def, args) => {
                let mut evaluated_args = Vec::with_capacity(args.len());

                for arg in args {
                    evaluated_args.push(arg.eval(var_values)?);
                }

                def.eval(evaluated_args, var_values)
            }
        }
    }

    pub fn is_variant_on(&self, var: &Var) -> bool {
        match self {
            Self::Sum(terms) | Self::Product(terms) => {
                let mut is_variant = false;
                
                for term in terms {
                    is_variant |= term.is_variant_on(var)
                }

                is_variant
            }

            Self::Var(checking_var) => {
                var == checking_var.as_ref()
            }

            Self::Const( .. ) => {
                false
            }

            Self::Function(def, args) => {
                let mut is_variant = false;
                
                for arg in args {
                    is_variant |= arg.is_variant_on(var)
                }
                is_variant |= def.is_variant_on_global(var);
                
                is_variant
            }
        }
    }
}

impl Add for Expr {
    type Output = Expr;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Sum(vec![self, rhs])
    }
}

impl Sub for Expr {
    type Output = Expr;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Sum(vec![
            self, 
            Self::Product(vec![
                Self::Const(Num::Rational { num: -1, den: 1 }),
                rhs
            ])
        ])
    }
}

impl Mul for Expr {
    type Output = Expr;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::Product(vec![self, rhs])
    }
}