#[cfg(test)]
mod tests {
  use crate::lexer::Lexer;
  use crate::test_common::{assert_eq_opt, coma, l_curly, r_curly, symbol};

  #[test]
  fn lex_token_one_by_one() {
    let source = "symbol_nt{field0,field1}";
    let mut lexer = Lexer::new(source);
    assert_eq_opt(lexer.lex(), symbol("symbol_nt"));
    assert_eq_opt(lexer.lex(), l_curly());
    assert_eq_opt(lexer.lex(), symbol("field0"));
    assert_eq_opt(lexer.lex(), coma());
    assert_eq_opt(lexer.lex(), symbol("field1"));
    assert_eq_opt(lexer.lex(), r_curly());
  }
}
