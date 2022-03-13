use evercraft::*;

#[test]
fn when_rolling_a_natural_1() {
  let mut attacker = standard_combatant();
  let mut defender = standard_combatant();

  let attack = resolve_attack(1, &mut attacker, &mut defender);

  it_misses(&attack);
  it_is_not_a_critical(&attack);
  it_deals_no_damage(&defender);
}

#[test]
fn when_roll_proceeds_defenders_armor_class() {
  let mut attacker = standard_combatant();
  let mut defender = standard_combatant();

  let attack = resolve_attack(9, &mut attacker, &mut defender);

  it_misses(&attack);
  it_is_not_a_critical(&attack);
  it_deals_no_damage(&defender);
}

#[test]
fn when_roll_equals_defenders_armor_class() {
  let mut attacker = standard_combatant();
  let mut defender = standard_combatant();

  let attack = resolve_attack(10, &mut attacker, &mut defender);

  it_hits(&attack);
  it_is_not_a_critical(&attack);
  it_deals_expected_damage(&defender, 1);
}

#[test]
fn when_roll_exceeds_defenders_armor_class() {
  let mut attacker = standard_combatant();
  let mut defender = standard_combatant();

  let attack = resolve_attack(11, &mut attacker, &mut defender);

  it_hits(&attack);
  it_is_not_a_critical(&attack);
  it_deals_expected_damage(&defender, 1);
}

#[test]
fn when_roll_is_a_natural_20() {
  let mut attacker = standard_combatant();
  let mut defender = standard_combatant();

  let attack = resolve_attack(20, &mut attacker, &mut defender);

  it_hits(&attack);
  it_is_a_critical(&attack);
  it_deals_expected_damage(&defender, 2);
}

fn resolve_attack(roll: u8, attacker: &mut dyn Combatant, defender: &mut dyn Combatant) -> ResolvedAttack {
  let mut attack = Attack::between(attacker, defender);
  attack.resolve(roll)
}

fn it_hits(attack: &ResolvedAttack) {
  assert!(attack.hit() == true);
}

fn it_misses(attack: &ResolvedAttack) {
  assert!(attack.hit() == false);
}

fn it_is_a_critical(attack: &ResolvedAttack) {
  assert!(attack.critical() == true);
}

fn it_is_not_a_critical(attack: &ResolvedAttack) {
  assert!(attack.critical() == false);
}

fn it_deals_no_damage(combatant: &MockCombatant) {
  assert_eq!(combatant.damage_taken(), 0);
}

fn it_deals_expected_damage(combatant: &MockCombatant, expected: u16) {
  assert_eq!(combatant.damage_taken(), expected);
}

fn standard_combatant() -> MockCombatant {
  MockCombatant {
    ac: 10,
    damage: 0
  }
}

struct MockCombatant {
  ac: u8,
  damage: u16,
}

impl MockCombatant {
  fn damage_taken(&self) -> u16 {
    self.damage
  }
}

impl Combatant for MockCombatant {
  fn ac(&self) -> u8 { self.ac }
  fn hp(&self) -> i16 { 5 }
  fn alive(&self) -> bool { true }
  fn hit_modifier(&self) -> i8 { 0 }
  fn hit_damage(&self) -> u16 { 1 }
  fn crit_damage(&self) -> u16 { 2 }
  fn damage(&mut self, amount: u16) {
    self.damage = self.damage + amount;
  }
}
