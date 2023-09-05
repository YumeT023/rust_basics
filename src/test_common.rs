use crate::lexer::{Token, TokenKind};

pub fn symbol(name: &'static str) -> Token<'static> {
  Token::new(TokenKind::Symbol, name)
}

pub fn l_curly() -> Token<'static> {
  Token::new(TokenKind::LCurly, "{")
}

pub fn r_curly() -> Token<'static> {
  Token::new(TokenKind::RCurly, "}")
}

pub fn coma() -> Token<'static> {
  Token::new(TokenKind::Coma, ",")
}

pub fn assert_eq_opt<T: PartialEq>(left: Option<T>, right: T) {
  assert!(left.is_some_and(|t| t == right))
}
