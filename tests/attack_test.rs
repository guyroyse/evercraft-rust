use evercraft::*;

#[test]
fn it_misses_on_a_natural_1() {
  let mut attacker = Hero::new();
  let mut defender = Hero::new();
  let roll = 1;
  let expected_hp = defender.hit_points();

  let mut attack = Attack::between(&mut attacker, &mut defender);
  let attack = attack.resolve(roll);

  assert!(attack.hit() == false);
  assert!(attack.critical() == false);
  assert_eq!(defender.hit_points(), expected_hp);
}

#[test]
fn it_misses_when_a_roll_proceeds_defenders_armor_class() {
  let mut attacker = Hero::new();
  let mut defender = Hero::new();
  let roll = defender.armor_class() - 1;
  let expected_hp = defender.hit_points();

  let mut attack = Attack::between(&mut attacker, &mut defender);
  let attack = attack.resolve(roll);

  assert!(attack.hit() == false);
  assert!(attack.critical() == false);
  assert_eq!(defender.hit_points(), expected_hp);
}

#[test]
fn it_hits_when_roll_meets_defenders_armor_class() {
  let mut attacker = Hero::new();
  let mut defender = Hero::new();
  let roll = defender.armor_class();
  let expected_hp = defender.hit_points() - 1;

  let mut attack = Attack::between(&mut attacker, &mut defender);
  let attack = attack.resolve(roll);

  assert!(attack.hit() == true);
  assert!(attack.critical() == false);
  assert_eq!(defender.hit_points(), expected_hp);
}

#[test]
fn it_hits_when_roll_exceeds_defenders_armor_class() {
  let mut attacker = Hero::new();
  let mut defender = Hero::new();
  let roll = defender.armor_class() + 1;
  let expected_hp = defender.hit_points() - 1;

  let mut attack = Attack::between(&mut attacker, &mut defender);
  let attack = attack.resolve(roll);

  assert!(attack.hit() == true);
  assert!(attack.critical() == false);
  assert_eq!(defender.hit_points(), expected_hp);
}

#[test]
fn it_hits_on_a_natural_20() {
  let mut attacker = Hero::new();
  let mut defender = Hero::new();
  let roll = 20;
  let expected_hp = defender.hit_points() - 2;

  let mut attack = Attack::between(&mut attacker, &mut defender);
  let attack = attack.resolve(roll);

  assert!(attack.hit() == true);
  assert!(attack.critical() == true);
  assert_eq!(defender.hit_points(), expected_hp);
}
