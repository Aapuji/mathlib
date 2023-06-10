use num_complex::Complex64;
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::ops::{Add, Mul, Sub, Div};
use std::sync::Arc;

use crate::{function::FuncDef, num::Num, var::Var};

/// A type represnting a possible error during expression evaluation
#[derive(Debug)]
pub enum EvalError {
    VarMissing { name: String },
    FnArgCountMismatch {},
}

impl Display for EvalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::VarMissing { name } => write!(f, "Missing variable '{name}' in eval."),
            Self::FnArgCountMismatch {} => write!(f, "Incorrect number of function arguments."),
        }
    }
}

pub type EvalResult = Result<Complex64, EvalError>;

/// A type representing a mathematical expression.
#[derive(Debug, Clone)]
pub enum Expr {
    Sum(Vec<Expr>),
    Product(Vec<Expr>),
    Var(Arc<Var>),
    Const(Num),
    Function(Arc<dyn FuncDef>, Vec<Expr>),
}

impl Expr {
    /// Evaluates an expression given a context of variables and their values.
    pub fn eval(&self, var_values: &HashMap<&Var, Complex64>) -> EvalResult {
        match self {
            Self::Sum(terms) => {
                let mut sum = Complex64::new(0.0, 0.0);

                for term in terms {
                    sum += term.eval(var_values)?;
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

            Self::Var(var) => Ok(var_values
                .get(var.as_ref())
                .ok_or(EvalError::VarMissing {
                    name: var.get_name(),
                })?
                .clone()),

            Self::Const(num) => Ok(num.eval_float()),

            Self::Function(def, args) => {
                let mut evaluated_args = Vec::with_capacity(args.len());

                for arg in args {
                    evaluated_args.push(arg.eval(var_values)?);
                }

                def.eval(evaluated_args, var_values)
            }
        }
    }

    /// Determines whether an expression is variant on the given variable.
    pub fn is_variant_on(&self, var: &Var) -> bool {
        match self {
            Self::Sum(terms) | Self::Product(terms) => {
                let mut is_variant = false;

                for term in terms {
                    is_variant |= term.is_variant_on(var)
                }

                is_variant
            }

            Self::Var(checking_var) => var == checking_var.as_ref(),

            Self::Const(..) => false,

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

    /// Checks if all of the operands are constants
    pub fn constant_operands(terms: &Vec<Expr>) -> bool {
        let mut const_operands = true;

        for term in terms.iter() {
            if let Expr::Const(_) = term {
                ()
            } else {
                const_operands = false;
                break;
            }
        }

        const_operands
    }

    /// Takes partial derivative of the expression with respect to `var`. 
    pub fn derivative(&self, var: &Var) -> Expr {
        match self {
            Expr::Sum(terms) => {
                if Self::constant_operands(terms) {
                    Expr::Const(Num::Zero)
                } else {
                    // (a + b + c)' = a' + b' + c'
                    Expr::Sum(terms.into_iter().map(|v| v.derivative(var)).collect())
                }
            }

            Expr::Product(terms) => {
                if Self::constant_operands(&terms) {
                    return Expr::Const(Num::Zero);
                }

                // (abc)' = a'bc + ab'c + abc'

                let mut new_terms = vec![terms.clone(); terms.len()];

                for i in 0..terms.len() {
                    new_terms[i][i] = new_terms[i][i].derivative(var);
                }

                dbg!(&new_terms);

                Expr::Sum(
                    new_terms
                        .into_iter()
                        .map(|v| Expr::Product(v))
                        .collect(),
                )

            }

            Expr::Var(expr_var) => {
                // dx/dx = 1, dy/dx = 0, assuming x and y are both independent vars
                if expr_var.as_ref() == var {
                    Expr::Const(Num::One)
                } else {
                    Expr::Const(Num::Zero)
                }
            }

            Expr::Const(_) => {
                // (c)' = 0
                Expr::Const(Num::Zero)
            }

            Expr::Function(_, _) => todo!(),
        }
            // .simplify_trivial_single_layer()
    }

    /** Handle simple simplification identities involving single terms. */
    pub fn simplify_trivial(&self) -> Expr {
        match self {
            Expr::Sum(terms) => {
                Expr::Sum(terms.into_iter().map(|v| v.simplify_trivial()).collect())
            }
            Expr::Product(terms) => {
                Expr::Product(terms.into_iter().map(|v| v.simplify_trivial()).collect())
            }
            Expr::Function(def, arg_terms) => Expr::Function(
                def.clone(),
                arg_terms
                    .into_iter()
                    .map(|v| v.simplify_trivial())
                    .collect(),
            ),
            _ => self.clone(),
        }
        .simplify_trivial_single_layer()
    }

    /// Documentation: TODO
    pub fn simplify_trivial_single_layer(&self) -> Expr {
        match self {
            Expr::Sum(terms) => {
                // Combines Sum of Sums into a single Sum.
                let result: Vec<Expr> = terms
                    .clone()
                    .into_iter()
                    .flat_map(|operand| if let Expr::Sum(subterms) = operand {
                        subterms.into_iter()
                    } else {
                        vec![operand].into_iter()
                    })
                    .filter(|value| if let Expr::Const(n) = value {
                        !n.is_zero()
                    } else {
                        true
                    })
                    .collect();

                // todo!();

                // let terms_out: Vec<Expr> = terms
                //     .clone()
                //     .into_iter()
                //     .flat_map(|v| match v {
                //         Expr::Sum(subterms) => subterms,
                //         _ => vec![v],
                //     })
                //     .filter(|v| match v {
                //         Expr::Const(n) => !n.is_zero(),
                //         _ => true,
                //     })
                //     .collect();

                if result.len() == 0 {
                    Expr::Const(Num::Zero)
                } else if result.len() == 1 {
                    result[0].clone()
                } else {
                    Expr::Sum(result)
                }
            }

            Expr::Product(terms) => {
                // let terms_out: Vec<Expr> = terms
                //     .clone()
                //     .into_iter()
                //     .flat_map(|v| match v {
                //         Expr::Product(subterms) => subterms,
                //         _ => vec![v],
                //     })
                //     .filter(|v| match v {
                //         Expr::Const(n) => !n.is_one(),
                //         _ => true,
                //     })
                //     .collect();

                // Combines Product of Products into a single Product.
                let result: Vec<Expr> = terms
                    .clone()
                    .into_iter()
                    .flat_map(|operand| if let Expr::Product(subterms) = operand {
                        subterms.into_iter()
                    } else {
                        vec![operand].into_iter()
                    })
                    .filter(|value| if let Expr::Const(n) = value {
                        !n.is_one()
                    } else {
                        true
                    })
                    .collect();

                // Checks if there are any zeros.
                let any_zeros = result
                    .iter()
                    .filter(|operand| if let Expr::Const(n) = operand {
                        n.is_zero()
                    } else {
                        false
                    })
                    .count() > 0;

                if any_zeros {
                    Expr::Const(Num::Zero)
                } else if result.len() == 1 {
                    result[0].clone()
                } else if result.len() == 0 {
                    Expr::Const(Num::Zero)
                } else {
                    Expr::Product(result)
                }
            }

            Expr::Var(expr_var) => Expr::Var(expr_var.clone()),
            
            Expr::Const(num) => {
                // (c)' = 0
                Expr::Const(num.reduce())
            }
            
            Expr::Function(def, args) => Expr::Function(def.clone(), args.clone()),
        }
    }

    /// Documentation: TODO
    pub fn exact_match(&self, other: &Expr) -> bool {
        match self {
            Expr::Sum(terms_self) => match other {
                Expr::Sum(terms_other) => {
                    if terms_self.len() != terms_other.len() {
                        return false;
                    }
                    let mut terms_to_match = terms_other.clone();
                    for term in terms_self {
                        let mut found = None;
                        for i in 0..terms_to_match.len() {
                            if Expr::exact_match(term, &terms_to_match[i]) {
                                found = Some(i);
                                break;
                            }
                        }
                        if let Some(i) = found {
                            terms_to_match.drain(i..i + 1);
                        } else {
                            return false;
                        }
                    }
                    true
                }
                _ => false,
            }
            
            Expr::Product(terms_self) => match other {
                Expr::Product(terms_other) => {
                    if terms_self.len() != terms_other.len() {
                        return false;
                    }
                    let mut terms_to_match = terms_other.clone();
                    for term in terms_self {
                        let mut found = None;
                        for i in 0..terms_to_match.len() {
                            if Expr::exact_match(term, &terms_to_match[i]) {
                                found = Some(i);
                                break;
                            }
                        }
                        if let Some(i) = found {
                            terms_to_match.drain(i..i + 1);
                        } else {
                            return false;
                        }
                    }
                    true
                }
                _ => false,
            }

            Expr::Var(var_self) => match other {
                Expr::Var(var_other) => var_self == var_other,
                _ => false,
            }

            Expr::Const(num_self) => match other {
                Expr::Const(num_other) => num_self == num_other,
                _ => false,
            }

            Expr::Function(_, _) => todo!(),
        }
    }
}

impl Add for Expr {
    type Output = Expr;

    fn add(self, rhs: Self) -> Self::Output {
        Expr::Sum(vec![self.clone(), rhs.clone()])
    }
}

impl Add<Vec<Expr>> for Expr {
    type Output = Expr;

    fn add(self, rhs: Vec<Expr>) -> Self::Output {
        let mut args = vec![self];
        args.extend(rhs.into_iter());

        Expr::Sum(args)
    }
}

impl Sub for Expr {
    type Output = Expr;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Sum(vec![
            self,
            Self::Product(vec![Self::Const(Num::from(-1)), rhs]),
        ])
    }
}

impl Sub<Vec<Expr>> for Expr {
    type Output = Expr;

    fn sub(self, rhs: Vec<Expr>) -> Self::Output {
        Self::Sum(vec![
            self,
            Self::Product(vec![
                Self::Const(Num::from(-1)),
                Self::Sum(rhs)
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

impl Mul<Vec<Expr>> for Expr {
    type Output = Expr;

    fn mul(self, rhs: Vec<Expr>) -> Self::Output {
        let mut args = vec![self];
        args.extend(rhs.into_iter());

        Self::Product(args)
    }
}

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

            Expr::Var(var) => write!(f, "\u{001b}[95m{}", var.get_name()), // Remove colors before production

            Expr::Const(num) => write!(f, "\u{001b}[94m{}", num),

            Expr::Function(_, _) => todo!(),
        }?;
        write!(f, "\u{001b}[0m")
    }
}
