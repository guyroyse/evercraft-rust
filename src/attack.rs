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

    match (hit, crit) {
      (_, true) => self.resolve_crit(),
      (true, _) => self.resolve_hit(),
      _ => ()
    }

    ResolvedAttack { hit, crit }
  }

  fn resolve_hit(&mut self) {
    self.defender.damage(self.attacker.hit_damage());
    self.attacker.add_xp(10);
  }

  fn resolve_crit(&mut self) {
    self.defender.damage(self.attacker.crit_damage());
    self.attacker.add_xp(10);
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
