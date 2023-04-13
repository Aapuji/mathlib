use std::{sync::Arc, fmt::{Debug, Display}};

use super::var::Var;

pub mod add;


pub trait Expr : Debug + Display {
    fn is_variant_on(&self, var: Arc<Var>) -> bool;
}

pub struct ArcExpr(pub Arc<dyn Expr>);
impl Clone for ArcExpr {
    fn clone(&self) -> Self {
        ArcExpr(self.0.clone())
    }
}

