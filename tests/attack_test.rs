use evercraft::*;

#[test]
fn it_misses_on_a_natural_1() {
  let attacker = Hero::new();
  let defender = Hero::new();
  let roll = 1;

  let attack = Attack::between(attacker, defender);

  let hit = attack.resolve(roll);
  assert!(hit == false);
}

#[test]
fn it_misses_when_a_roll_proceeds_defenders_armor_class() {
  let attacker = Hero::new();
  let defender = Hero::new();
  let roll = defender.armor_class() - 1;

  let attack = Attack::between(attacker, defender);

  let hit = attack.resolve(roll);
  assert!(hit == false);
}

#[test]
fn it_hits_when_roll_meets_defenders_armor_class() {
  let attacker = Hero::new();
  let defender = Hero::new();
  let roll = defender.armor_class();

  let attack = Attack::between(attacker, defender);

  let hit = attack.resolve(roll);
  assert!(hit == true);
}

#[test]
fn it_hits_when_roll_exceeds_defenders_armor_class() {
  let attacker = Hero::new();
  let defender = Hero::new();
  let roll = defender.armor_class() + 1;

  let attack = Attack::between(attacker, defender);

  let hit = attack.resolve(roll);
  assert!(hit == true);
}

#[test]
fn it_hits_on_a_natural_20() {
  let attacker = Hero::new();
  let defender = Hero::new();
  let roll = 20;

  let attack = Attack::between(attacker, defender);

  let hit = attack.resolve(roll);
  assert!(hit == true);
}
