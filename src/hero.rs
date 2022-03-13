use crate::*;
use std::cmp;

pub struct Hero {
  name: String,
  alignment: Alignment,
  strength: Ability,
  dexterity: Ability,
  constitution: Ability,
  damage: u16
}

impl Hero {
  pub fn new() -> Self {
    Hero {
      name: String::new(),
      alignment: Alignment::Neutral,
      strength: Ability::new(),
      dexterity: Ability::new(),
      constitution: Ability::new(),
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

  pub fn constitution(&mut self) -> &mut Ability {
    &mut self.constitution
  }
}

impl Combatant for Hero {
  fn ac(&self) -> u8 {
    let base_ac = 10 as i8;
    let dex_mod = self.dexterity.modifier() as i8;
    (base_ac + dex_mod) as u8
  }

  fn hp(&self) -> u16 {
    let base_hp = 5 as i16;
    let con_mod = self.constitution.modifier() as i16;
    (base_hp + con_mod) as u16
  }

  fn current_hp(&self) -> i16 {
    let hp = self.hp() as i16;
    let damage = self.damage as i16;
    hp - damage
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
    self.current_hp() > 0
  }
}
