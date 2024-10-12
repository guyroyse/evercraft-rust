use crate::*;

pub struct Attack<'a> {
  #[allow(dead_code)]
  attacker: &'a mut dyn Combatant,
  defender: &'a mut dyn Combatant,
}

pub struct ResolvedAttack {
  hit: bool,
  crit: bool,
}

pub struct Roll {
  roll: u8,
}

impl<'a> Attack<'a> {
  pub fn between(attacker: &'a mut dyn Combatant, defender: &'a mut dyn Combatant) -> Attack<'a> {
    Attack { attacker, defender }
  }

  pub fn resolve(&mut self, roll: u8) -> ResolvedAttack {
    let smart_roll = Roll::new(roll);
    let adjusted_roll = smart_roll.value_plus(self.attacker.hit_modifier());

    let hit = !smart_roll.natural_1() && adjusted_roll >= (self.defender.ac() as i8);
    let crit = smart_roll.natural_20();

    match (hit, crit) {
      (_, true) => self.resolve_crit(),
      (true, _) => self.resolve_hit(),
      _ => (),
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

impl Roll {
  pub fn new(roll: u8) -> Self {
    Roll { roll }
  }

  pub fn natural_1(&self) -> bool {
    self.roll == 1
  }

  pub fn natural_20(&self) -> bool {
    self.roll == 20
  }

  pub fn value(&self) -> u8 {
    self.roll as u8
  }

  pub fn value_plus(&self, modifier: i8) -> i8 {
    (self.roll as i8) + modifier
  }
}
