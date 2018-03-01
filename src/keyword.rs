macro_rules! def_keywords {
    {$($name: ident: $keyword: expr,)+} => {
        $(pub const $name: &'static str = $keyword;)+
    }
}

def_keywords! {
    OPENQASM:    "OPENQASM",
    INCLUDE:     "include",
    QREG:        "qreg",
    CREG:        "creg",
    BARRIER:     "barrier",
    GATE:        "gate",
    IF:          "if",
    MEASURE:     "measure",
    OPAQUE:      "opaque",
    RESET:       "reset",
    // builtin const
    PI:          "pi",
    // builtin gates
    U:           "U",
    CX:          "CX",
    // builtin unary op
    SIN:         "sin",
    COS:         "cos",
    TAN:         "tan",
    EXP:         "exp",
    LN:          "ln",
    SQRT:        "sqrt",
    // punctuations
    PLUS:        "+",
    MINUS:       "-",
    TIMES:       "*",
    DEVIDE:      "/",
    POWER:       "^",
    COMMA:       ",",
    SEMICOLON:   ";",
    DOUBLEEQUAL: "==",
    ARROW:       "->",
    // brackets
    LPAREN:      "(",
    RPAREN:      ")",
    LBRACE:      "{",
    RBRACE:      "}",
    LSQBRACKET:  "[",
    RSQBRACKET:  "]",
}
