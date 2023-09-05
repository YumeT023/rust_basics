use crate::cursor::Cursor;

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum TokenKind {
  Eof = 0,
  Symbol,
  LCurly,
  RCurly,
  Coma,
  Whitespace,
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug)]
pub struct Lexer<'a> {
  pub cur: Cursor<'a>,
}

impl<'a> Lexer<'a> {
  pub fn new(source: &'a str) -> Self {
    Lexer {
      cur: Cursor::new(source)
    }
  }

  pub fn lex(&mut self) -> Option<Token> {
    // Ensures there is no current_tok yet
    self.cur.reset_current_tok();
    self.cur.stretch();
    let kind = match self.cur.current_tok_char() {
      Some(c) => match c {
        '{' => TokenKind::LCurly,
        '}' => TokenKind::RCurly,
        ',' => TokenKind::Coma,
        c if is_symbol_start(c) => self.symbol(),
        _ => return None
      },
      _ => TokenKind::Eof
    };
    let t = Token::new(kind, self.cur.current_tok_val());
    Some(t)
  }

  pub fn symbol(&mut self) -> TokenKind {
    self.cur.stretch_while(|c| is_symbol_continue(c));
    TokenKind::Symbol
  }
}

pub fn is_symbol_start(c: char) -> bool {
  c.is_alphabetic() || c == '_'
}

pub fn is_symbol_continue(c: char) -> bool {
  c.is_alphanumeric() || c == '_'
}
