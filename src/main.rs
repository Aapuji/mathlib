use std::sync::Arc;
use mathlib::expr::{Expr, ArcExpr, constant::ConstExpr};
use mathlib::var::Var;
use mathlib::num::Num;
use mathlib::function::Function;
use mathlib::arcify_expr;

fn main() {
    let (a_var, a) = Var::new("a").wrap();
    let (b_var, b) = Var::new("b").wrap();
    let two = arcify_expr!(ConstExpr::new(Num::Rational { num: 2, den: 1 }));
    let e = arcify_expr!(ConstExpr::new(Num::E));
    let inf = arcify_expr!(ConstExpr::new(Num::Infinity));


    let s_r = a.clone() + two.clone() + b.clone() * a.clone() * (two.clone() + e.clone());

    let s = s_r.0;

    // let x = Var::new("x");
    // let f = Function::new(
    //     String::from("f"),
    //     vec![x.clone()],
    //     AddExpr::new(vec![Arc::new(1), Arc::new(2)])
    // );

    println!();
    println!("{},{}", a_var, b_var);
    println!("{}",s);
    println!("{}", Arc::<dyn Expr>::from(inf));
    println!("hello world");
}
