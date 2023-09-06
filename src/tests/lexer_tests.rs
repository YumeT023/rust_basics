#[cfg(test)]
mod tests {
  use crate::lexer::{Lexer, Span, tokenize};
  use crate::tests::util::{assert_eq_opt, coma, l_curly, r_curly, symbol};

  #[test]
  fn lex_token_one_by_one() {
    let source = "symbol_nt{field0,field1}";
    let mut lexer = Lexer::new(source);
    assert_eq_opt(lexer.next_tok(), symbol("symbol_nt", Span::new(0, 9)));
    assert_eq_opt(lexer.next_tok(), l_curly(Span::new(9, 10)));
    assert_eq_opt(lexer.next_tok(), symbol("field0", Span::new(10, 16)));
    assert_eq_opt(lexer.next_tok(), coma(Span::new(16, 17)));
    assert_eq_opt(lexer.next_tok(), symbol("field1", Span::new(17, 23)));
    assert_eq_opt(lexer.next_tok(), r_curly(Span::new(23, 24)));
  }

  #[test]
  fn tokenize_source_into_tok_stream() {
    let source = "symbol_nt{field0,field1}";
    let tokens = tokenize(source);
    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens, vec![
      symbol("symbol_nt", Span::new(0, 9)),
      l_curly(Span::new(9, 10)),
      symbol("field0", Span::new(10, 16)),
      coma(Span::new(16, 17)),
      symbol("field1", Span::new(17, 23)),
      r_curly(Span::new(23, 24)),
    ]);
  }
}
