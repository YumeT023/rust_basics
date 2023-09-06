use std::fmt::Debug;

use crate::lexer::{Span, Token, TokenKind};

pub fn symbol(name: &'static str, span: Span) -> Token<'static> {
  Token::new(TokenKind::Symbol, name, span)
}

pub fn l_curly(span: Span) -> Token<'static> {
  Token::new(TokenKind::LCurly, "{", span)
}

pub fn r_curly(span: Span) -> Token<'static> {
  Token::new(TokenKind::RCurly, "}", span)
}

pub fn coma(span: Span) -> Token<'static> {
  Token::new(TokenKind::Coma, ",", span)
}

pub fn assert_eq_opt<T: PartialEq + Debug>(left: Option<T>, right: T) {
  assert!(left.is_some_and(|t| t == right))
}
