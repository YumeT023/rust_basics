#[cfg(test)]
mod tests {
  use crate::lexer::{Lexer, tokenize};
  use crate::test_common::{assert_eq_opt, coma, l_curly, r_curly, symbol};

  #[test]
  fn lex_token_one_by_one() {
    let source = "symbol_nt{field0,field1}";
    let mut lexer = Lexer::new(source);
    assert_eq_opt(lexer.next_tok(), symbol("symbol_nt"));
    assert_eq_opt(lexer.next_tok(), l_curly());
    assert_eq_opt(lexer.next_tok(), symbol("field0"));
    assert_eq_opt(lexer.next_tok(), coma());
    assert_eq_opt(lexer.next_tok(), symbol("field1"));
    assert_eq_opt(lexer.next_tok(), r_curly());
  }

  #[test]
  fn tokenize_source_into_tok_stream() {
    let source = "symbol_nt{field0,field1}";
    let tokens = tokenize(source);
    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens, vec![
      symbol("symbol_nt"),
      l_curly(),
      symbol("field0"),
      coma(),
      symbol("field1"),
      r_curly(),
    ]);
  }
}
