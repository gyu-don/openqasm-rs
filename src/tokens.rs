struct Token {
    pub token: TokenType,
    pub filename: Option<&str>,
    pub position: (usize, usize),
}

enum TokenType {
    Real(f64),
    UInt(usize),
    Idenfier(String),
    StringLiteral(String),
    Comment(String),
    // keywords
    Openqasm,
    Qreg,
    Creg,
    If,
    Gate,
    Opaque,
    Barrier,
    Measure,
    Reset,
    Include,
    // builtin const
    Pi,
    // builtin gates
    U,
    CX,
    // builtin unary op
    Sin,
    Cos,
    Tan,
    Exp,
    Ln,
    Sqrt,
    // Marks
    Plus,
    Minus,
    Times,
    Devide,
    Power,
    Comma,
    Semicolon,
    DoubleEqual,
    // brackets
    LParen,
    RParen,
    LBrace,
    RBrace,
    LSqBracket,
    RSqBracket,
}
