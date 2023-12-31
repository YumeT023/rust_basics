#[cfg(test)]
mod tests {
  use crate::lexer::cursor::Cursor;
  use crate::lexer::Span;

  #[test]
  fn create_cursor_with_given_input() {
    let source = "lexer source";
    assert_eq!(Cursor::new(source), Cursor {
      chars: source.chars(),
      source,
      c0: 0,
      c1: 0,
    })
  }

  #[test]
  fn basic_move_within_on_source() {
    let source = "cursor";
    let mut cursor = Cursor::new(source);

    assert!(!cursor.is_eof());

    assert_eq!(cursor.current_tok_span(), (Span::new(0, 0), /* none */ ""));
    assert_eq!(cursor.current_tok_len(), 0);
    cursor.stretch(); // "c"
    cursor.stretch(); // "cu"
    assert_eq!(cursor.current_tok_span(), (Span::new(0, 2), "cu"));
    assert_eq!(cursor.current_tok_len(), 2);

    // Moves to next tok
    cursor.reset_current_tok();
    assert_eq!(cursor.current_tok_span(), (Span::new(2, 2), /* none */ ""));
    assert_eq!(cursor.current_tok_len(), 0);
    cursor.stretch(); // "r"
    cursor.stretch(); // "rs"
    cursor.stretch(); // "rso"
    cursor.stretch(); // "rsor"
    assert_eq!(cursor.current_tok_span(), (Span::new(2, 6), "rsor"));
    assert_eq!(cursor.current_tok_len(), 4);

    assert!(cursor.is_eof());
  }

  #[test]
  fn take_char_while() {
    let source = "   symbol   ";
    let mut cursor = Cursor::new(source);

    cursor.stretch_while(|c| c.is_whitespace());
    assert_eq!(cursor.current_tok_len(), 3);
    cursor.reset_current_tok();

    cursor.stretch_while(|c| c.is_alphabetic());
    assert_eq!(cursor.current_tok_val(), "symbol");
    cursor.reset_current_tok();

    cursor.stretch_while(|c| c.is_whitespace());
    assert_eq!(cursor.current_tok_len(), 3);
    cursor.reset_current_tok();
  }
}
