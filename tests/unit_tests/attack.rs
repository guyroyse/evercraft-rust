use evercraft::*;

#[test]
fn when_rolling_a_natural_1() {
  let mut attacker = attacker_with_hit_modifier_1();
  let mut defender = defender_with_ac_12();

  let attack = resolve_attack(1, &mut attacker, &mut defender);

  it_misses(&attack);
  it_is_not_a_crit(&attack);
  it_deals_no_damage(&defender);
  it_gives_the_attacker_no_xp(&attacker);
}

#[test]
fn when_rolling_a_natural_1_with_a_really_big_hit_modifier() {
  let mut attacker = attacker_with_hit_modifier_11();
  let mut defender = defender_with_ac_12();

  let attack = resolve_attack(1, &mut attacker, &mut defender);

  it_misses(&attack);
  it_is_not_a_crit(&attack);
  it_deals_no_damage(&defender);
  it_gives_the_attacker_no_xp(&attacker);
}

#[test]
fn when_roll_plus_hit_modifier_proceeds_defenders_armor_class() {
  let mut attacker = attacker_with_hit_modifier_1();
  let mut defender = defender_with_ac_12();

  let attack = resolve_attack(10, &mut attacker, &mut defender);

  it_misses(&attack);
  it_is_not_a_crit(&attack);
  it_deals_no_damage(&defender);
  it_gives_the_attacker_no_xp(&attacker);
}

#[test]
fn when_roll_plus_hit_modifier_equals_defenders_armor_class() {
  let mut attacker = attacker_with_hit_modifier_1();
  let mut defender = defender_with_ac_12();

  let attack = resolve_attack(11, &mut attacker, &mut defender);

  it_hits(&attack);
  it_is_not_a_crit(&attack);
  it_deals_expected_damage(&defender, 2);
  it_gives_the_attacker_xp(&attacker, 10);
}

#[test]
fn when_roll_plus_hit_modifier_exceeds_defenders_armor_class() {
  let mut attacker = attacker_with_hit_modifier_1();
  let mut defender = defender_with_ac_12();

  let attack = resolve_attack(12, &mut attacker, &mut defender);

  it_hits(&attack);
  it_is_not_a_crit(&attack);
  it_deals_expected_damage(&defender, 2);
  it_gives_the_attacker_xp(&attacker, 10);
}

#[test]
fn when_roll_is_a_natural_20() {
  let mut attacker = attacker_with_hit_modifier_1();
  let mut defender = defender_with_ac_12();

  let attack = resolve_attack(20, &mut attacker, &mut defender);

  it_hits(&attack);
  it_is_a_crit(&attack);
  it_deals_expected_damage(&defender, 3);
  it_gives_the_attacker_xp(&attacker, 10);
}

fn resolve_attack(
  roll: u8,
  attacker: &mut dyn Combatant,
  defender: &mut dyn Combatant
) -> ResolvedAttack {
  let mut attack = Attack::between(attacker, defender);
  attack.resolve(roll)
}

fn it_hits(attack: &ResolvedAttack) {
  assert!(attack.hit() == true);
}

fn it_misses(attack: &ResolvedAttack) {
  assert!(attack.hit() == false);
}

fn it_is_a_crit(attack: &ResolvedAttack) {
  assert!(attack.crit() == true);
}

fn it_is_not_a_crit(attack: &ResolvedAttack) {
  assert!(attack.crit() == false);
}

fn it_deals_no_damage(combatant: &MockCombatant) {
  assert_eq!(combatant.damage_taken(), 0);
}

fn it_deals_expected_damage(combatant: &MockCombatant, expected: u16) {
  assert_eq!(combatant.damage_taken(), expected);
}

fn it_gives_the_attacker_no_xp(combatant: &MockCombatant) {
  assert_eq!(combatant.xp_received(), 0);
}

fn it_gives_the_attacker_xp(combatant: &MockCombatant, expected: u32) {
  assert_eq!(combatant.xp_received(), expected);
}

fn attacker_with_hit_modifier_1() -> MockCombatant {
  MockCombatant { ac: 10, damage: 0, xp: 0, hit_modifier: 1, hit_damage: 2, crit_damage: 3 }
}

fn attacker_with_hit_modifier_11() -> MockCombatant {
  MockCombatant { ac: 10, damage: 0, xp: 0, hit_modifier: 11, hit_damage: 2, crit_damage: 3 }
}

fn defender_with_ac_12() -> MockCombatant {
  MockCombatant { ac: 12, damage: 0, xp: 0, hit_modifier: 0, hit_damage: 0, crit_damage: 0 }
}

struct MockCombatant {
  ac: u8,
  hit_modifier: i8,
  hit_damage: u16,
  crit_damage: u16,
  damage: u16,
  xp: u32,
}

impl MockCombatant {
  fn damage_taken(&self) -> u16 {
    self.damage
  }
  fn xp_received(&self) -> u32 {
    self.xp
  }
}

impl Combatant for MockCombatant {
  fn ac(&self) -> u8 {
    self.ac
  }
  fn hp(&self) -> u16 {
    5
  }
  fn current_hp(&self) -> i16 {
    5
  }
  fn alive(&self) -> bool {
    true
  }
  fn hit_modifier(&self) -> i8 {
    self.hit_modifier
  }
  fn hit_damage(&self) -> u16 {
    self.hit_damage
  }
  fn crit_damage(&self) -> u16 {
    self.crit_damage
  }
  fn xp(&self) -> u32 {
    0
  }
  fn damage(&mut self, amount: u16) {
    self.damage += amount;
  }
  fn add_xp(&mut self, amount: u32) {
    self.xp += amount;
  }
}
