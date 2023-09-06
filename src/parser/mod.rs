use crate::error::Error;
use crate::lexer::{Token, tokenize, TokenKind};
use crate::parser::ast::{PropField, Query, Symbol};

pub mod ast;

#[derive(Debug)]
// the tokens in the iterator will be dropped with this Parser
pub struct Parser<'a, T>
  where
    T: Iterator<Item=Token<'a>>
{
  pub tokens: T,
  // Use two token cursor for backtracking and to avoid cloning 'tokens' iterator
  t0: Option<Token<'a>>,
  t1: Option<Token<'a>>,
}

impl<'a, T> Parser<'a, T>
  where
    T: Iterator<Item=Token<'a>>
{
  pub fn new(input: T) -> Self {
    let mut p = Parser {
      tokens: input,
      t0: None,
      t1: None,
    };
    p.scroll();
    p
  }

  // Parses <query> ::= <symbol> '{' <prop_list> '}' ;
  pub fn parse_query(&mut self) -> Result<Query, Error> {
    // '?' means we are going to propagate the error that this fn_call may cause so we can delegate
    // error handling to the caller
    let symbol = self.parse_symbol()?;
    self.enter_block()?;
    let fields = self.parse_prop_seq()?;
    self.exit_block()?;
    Ok(Query::new(symbol, fields))
  }

  // Parses <symbol> ::= [a-zA-Z_][a-zA-Z0-9_]* ;
  // using Result<T, E> is a way (monad) that we can use to handle error in Rust, it has two variants
  // which are Ok(T) and Err(E), T is the type of data we'd return and E is error's
  pub fn parse_symbol(&mut self) -> Result<Symbol, Error> {
    self.assert_kind(TokenKind::Symbol)?;
    let t = self.t0.unwrap();
    self.scroll();
    Ok(Symbol::from(t))
  }

  // Parses single prop: <prop_field> ::= <query> | <symbol> ;
  // Option<T> is another tagged enum for handling return type, it has two variants which are
  // Some(T) and None, it acts like Optional<T> monad in Java
  // you won't be dealing with undefined and null in Rust:)
  pub fn parse_prop_field(&mut self) -> Result<Option<PropField>, Error> {
    let field = match self.t0 {
      Some(t) => match t.kind {
        TokenKind::Symbol => {
          if self.t1.is_some_and(|t1| t1.kind == TokenKind::LCurly) {
            Some(PropField::Query(self.parse_query()?))
          } else {
            Some(PropField::Symbol(self.parse_symbol()?))
          }
        }
        TokenKind::RCurly => None,
        _ => return Err(Error::parse(format!("Expected token::(Symbol | RCurly), found token::{:?}", t.kind)))
      },
      None => return Err(Error::parse("Unexpected token::Eof".to_string()))
    };
    Ok(field)
  }

  // Parses sequence of prop separated by ',': <prop_field_seq> ::= <prop_field> | <prop_field> <prop_list> ;
  pub fn parse_prop_seq(&mut self) -> Result<Vec<PropField>, Error> {
    let mut fields = vec![];
    // we can translate this while loop as follows
    // if self.parse_prop_field()? returns a value matching Some(_), meaning it is the Some(T)
    // variant in Option<T>
    while let Some(p) = self.parse_prop_field()? {
      fields.push(p);
      if self.t0.is_some_and(|t| t.kind != TokenKind::Coma) {
        break;
      }
      self.scroll();
    }
    Ok(fields)
  }

  // Enters block '{'
  // This will be useful if we need to keep track of the scope context
  // We can also use this fn to trigger onEnter(...) for a given Listener
  fn enter_block(&mut self) -> Result<(), Error> {
    self.assert_kind(TokenKind::LCurly)?;
    self.scroll();
    Ok(())
  }

  // Enters block '}'
  // This will be useful if we need to keep track of the scope context
  // We can also use this fn to trigger onExit(...) for a given Listener
  // onExit()
  fn exit_block(&mut self) -> Result<(), Error> {
    self.assert_kind(TokenKind::RCurly)?;
    self.scroll();
    Ok(())
  }

  // Moves token cursor forward
  // t0 takes token from t1
  // t1 takes the next in the iterator
  fn scroll(&mut self) -> Option<Token> {
    let t0 = self.t1.or_else(|| self.tokens.next());
    self.t0 = t0;
    self.t1 = self.tokens.next();
    t0
  }

  // Asserts the left most token in the cursor ('t0') has the given TokenKind
  fn assert_kind(&mut self, kind: TokenKind) -> Result<Token, Error> {
    match self.t0 {
      Some(t) => {
        if t.kind != kind {
          return Err(Error::parse(format!("Expected token::{:?}, found token::{:?}", kind, t.kind)));
        }
        Ok(t)
      }
      _ => Err(Error::parse("Unexpected token::Eof".to_string()))
    }
  }
}

pub fn parse(source: &str) -> Result<Query, Error> {
  let token_stream = tokenize(source);
  let mut parser = Parser::new(token_stream.into_iter());
  parser.parse_query()
}
