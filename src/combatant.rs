pub trait Combatant {
  fn ac(&self) -> u8;
  fn hp(&self) -> i16;
  fn hit_modifier(&self) -> i8;
  fn hit_damage(&self) -> u16;
  fn crit_damage(&self) -> u16;
  fn damage(&mut self, amount: u16);
  fn alive(&self) -> bool;
}
