use std::{sync::Arc, fmt::{Display, write}, collections::HashMap};
use num_complex::Complex64;

use crate::{expr::{Expr, ArcExpr}, var::Var};

#[derive(Debug)]
pub struct Function<E>
where
    E: Expr
{
    name: String,
    args: Vec<Arc<Var>>,
    expr: Arc<E>
}

impl<E> Function<E>
where 
    E: Expr
{
    pub fn new(name: String, args: Vec<Arc<Var>>, expr: Arc<E>) -> Self {
        Self {
            name,
            args,
            expr
        }
    }
}

impl<E> Display for Function<E>
where
    E: Expr
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}(", self.name)?;

        for (i, arg) in self.args[1..].into_iter().enumerate() {
            write!(f, "{},", arg)?;

            if i < self.args.len() - 1 {
                write!(f, " ")?;
            }
        }

        write!(f, ") = {}", self.expr)?;

        Ok(())
    }
}

impl<E> Expr for Function<E>
where
    E: Expr
{
    fn eval(&self, var_values: &HashMap<&Var, Complex64>) -> crate::expr::EvalResult {
        self.expr.eval(var_values)
    }

    fn is_variant_on(&self, var: Arc<Var>) -> bool {
        todo!()
    }
}