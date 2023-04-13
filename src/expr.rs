use std::{sync::Arc, fmt::{Debug, Display}};

pub trait Expr : Debug + Display {
    fn is_variant_on(&self, var: Arc<Var>) -> bool;
}

pub struct ArcExpr(pub Arc<dyn Expr>);
impl Clone for ArcExpr {
    fn clone(&self) -> Self {
        ArcExpr(self.0.clone())
    }
}

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

/** Independent variable, unknown  */
#[derive(PartialEq, Debug)]
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
        write!(f, "TODO")
    }
}

impl Expr for Var {
    fn is_variant_on(&self, var: Arc<Var>) -> bool {
        self == var.as_ref()
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
