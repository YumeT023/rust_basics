
use crate::lexer::Token;

#[derive(Debug, PartialEq)]
pub struct Symbol {
  pub name: String,
}

// N1
// enums in Rust are tagged (carry information about which variant they are)
// It is possible to implement a trait for Query and Symbol. This is a common practice in many programming language to provide
// common behaviour for different types but that will enforces us to change the 'PropsList' type as follow
// type PropList = Vec<Box<dyn CommonTrait>>
// - trait objects must include the 'dyn' keyword to tell Rust that we will be doing dynamic dispatch
// - It should then be Wrapped inside a Box (or another ptr type, like 'Arc', 'Rc') because its size is not known at
// compile time. This is because different types can implement the same trait and have different sizes. To solve that
// problem, we can allocate trait objects on the heap (by using 'Box', 'Arc', 'Rc') because ptr types are always the same
// regardless of the size of the data they point to. They contain a pointer to the data stored on the heap
#[derive(Debug, PartialEq)]
pub enum PropField {
  Query(Query),
  Symbol(Symbol),
}

// we can refer to variant (which carry information about the ast) with PropField
pub type PropList = Vec<PropField>;

#[derive(Debug, PartialEq)]
pub struct Query {
  pub symbol: Symbol,
  pub fields: PropList,
}

// N2. same as on N1
pub enum Ast {
  Query(Query),
}

impl From<Token<'_>> for Symbol {
  fn from(t: Token) -> Self {
    Symbol {
      name: String::from(t.str_val)
    }
  }
}

impl Query {
  pub fn new(symbol: Symbol, fields: Vec<PropField>) -> Self {
    Query {
      symbol,
      fields,
    }
  }
}
