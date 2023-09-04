#[cfg(test)]
mod tests {
  use crate::error::Error;
  use crate::parser::Parser;
  use crate::ast::{PropField, Query, Symbol};
  use crate::test_common::{coma, l_curly, r_curly, symbol};

  pub fn ast_symbol(name: &'static str) -> Symbol {
    Symbol::from(symbol(name))
  }

  #[test]
  fn parse_empty_prop_query() -> Result<(), Error> {
    let query_with_no_field = vec![
      symbol("sym"),
      l_curly(),
      r_curly(),
    ].into_iter();

    let mut parser = Parser::new(query_with_no_field);
    let query = parser.parse_query()?;

    assert_eq!(query.symbol, ast_symbol("sym"));
    assert_eq!(query.fields.len(), 0);
    Ok(())
  }


  #[test]
  fn parse_coma_separated_prop_query() -> Result<(), Error> {
    let query_with_symbol_field = vec![
      symbol("sym"),
      l_curly(), // {
      symbol("field0"), coma(),
      symbol("field1"), coma(),
      symbol("field2"),
      r_curly(), // }
    ].into_iter();

    let mut parser = Parser::new(query_with_symbol_field);
    let query = parser.parse_query()?;

    assert_eq!(query.symbol, ast_symbol("sym"));
    assert_eq!(query.fields.len(), 3);
    assert_eq!(query.fields[0], PropField::Symbol(ast_symbol("field0")));
    assert_eq!(query.fields[1], PropField::Symbol(ast_symbol("field1")));
    assert_eq!(query.fields[2], PropField::Symbol(ast_symbol("field2")));
    Ok(())
  }

  #[test]
  fn parse_nested_query() -> Result<(), Error> {
    let query_with_symbol_field = vec![
      symbol("sym"),
      l_curly(), // {
      symbol("nested_sym"),
      l_curly(), // | {
      symbol("n_field0"), coma(),
      symbol("n_field1"), coma(),
      r_curly(), coma(), // | }
      symbol("field0"), coma(),
      r_curly(), // }
    ].into_iter();

    let mut parser = Parser::new(query_with_symbol_field);
    let query = parser.parse_query()?;

    assert_eq!(query.symbol, ast_symbol("sym"));
    assert_eq!(query.fields.len(), 2);
    assert_eq!(query.fields[0], PropField::Query(Query::new(ast_symbol("nested_sym"), vec![
      PropField::Symbol(ast_symbol("n_field0")),
      PropField::Symbol(ast_symbol("n_field1")),
    ])));
    assert_eq!(query.fields[1], PropField::Symbol(ast_symbol("field0")));
    Ok(())
  }
}
