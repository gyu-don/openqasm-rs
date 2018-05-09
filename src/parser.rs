use std::f64;

pub enum UnaryOp {
    Sin,
    Cos,
    Tan,
    Exp,
    Ln,
    Sqrt,
}

pub type OpenqasmVersion = f64;
pub type GateName = String;
pub type PathString = String;
pub type CommentString = Vec<u8>;
pub type NnInteger = u32;

pub type Params = Vec<Expr>;
pub type QArgs = Vec<QReg>;

pub type Program = Vec<Statement>;

pub enum Statement {
    Openqasm(OpenqasmVersion),
    Include(PathString),
    Comment(CommentString),
    If(CReg, NnInteger, Program),
    Measure(QReg, CReg),
    Opaque(GateName, Params, QArgs),
    Gate(GateName, Params, QArgs, Program),
    Reset(QReg),
    Barrier,
    Apply(GateName, Params, QArgs),
}

pub enum Predicate {
}

pub enum QReg {
}

pub enum CReg {
}

impl UnaryOp {
    pub fn apply(&self, v: f64) -> f64 {
        match *self {
            UnaryOp::Sin => v.sin(),
            UnaryOp::Cos => v.cos(),
            UnaryOp::Tan => v.tan(),
            UnaryOp::Exp => v.exp(),
            UnaryOp::Ln => v.ln(),
            UnaryOp::Sqrt => v.sqrt(),
        }
    }
}

pub enum Expr {
    Real(f64),
    NnInteger(usize),
    Pi,
    Id(String),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    UnaryOp(UnaryOp, Box<Expr>),
}

impl Expr {
    pub fn eval(&self) -> f64 {
        match *self {
            Expr::Real(v) => v,
            Expr::NnInteger(v) => v as f64,
            Expr::Pi => f64::consts::PI,
            Expr::Id(_) => unimplemented!(),
            Expr::Add(ref lhs, ref rhs) => lhs.eval() + rhs.eval(),
            Expr::Sub(ref lhs, ref rhs) => lhs.eval() - rhs.eval(),
            Expr::Mul(ref lhs, ref rhs) => lhs.eval() * rhs.eval(),
            Expr::Div(ref lhs, ref rhs) => lhs.eval() / rhs.eval(),
            Expr::Neg(ref v) => -v.eval(),
            Expr::Pow(ref lhs, ref rhs) => lhs.eval().powf(rhs.eval()),
            Expr::UnaryOp(ref op, ref v) => op.apply(v.eval()),
        }
    }
}
