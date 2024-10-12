use evercraft::*;

#[test]
fn it_has_default_hit_modifier() {
  let hero = Hero::new();
  assert_eq!(hero.hit_modifier(), 0);
}

#[test]
fn it_adds_strength_modifier_to_hit_modifier() {
  let mut hero = Hero::new();
  hero.strength().set_score(14).ok();
  assert_eq!(hero.hit_modifier(), 2);
}
