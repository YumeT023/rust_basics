#[derive(Debug, PartialEq, Eq)]
pub struct Cursor<'a> {
  pub source: &'a str,
  pub c0: usize,
  pub c1: usize,
}

impl<'a> Cursor<'a> {
  pub fn new(source: &'a str) -> Self {
    Cursor {
      source,
      // Double cursor that will be used to delimit a token
      c0: 0,
      c1: 0,
    }
  }

  pub fn stretch(&mut self) {
    self.c1 += 1;
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

