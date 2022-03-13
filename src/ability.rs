use crate::*;

pub struct Ability {
  score: u8
}

impl Ability {
  pub fn new() -> Self {
    Ability { score: 10 }
  }

  pub fn score(&self) -> u8 {
    self.score
  }

  pub fn set_score(&mut self, score: u8) -> Result<(), EvercraftError> {
    if score >=1 && score <= 20 {
      self.score = score;
      Ok(())
    } else {
      Err(EvercraftError { message: "Ability score must be between 1 and 20 inclusive".into() })
    }
  }

  pub fn modifier(&self) -> i8 {
    let score = self.score as i8;
    score / 2 - 5
  }
}
