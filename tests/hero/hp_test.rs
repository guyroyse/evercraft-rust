use evercraft::*;

#[test]
fn it_has_default_hp() {
  let hero = Hero::new();
  assert_eq!(hero.hp(), 5);
}

#[test]
fn it_doesnt_go_down_when_damaged() {
  let mut hero = Hero::new();
  hero.damage(3);
  assert_eq!(hero.hp(), 5);
}

#[test]
fn it_add_constitution_modifier_to_hp() {
  let mut hero = Hero::new();
  hero.constitution().set_score(14).ok();
  assert_eq!(hero.hp(), 7);
}
