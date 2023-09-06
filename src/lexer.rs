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
pub struct Span {
  pub start: usize,
  pub end: usize,
}

impl Default for Span {
  fn default() -> Self {
    Span {
      start: 0,
      end: 0,
    }
  }
}

impl Span {
  pub fn new(start: usize, end: usize) -> Self {
    Span {
      start,
      end,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq)]
// here, lifetime 'a means, if the token is dropped (cleaned from the stack), the 'str_val' will
// also be dropped
pub struct Token<'a> {
  pub kind: TokenKind,
  pub str_val: &'a str,
  pub span: Span,
}

impl<'a> Token<'a> {
  pub fn new(kind: TokenKind, val: &'a str, span: Span) -> Self {
    Token {
      kind,
      str_val: val,
      span,
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

  pub fn next_tok(&mut self) -> Option<Token<'a>> {
    // Ensures there is no current_tok yet
    self.cur.reset_current_tok();
    self.cur.stretch();
    let kind = match self.cur.current_tok_char() {
      Some(c) => match c {
        '{' => TokenKind::LCurly,
        '}' => TokenKind::RCurly,
        ',' => TokenKind::Coma,
        c if c.is_whitespace() => return self.next_tok(), // ws : _ -> skip
        c if is_symbol_start(c) => self.symbol(),
        _ => TokenKind::Eof
      },
      _ => return None
    };
    let (span, str_val) = self.cur.current_tok_span();
    Some(Token::new(kind, str_val, span))
  }

  pub fn symbol(&mut self) -> TokenKind {
    self.cur.stretch_while(|c| is_symbol_continue(c));
    TokenKind::Symbol
  }
}

pub fn tokenize(source: &str) -> Vec<Token> {
  let mut tokens = vec![];
  let mut lexer = Lexer::new(source);

  while let Some(t) = lexer.next_tok() {
    tokens.push(t);
    if t.kind == TokenKind::Eof {
      break;
    }
  }
  tokens
}

pub fn is_symbol_start(c: char) -> bool {
  c.is_alphabetic() || c == '_'
}

pub fn is_symbol_continue(c: char) -> bool {
  c.is_alphanumeric() || c == '_'
}
