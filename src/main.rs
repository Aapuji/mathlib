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

    // a + 2ba(2+e)
    let s = a.clone() + two.clone() + b.clone() * a.clone() * (two.clone() + e.clone());

    // 0a + 1b + 0
    let k = a.clone() * Expr::Const(Num::zero()) + b.clone() * Expr::Const(Num::one()) + Expr::Const(Num::zero());

    // 2a + e + aa + b
    let z = a.clone() * Expr::Const(Num::I).clone() + e.clone() + a.clone() * a.clone() + b.clone();
    let dz_da = z.derivative(a_var.as_ref()).simplify_trivial();

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
    println!("k: {}, ks: {}", k, k.simplify_trivial());
    println!("a: {}, b: {}", a_var, b_var);
    println!("z: {}, dz_da: {}", z, dz_da);
    println!("sum: {}", s.simplify_trivial());
    println!("sum eval: {}", s.eval(&v).unwrap());
    println!("infinity: {}", inf);
    println!();
}
