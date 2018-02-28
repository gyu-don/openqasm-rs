use keyword;

pub struct Token<'a> {
    pub token: TokenType,
    pub filename: Option<&'a str>,
    pub position: (usize, usize),
}

pub enum TokenType {
    Real(f64),
    UInt(usize),
    Idenfier(String),
    StringLiteral(String),
    Comment(String),
    // keywords
    Openqasm,
    Include,
    Qreg,
    Creg,
    Barrier,
    Gate,
    If,
    Measure,
    Opaque,
    Reset,
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
    Arrow,
    // brackets
    LParen,
    RParen,
    LBrace,
    RBrace,
    LSqBracket,
    RSqBracket,
}

pub fn match_keyword_exact(s: &str) -> Option<TokenType> {
    macro_rules! match_str_some {
        ($s: expr, { $($key: expr => $value: expr,)* }) => {
            $(if $s == $key { return Some($value); })*
            return None;
        }
    }
    match_str_some! (s, {
        keyword::OPENQASM    => TokenType::Openqasm,
        keyword::INCLUDE     => TokenType::Include,
        keyword::QREG        => TokenType::Qreg,
        keyword::CREG        => TokenType::Creg,
        keyword::BARRIER     => TokenType::Barrier,
        keyword::GATE        => TokenType::Gate,
        keyword::IF          => TokenType::If,
        keyword::MEASURE     => TokenType::Measure,
        keyword::OPAQUE      => TokenType::Opaque,
        keyword::RESET       => TokenType::Reset,
        keyword::PI          => TokenType::Pi,
        keyword::U           => TokenType::U,
        keyword::CX          => TokenType::CX,
        keyword::SIN         => TokenType::Sin,
        keyword::COS         => TokenType::Cos,
        keyword::TAN         => TokenType::Tan,
        keyword::EXP         => TokenType::Exp,
        keyword::LN          => TokenType::Ln,
        keyword::SQRT        => TokenType::Sqrt,
        keyword::PLUS        => TokenType::Plus,
        keyword::MINUS       => TokenType::Minus,
        keyword::TIMES       => TokenType::Times,
        keyword::DEVIDE      => TokenType::Devide,
        keyword::POWER       => TokenType::Power,
        keyword::COMMA       => TokenType::Comma,
        keyword::SEMICOLON   => TokenType::Semicolon,
        keyword::DOUBLEEQUAL => TokenType::DoubleEqual,
        keyword::ARROW       => TokenType::Arrow,
        keyword::LPAREN      => TokenType::LParen,
        keyword::RPAREN      => TokenType::RParen,
        keyword::LBRACE      => TokenType::LBrace,
        keyword::RBRACE      => TokenType::RBrace,
        keyword::LSQBRACKET  => TokenType::LSqBracket,
        keyword::RSQBRACKET  => TokenType::RSqBracket,
    });
}
