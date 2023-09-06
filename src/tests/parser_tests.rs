#[cfg(test)]
mod tests {
  use crate::error::Error;
  use crate::lexer::Span;
  use crate::parser::ast::{PropField, Query, Symbol};
  use crate::parser::{parse, Parser};
  use crate::tests::util::{coma, l_curly, r_curly, symbol};

  pub fn ast_symbol(name: &'static str, span: Span) -> Symbol {
    Symbol::from(symbol(name, span))
  }

  #[test]
  fn parse_empty_prop_query() -> Result<(), Error> {
    let query_with_no_field = vec![
      symbol("sym", Span::default()),
      l_curly(Span::default()),
      r_curly(Span::default()),
    ].into_iter();

    let mut parser = Parser::new(query_with_no_field);
    let query = parser.parse_query()?;

    assert_eq!(query.symbol, ast_symbol("sym", Span::default()));
    assert_eq!(query.fields.len(), 0);
    Ok(())
  }


  #[test]
  fn parse_coma_separated_prop_query() -> Result<(), Error> {
    let query_with_symbol_field = vec![
      symbol("sym", Span::default()),
      l_curly(Span::default()), // {
      symbol("field0", Span::default()), coma(Span::default()),
      symbol("field1", Span::default()), coma(Span::default()),
      symbol("field2", Span::default()),
      r_curly(Span::default()), // }
    ].into_iter();

    let mut parser = Parser::new(query_with_symbol_field);
    let query = parser.parse_query()?;

    assert_eq!(query.symbol, ast_symbol("sym", Span::default()));
    assert_eq!(query.fields.len(), 3);
    assert_eq!(query.fields[0], PropField::Symbol(ast_symbol("field0", Span::default())));
    assert_eq!(query.fields[1], PropField::Symbol(ast_symbol("field1", Span::default())));
    assert_eq!(query.fields[2], PropField::Symbol(ast_symbol("field2", Span::default())));
    Ok(())
  }

  #[test]
  fn parse_nested_query() -> Result<(), Error> {
    let query_with_symbol_field = vec![
      symbol("sym", Span::default()),
      l_curly(Span::default()), // {
      symbol("nested_sym", Span::default()),
      l_curly(Span::default()), // | {
      symbol("n_field0", Span::default()), coma(Span::default()),
      symbol("n_field1", Span::default()), coma(Span::default()),
      r_curly(Span::default()), coma(Span::default()), // | }
      symbol("field0", Span::default()), coma(Span::default()),
      r_curly(Span::default()), // }
    ].into_iter();

    let mut parser = Parser::new(query_with_symbol_field);
    let query = parser.parse_query()?;

    assert_eq!(query.symbol, ast_symbol("sym", Span::default()));
    assert_eq!(query.fields.len(), 2);
    assert_eq!(query.fields[0], PropField::Query(Query::new(ast_symbol("nested_sym", Span::default()), vec![
      PropField::Symbol(ast_symbol("n_field0", Span::default())),
      PropField::Symbol(ast_symbol("n_field1", Span::default())),
    ])));
    assert_eq!(query.fields[1], PropField::Symbol(ast_symbol("field0", Span::default())));
    Ok(())
  }

  #[test]
  fn parse_source() -> Result<(), Error> {
    let source = "sym { nested_sym { n_field0, n_field1 }, field0,}";

    let query = parse(source)?;

    assert_eq!(query.symbol, ast_symbol("sym", Span::default()));
    assert_eq!(query.fields.len(), 2);
    assert_eq!(query.fields[0], PropField::Query(Query::new(ast_symbol("nested_sym", Span::default()), vec![
      PropField::Symbol(ast_symbol("n_field0", Span::default())),
      PropField::Symbol(ast_symbol("n_field1", Span::default())),
    ])));
    assert_eq!(query.fields[1], PropField::Symbol(ast_symbol("field0", Span::default())));
    Ok(())
  }
}
