use std::str::Chars;

#[derive(Debug)]
pub struct Cursor<'a> {
  pub chars: Chars<'a>,
  pub source: &'a str,
  pub c0: usize,
  pub c1: usize,
}

impl<'a> PartialEq<Self> for Cursor<'a> {
  fn eq(&self, other: &Self) -> bool {
    self.source == other.source
      && self.chars.as_str() == other.chars.as_str()
      && self.c1 == other.c1
      && self.c0 == other.c0
  }
}

impl<'a> Cursor<'a> {
  pub fn new(source: &'a str) -> Self {
    Cursor {
      source,
      chars: source.chars(),
      // Double cursor that will be used to delimit a token
      c0: 0,
      c1: 0,
    }
  }

  pub fn stretch(&mut self) {
    self.c1 += 1;
  }

  pub fn stretch_while(&mut self, f: impl Fn(char) -> bool) {
    let init_len = self.chars.as_str().len();
    let mut chars = self.chars.clone();
    while chars.next().is_some_and(|c| f(c)) { self.chars.next(); }
    self.c1 += init_len - self.chars.as_str().len();
  }

  pub fn done_current_tok(&mut self) {
    self.c0 = self.c1;
  }

  pub fn current_tok_len(&self) -> u32 {
    (self.c1 - self.c0) as u32
  }

  pub fn current_tok_span(&self) -> (usize, &str, usize) {
    (self.c0, self.current_tok_val(), self.c1)
  }

  pub fn current_tok_val(&self) -> &str {
    &self.source[self.c0..self.c0 + self.current_tok_len() as usize]
  }

  pub fn is_eof(&self) -> bool {
    self.c1 >= self.source.len()
  }
}
