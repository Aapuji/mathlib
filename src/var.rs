use std::{fmt::Display, sync::Arc};

/** Independent variable, unknown  */
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Var {
    name: String,
}

impl Var {
    pub fn new(name: &str) -> Arc<Self> {
        Arc::new(Self::new_owned(name))
    }
    pub fn new_owned(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VAR[{}]", self.name)
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
