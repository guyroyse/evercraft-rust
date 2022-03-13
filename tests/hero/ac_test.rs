use evercraft::*;

#[test]
fn it_has_default_ac() {
  let hero = Hero::new();
  assert_eq!(hero.ac(), 10);
}

#[test]
fn it_adds_dexterity_modifier_to_ac() {
  let mut hero = Hero::new();
  hero.dexterity().set_score(14).ok();
  assert_eq!(hero.ac(), 12);
}
