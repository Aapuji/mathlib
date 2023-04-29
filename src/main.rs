use std::collections::HashMap;

use mathlib::expr::Expr;
use mathlib::num::Num;
use mathlib::var::Var;
use num_complex::Complex64;

fn main() {
    let x_var = Var::new("x");
    let y_var = Var::new("y");

    let x = Expr::Var(x_var.clone());
    let y = Expr::Var(y_var.clone());
    let two = Expr::Const(Num::int(2));
    let e = Expr::Const(Num::E);
    let inf = Expr::Const(Num::Infinity);

    // x + 2 + x*y*(2+e)
    let f = x.clone() + two.clone() + x.clone() * y.clone() * (two.clone() + e.clone());

    // 0*x + 1*y + 0
    let g = x.clone() * Expr::Const(Num::Zero)
        + y.clone() * Expr::Const(Num::One)
        + Expr::Const(Num::Zero);

    // x*i + e + x*x*e + y
    let z = x.clone() * Expr::Const(Num::I).clone()
        + e.clone()
        + x.clone() * x.clone() * Expr::Const(Num::E)
        + y.clone();
    let dz_dx = z.simplify_trivial().derivative(x_var.as_ref()); //.simplify_trivial();

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
    println!("{x} = {}", v.get(x_var.as_ref()).unwrap());
    println!("{y} = {}", v.get(y_var.as_ref()).unwrap());
    println!();
    println!("f = {} = {}", f.simplify_trivial(), f.eval(&v).unwrap());
    println!("k = {} = {}", g, g.simplify_trivial());
    println!("z = {}, dz/dx = {}", z, dz_dx);
    println!(
        "g = {}, dg/dx = {}",
        g.simplify_trivial(),
        g.derivative(x_var.as_ref())
    );
    println!();
    println!("undefined: {}", Expr::Const(Num::rational(1, 0)));
    println!("infinity: {}", inf);
    println!();
}
