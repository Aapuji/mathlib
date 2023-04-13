use std::sync::Arc;

use mathlib::{expr::ArcExpr, var::Var};

fn main() {
    let a = Arc::new(Var::new("a"));

    let a_r = ArcExpr(a);

    let s_r = &a_r + &a_r;

    let s = s_r.0;

    println!("{}",s);
    println!("\n\u{001b}[42m\u{001b}[30m :: hewwo world :: \u{001b}[0m\n");
}
