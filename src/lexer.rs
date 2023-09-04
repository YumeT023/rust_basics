#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum TokenKind {
  Eof = 0,
  Symbol,
  LCurly,
  RCurly,
  Coma,
  Whitespace
}

#[derive(Debug, Clone, Copy)]
// here, lifetime 'a means, if the token is dropped (cleaned from the stack), the 'str_val' will
// also be dropped
pub struct Token<'a> {
  pub kind: TokenKind,
  pub str_val: &'a str,
}

impl<'a> Token<'a> {
  pub fn new(kind: TokenKind, val: &'a str) -> Self {
    Token {
      kind,
      str_val: val,
    }
  }
}
