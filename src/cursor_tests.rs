#[cfg(test)]
mod tests {
  use crate::cursor::Cursor;

  #[test]
  fn create_cursor_with_given_input() {
    let source = "lexer source";
    assert_eq!(Cursor::new(source), Cursor {
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

    assert_eq!(cursor.current_tok_span(), (0, /* none */ "", 0));
    assert_eq!(cursor.current_tok_len(), 0);
    cursor.stretch(); // "c"
    cursor.stretch(); // "cu"
    assert_eq!(cursor.current_tok_span(), (0, "cu", 2));
    assert_eq!(cursor.current_tok_len(), 2);

    // Moves to next tok
    cursor.done_current_tok();
    assert_eq!(cursor.current_tok_span(), (2, /* none */ "", 2));
    assert_eq!(cursor.current_tok_len(), 0);
    cursor.stretch(); // "r"
    cursor.stretch(); // "rs"
    cursor.stretch(); // "rso"
    cursor.stretch(); // "rsor"
    assert_eq!(cursor.current_tok_span(), (2, "rsor", 6));
    assert_eq!(cursor.current_tok_len(), 4);

    assert!(cursor.is_eof());
  }
}
