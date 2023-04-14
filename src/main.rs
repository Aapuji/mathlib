use std::sync::Arc;
use mathlib::{expr::{ArcExpr, constant::ConstExpr}, var::Var, num::Num, arcify_expr};

fn main() {
    let (a_var,a) = Var::new("a").wrap();
    let (b_var,b) = Var::new("b").wrap();
    let two = arcify_expr!(ConstExpr::new(Num::Rational { num: 2, den: 1 }));
    let e = arcify_expr!(ConstExpr::new(Num::E));


    let s_r = a.clone() + two.clone() + b.clone() * a.clone() * (two.clone() + e.clone());

    let s = s_r.0;

    println!();
    println!("{},{}", a_var, b_var);
    println!("{}",s);
    println!("hello world");
}
