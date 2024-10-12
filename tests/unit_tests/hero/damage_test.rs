use evercraft::*;

#[test]
fn it_has_default_damage() {
  let hero = Hero::new();
  assert_eq!(hero.hit_damage(), 1);
  assert_eq!(hero.crit_damage(), 2);
}

#[test]
fn it_adds_strength_modifier_to_damage() {
  let mut hero = Hero::new();
  hero.strength().set_score(14).ok();
  assert_eq!(hero.hit_damage(), 3);
  assert_eq!(hero.crit_damage(), 6);
}

#[test]
fn it_has_a_minimum_damage_of_1_regardless_of_strength() {
  let mut hero = Hero::new();
  hero.strength().set_score(6).ok();
  assert_eq!(hero.hit_damage(), 1);
  assert_eq!(hero.crit_damage(), 1);
}
