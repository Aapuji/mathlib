use std::collections::HashMap;

use mathlib::expr::Expr;
use mathlib::num::Num;
use mathlib::var::Var;
use num_complex::Complex64;

fn main() {
    test_2();
}

fn test_2() {
    let x_var = Var::new("x");
    let y_var = Var::new("y");

    let x = Expr::Var(x_var.clone());
    let y = Expr::Var(y_var.clone());

    let f = x.clone() * Expr::Const(Num::rational(5, 1)) + x.clone() - y.clone() * Expr::Const(Num::rational(2, 1));

    println!("f = {},\nf simp = {},\ndf/dx = {},\ndf/dx simp = {}", 
        &f, 
        f.simplify_trivial_single_layer(), 
        f.derivative(x_var.as_ref()),
        f.derivative(x_var.as_ref()).simplify_trivial_single_layer()
    );

    println!("---");

    let g = y.clone() * Expr::Const(Num::One);
        // + Expr::Const(Num::int(5))
        // * y.clone();

    println!("g = {},\ng simp = {},\ndg/dy = {},\ndg/dy simp = {}", 
        &g, 
        g.simplify_trivial_single_layer(), 
        g.derivative(y_var.as_ref()),
        g.derivative(y_var.as_ref()).simplify_trivial_single_layer()
    );

}

fn test_1() {
    let x_var = Var::new("x");
    let y_var = Var::new("y");

    let x = Expr::Var(x_var.clone());
    let y = Expr::Var(y_var.clone());
    let two = Expr::Const(Num::from(2));
    let e = Expr::Const(Num::E);
    let inf = Expr::Const(Num::Infinity);

    // x + 2 + x*y*(2+e)
    let f = x.clone() + two.clone() + x.clone() * y.clone() * (two.clone() + e.clone());

    // 0*x + 1*y + 0
    let g = x.clone() * Expr::Const(Num::Zero)
        + y.clone() * Expr::Const(Num::One)
        + y.clone() * y.clone()
        + Expr::Const(Num::Zero);
        // + Expr::Const(Num::int(5))
        // * y.clone();

    // x*i + e + x*x*e + y
    let z = x.clone() * Expr::Const(Num::I)
        + e.clone()
        + x.clone() * x.clone() * Expr::Const(Num::E)
        + y.clone();
    let dz_dx = z.simplify_trivial().derivative(x_var.as_ref()); //.simplify_trivial();

    let mut ctx = HashMap::new();
    ctx.insert(x_var.as_ref(), Complex64::new(1.0, 2.0));
    ctx.insert(y_var.as_ref(), Complex64::new(-1.0, 0.0));

    println!();
    println!("{x} = {}", ctx.get(x_var.as_ref()).unwrap());
    println!("{y} = {}", ctx.get(y_var.as_ref()).unwrap());
    println!();
    println!("f = {} = {}", f.simplify_trivial(), f.eval(&ctx).unwrap());
    println!("k = {} = {}", g, g.simplify_trivial());
    println!("z = {}, dz/dx = {}", z, dz_dx);
    println!(
        "g = {}, dg/dx = {}, dg/dy = {}",
        g,
        g.derivative(x_var.as_ref()),
        g.derivative(y_var.as_ref())
    );
    println!();
    println!("undefined: {}", Expr::Const(Num::rational(1, 0)));
    println!("infinity: {}", inf);
    println!();
}
