use crate::*;

pub struct Attack<'a> {
  #[allow(dead_code)]
  attacker: &'a mut dyn Combatant,
  defender: &'a mut dyn Combatant
}

pub struct ResolvedAttack {
  hit: bool,
  crit: bool
}

impl<'a> Attack<'a> {
  pub fn between(attacker: &'a mut dyn Combatant, defender: &'a mut dyn Combatant) -> Attack<'a> {
    Attack { attacker, defender }
  }

  pub fn resolve(&mut self, roll: u8) -> ResolvedAttack {
    let hit = roll >= self.defender.ac();
    let crit = roll == 20;
    if hit && crit {
      self.defender.damage(2);
    } else if hit {
      self.defender.damage(1);
    }

    ResolvedAttack { hit, crit }
  }
}

impl ResolvedAttack {
  pub fn hit(&self) -> bool {
    self.hit
  }

  pub fn crit(&self) -> bool {
    self.crit
  }
}
