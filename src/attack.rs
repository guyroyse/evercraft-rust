use crate::Hero;

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
