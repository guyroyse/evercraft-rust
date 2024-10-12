use std::fmt::*;

#[derive(Debug)]
pub struct EvercraftError {
  pub message: String,
}

impl Display for EvercraftError {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{}", self.message)
  }
}
