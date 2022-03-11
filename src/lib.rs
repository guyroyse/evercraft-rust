use String;

#[derive(Clone)]
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

  pub fn armor_class(&self) -> u8 {
    10
  }

  pub fn hit_points(&self) -> i16 {
    5 - self.damage as i16
  }

  pub fn damage(&mut self, amount: u16) {
    self.damage = self.damage + amount;
  }
}

#[derive(Clone)]
pub enum Alignment { Good, Neutral, Evil }

pub fn resolve_attack(roll: u8, defender: &mut Hero) -> bool {
  let hit = roll >= defender.armor_class();
  let critical = roll == 20;
  if hit && critical {
    defender.damage(2);
  } else if hit {
    defender.damage(1);
  }
  hit
}

pub struct Attack<'a> {
  attacker: &'a mut Hero,
  defender: &'a mut Hero
}

pub struct ResolvedAttack {
  hit: bool,
  critical: bool
}

impl<'a> Attack<'a> {
  pub fn between(attacker: &'a mut Hero, defender: &'a mut Hero) -> Attack<'a> {
    Attack { attacker, defender }
  }

  pub fn resolve(&mut self, roll: u8) -> ResolvedAttack {
    let hit = roll >= self.defender.armor_class();
    let critical = roll == 20;
    if hit && critical {
      self.defender.damage(2);
    } else if hit {
      self.defender.damage(1);
    }

    ResolvedAttack { hit, critical }
  }
}

impl ResolvedAttack {
  pub fn hit(&self) -> bool {
    self.hit
  }

  pub fn critical(&self) -> bool {
    self.critical
  }
}
