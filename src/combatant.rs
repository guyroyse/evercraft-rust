pub trait Combatant {
  fn armor_class(&self) -> u8;
  fn hit_points(&self) -> i16;
  fn damage(&mut self, amount: u16);
  fn alive(&self) -> bool;
}
