use crate::*;
use std::cmp;

pub struct Hero {
  name: String,
  alignment: Alignment,
  strength: Ability,
  dexterity: Ability,
  damage: u16
}

impl Hero {
  pub fn new() -> Self {
    Hero {
      name: String::new(),
      alignment: Alignment::Neutral,
      strength: Ability::new(),
      dexterity: Ability::new(),
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

  pub fn strength(&mut self) -> &mut Ability {
    &mut self.strength
  }

  pub fn dexterity(&mut self) -> &mut Ability {
    &mut self.dexterity
  }
}

impl Combatant for Hero {
  fn ac(&self) -> u8 {
    (10 + self.dexterity.modifier()) as u8
  }

  fn hp(&self) -> i16 {
    5 - self.damage as i16
  }

  fn hit_modifier(&self) -> i8 {
    self.strength.modifier()
  }

  fn hit_damage(&self) -> u16 {
    cmp::max(1, 1 + self.strength.modifier()) as u16
  }

  fn crit_damage(&self) -> u16 {
    cmp::max(1, 2 + self.strength.modifier() * 2) as u16
  }

  fn damage(&mut self, amount: u16) {
    self.damage = self.damage + amount;
  }

  fn alive(&self) -> bool {
    self.hp() > 0
  }
}
