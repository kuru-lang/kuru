pub(super) struct TokenString(pub(super) [u8; 32]);

impl std::fmt::Debug for TokenString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.0).unwrap())
    }
}

#[derive(Debug)]
pub(super) enum Token {
    // An ascii identifier
    Ident(TokenString),
    // An integer
    Int(u128),
    // A decimal
    Dec(u128, u128),
    // A text literal
    Text(String),

    // 1-character tokens
    Access,    // .
    Ref,       // @
    Separator, // , \n
    Assign,    // :
    Paren(bool),
    Bracket(bool),
    Brace(bool),

    //
    Eval, // ->

    // And tokens
    And,          // &
    AndAssign,    // &:
    AndSet,       // &&
    AndSetAssign, // &&:

    // Inclusive Or tokens
    Ior,          // |
    IorAssign,    // |:
    IorSet,       // ||
    IorSetAssign, // ||:

    // Exclusive Or tokens
    Xor,          // ~
    XorAssign,    // ~:
    XorSet,       // ~~
    XorSetAssign, // ~~:

    // Addition tokens
    Add,          // +
    AddAssign,    // +:
    Concat,       // ++
    ConcatAssign, // ++:

    // Subtraction tokens
    Sub,         // -
    SubAssign,   // -:
    Trunc,       // --
    TruncAssign, // --:

    // Multiplication tokens
    Mul,              // *
    MulAssign,        // *:
    DotProduct,       // **
    DotProductAssign, // **:

    // Division tokens
    Div,          // /
    DivAssign,    // /:
    DivInt,       // //
    DivIntAssign, // //:
}
