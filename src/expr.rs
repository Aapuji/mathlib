use num_complex::Complex64;
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::ops::{Add, Mul, Sub};
use std::sync::Arc;

use crate::{function::FuncDef, num::Num, var::Var};

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

    pub fn derivative(&self, var: &Var) -> Expr {
        match self {
            Expr::Sum(terms) => {
                let mut const_operands = true;
                for term in terms.iter() {
                    if let Expr::Const(_) = term {
                        ()
                    } else {
                        const_operands = false;
                        break;
                    }
                }
                if const_operands {
                    return Expr::Const(Num::Zero);
                }

                // (a + b + c)' = a' + b' + c'
                Expr::Sum(terms.into_iter().map(|v| v.derivative(var)).collect())
            }

            Expr::Product(terms) => {
                let mut const_operands = true;
                for term in terms.iter() {
                    if let Expr::Const(_) = term {
                        ()
                    } else {
                        const_operands = false;
                        break;
                    }
                }
                if const_operands {
                    return Expr::Const(Num::Zero);
                }

                // (abc)' = a'bc + ab'c + abc'
                let mut new_terms = vec![terms.clone(); terms.len()];

                for i in 0..terms.len() {
                    new_terms[i][i] = new_terms[i][i].derivative(var);
                }

                Expr::Sum(
                    new_terms
                        .into_iter()
                        .map(|v| Expr::Product(v).simplify_trivial_single_layer())
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
        .simplify_trivial_single_layer()
    }

    /** Handle simple simplification identities involing single terms. */
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
    pub fn simplify_trivial_single_layer(&self) -> Expr {
        match self {
            Expr::Sum(terms) => {
                let terms_out: Vec<Expr> = terms
                    .clone()
                    .into_iter()
                    .flat_map(|v| match v {
                        Expr::Sum(subterms) => subterms,
                        _ => vec![v],
                    })
                    .filter(|v| match v {
                        Expr::Const(n) => !n.is_zero(),
                        _ => true,
                    })
                    .collect();

                if terms_out.len() == 0 {
                    Expr::Const(Num::Zero)
                } else if terms_out.len() == 1 {
                    terms_out[0].clone()
                } else {
                    Expr::Sum(terms_out)
                }
            }
            Expr::Product(terms) => {
                let terms_out: Vec<Expr> = terms
                    .clone()
                    .into_iter()
                    .flat_map(|v| match v {
                        Expr::Product(subterms) => subterms,
                        _ => vec![v],
                    })
                    .filter(|v| match v {
                        Expr::Const(n) => !n.is_one(),
                        _ => true,
                    })
                    .collect();

                let any_zeros = terms
                    .into_iter()
                    .filter(|v| match v {
                        Expr::Const(n) => n.is_zero(),
                        _ => false,
                    })
                    .count()
                    > 0;

                if any_zeros {
                    Expr::Const(Num::Zero)
                } else if terms_out.len() == 1 {
                    terms_out[0].clone()
                } else if terms_out.len() == 0 {
                    Expr::Const(Num::Zero)
                } else {
                    Expr::Product(terms_out)
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
            },
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
            },
            Expr::Var(var_self) => match other {
                Expr::Var(var_other) => var_self == var_other,
                _ => false,
            },
            Expr::Const(num_self) => match other {
                Expr::Const(num_other) => num_self == num_other,
                _ => false,
            },
            Expr::Function(_, _) => todo!(),
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
            Self::Product(vec![Self::Const(Num::int(-1)), rhs]),
        ])
    }
}

impl Mul for Expr {
    type Output = Expr;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::Product(vec![self, rhs])
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
            }
            Expr::Product(terms) => {
                let mut k: Vec<String> = vec![];
                for term in terms {
                    k.push(format!("{}", term));
                }
                write!(f, "({})", k.join(" * "))
            }
            Expr::Var(var) => write!(f, "\u{001b}[95m{}", var.get_name()),
            Expr::Const(num) => write!(f, "\u{001b}[94m{}", num),
            Expr::Function(_, _) => todo!(),
        }?;
        write!(f, "\u{001b}[0m")
    }
}
