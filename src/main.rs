use std::sync::Arc;
use mathlib::{expr::{ArcExpr, constant::ConstExpr}, var::Var, num::Num};

fn main() {
    let a = Arc::new(Var::new("a"));
    let b = Arc::new(Var::new("b"));
    let two = ArcExpr(Arc::new(ConstExpr::new(Num::Rational { num: 2, den: 1 })));
    let e = ArcExpr(Arc::new(ConstExpr::new(Num::E)));

    let a_r = ArcExpr(a.clone());
    let b_r = ArcExpr(b.clone());

    let s_r = a_r.clone() + two.clone() + b_r.clone() * a_r.clone() * (two.clone() + e.clone());

    let s = s_r.0;

    println!();
    println!("{}",s);
    println!("hello world");
}
