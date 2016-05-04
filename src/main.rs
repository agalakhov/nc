#![feature(box_syntax, box_patterns)]

mod expr;

use expr::Expr;

fn main() {
    let x = Expr::Mul(box Expr::Val(42.0), box Expr::Var('a'));
    let b = x.diff('a').simp();
    println!("expr = {:?}", b);
}
