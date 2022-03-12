use crate::*;

pub struct Hero {
  name: String,
  alignment: Alignment,
  damage: u16
}

impl Hero {
  pub fn new() -> Self {
    Hero {
      name: String::new(),
      alignment: Alignment::Neutral,
      damage: 0
    }
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn alignment(&self) -> &Alignment {
    &self.alignment
  }

  pub fn set_alignment(&mut self, alignment: Alignment) {
    self.alignment = alignment;
  }
}

impl Combatant for Hero {
  fn armor_class(&self) -> u8 {
    10
  }

  fn hit_points(&self) -> i16 {
    5 - self.damage as i16
  }

  fn damage(&mut self, amount: u16) {
    self.damage = self.damage + amount;
  }

  fn alive(&self) -> bool {
    self.hit_points() > 0
  }
}
