#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Error {
  Parse {
    message: String
  },
}

impl Error {
  pub fn parse(message: String) -> Self {
    Self::Parse {
      message
    }
  }
}
