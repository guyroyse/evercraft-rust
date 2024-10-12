use evercraft::*;

#[test]
fn it_has_default_current_hp() {
  let hero = Hero::new();
  assert_eq!(hero.current_hp(), 5);
}

#[test]
fn it_add_constitution_modifier_to_current_hp() {
  let mut hero = Hero::new();
  hero.constitution().set_score(14).ok();
  assert_eq!(hero.current_hp(), 7);
}

#[test]
fn it_goes_down_when_damaged() {
  let mut hero = Hero::new();
  hero.damage(3);
  assert_eq!(hero.current_hp(), 2)
}

#[test]
fn it_goes_down_when_damaged_repeatedly() {
  let mut hero = Hero::new();
  hero.damage(3);
  hero.damage(1);
  assert_eq!(hero.current_hp(), 1)
}

#[test]
fn it_goes_negative_when_damaged_enough() {
  let mut hero = Hero::new();
  hero.damage(10);
  assert_eq!(hero.current_hp(), -5)
}
