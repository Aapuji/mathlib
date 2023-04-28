use std::collections::HashMap;

use mathlib::expr::Expr;
use mathlib::var::Var;
use mathlib::num::Num;
use num_complex::Complex64;

fn main() {
    let x_var = Var::new("a");
    let y_var = Var::new("b");

    let x = Expr::Var(x_var.clone());
    let y = Expr::Var(y_var.clone());
    let two = Expr::Const(Num::Rational { num: 2, den: 1 });
    let e = Expr::Const(Num::E);
    let inf = Expr::Const(Num::Infinity);

    // x + 2 + x*y*(2+e)
    let f = x.clone() + two.clone() + x.clone() * y.clone() * (two.clone() + e.clone());

    // 0*x + 1*y + 0
    let g = x.clone() * Expr::Const(Num::Zero) + y.clone() * Expr::Const(Num::One) + Expr::Const(Num::Zero);

    // x*i + e + x*x + y
    let z = x.clone() * Expr::Const(Num::I).clone() + e.clone() + x.clone() * x.clone() + y.clone();
    let dz_da = z.derivative(x_var.as_ref()).simplify_trivial();

    // let x = Var::new("x");
    // let f = Function::new(
    //     String::from("f"),
    //     vec![x.clone()],
    //     AddExpr::new(vec![Arc::new(1), Arc::new(2)])
    // );

    let mut v = HashMap::new();
    v.insert(x_var.as_ref(), Complex64::new(1.0, 2.0));
    v.insert(y_var.as_ref(), Complex64::new(-1.0, 0.0));

    println!();
    println!("k: {}, ks: {}", g, g.simplify_trivial());
    println!("a: {}, b: {}", x_var, y_var);
    println!("z: {}, dz_da: {}", z, dz_da);
    println!("sum: {}", f.simplify_trivial());
    println!("sum eval: {}", f.eval(&v).unwrap());
    println!("infinity: {}", inf);
    println!("\n\ndg/dx: {}", g.derivative(x_var.as_ref()));
}
