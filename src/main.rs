use std::sync::Arc;
use mathlib::{expr::ArcExpr, var::Var};

fn main() {
    let a = Arc::new(Var::new("a"));
    let b = Arc::new(Var::new("b"));

    let a_r = ArcExpr(a.clone());
    let b_r = ArcExpr(b.clone());

    let s_r = a_r.clone() + b_r.clone() * a_r.clone();

    let s = s_r.0;

    println!();
    println!("{}",s);
    println!("hello world");
}
