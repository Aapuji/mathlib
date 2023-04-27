use std::collections::HashMap;

use mathlib::expr::Expr;
use mathlib::var::Var;
use mathlib::num::Num;
use num_complex::Complex64;

fn main() {
    let a_var = Var::new("a");
    let b_var = Var::new("b");

    let a = Expr::Var(a_var.clone());
    let b = Expr::Var(b_var.clone());
    let two = Expr::Const(Num::Rational { num: 2, den: 1 });
    let e = Expr::Const(Num::E);
    let inf = Expr::Const(Num::Infinity);


    let s = a.clone() + two.clone() + b.clone() * a.clone() * (two.clone() + e.clone());

    // let x = Var::new("x");
    // let f = Function::new(
    //     String::from("f"),
    //     vec![x.clone()],
    //     AddExpr::new(vec![Arc::new(1), Arc::new(2)])
    // );

    let mut v = HashMap::new();
    v.insert(a_var.as_ref(), Complex64::new(1.0, 2.0));
    v.insert(b_var.as_ref(), Complex64::new(-1.0, 0.0));

    println!();
    println!("a: {}, b: {}", a_var, b_var);
    println!("sum: {}", s);
    println!("sum eval: {}", s.eval(&v).unwrap());
    println!("infinity: {}", inf);
}
