#![feature(box_syntax, box_patterns)]

#[derive(Clone,Debug)]
pub enum Expr {
    Val(f64),
    Var(char),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}


impl Expr {

    pub fn diff(self, var : char) -> Expr {
        match self {

            Expr::Val(_)             => Expr::Val(0.0),
            Expr::Var(x) if x == var => Expr::Val(1.0),
            Expr::Var(_)             => Expr::Val(0.0),
            Expr::Add(box a, box b) => Expr::Add(box a.diff(var), box b.diff(var)),
            Expr::Mul(box a, box b) => Expr::Add(box Expr::Mul(box a.clone().diff(var), box b.clone()), box Expr::Mul(box a, box b.diff(var))),

        }
    }


    pub fn simp(self) -> Expr {
        match self {

            Expr::Add(box Expr::Val(a), box Expr::Val(b))   => Expr::Val(a + b),
            Expr::Add(box Expr::Val a, box b) if a == 0.0   => b,
            Expr::Add(box b, box Expr::Val a) if a == 0.0   => b,

            Expr::Mul(box Expr::Val(a), box Expr::Val(b))   => Expr::Val(a * b),
            Expr::Mul(box Expr::Val a, _)     if a == 0.0   => Expr::Val(0.0),
            Expr::Mul(_, box Expr::Val)       if a == 0.0   => Expr::Val(0.0),
            Expr::Mul(box Expr::Val a, box b) if a == 1.0   => b,
            Expr::Mul(box b, box Expr::Val a) if a == 1.0   => b,

            e => e
        }
    }

}
