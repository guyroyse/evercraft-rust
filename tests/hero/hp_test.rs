use evercraft::*;

#[test]
fn it_has_default_hp() {
  let hero = Hero::new();
  assert_eq!(hero.hp(), 5);
}

#[test]
fn its_hp_goes_down_when_damaged() {
  let mut hero = Hero::new();
  hero.damage(3);
  assert_eq!(hero.hp(), 2)
}

#[test]
fn its_hp_goes_down_when_damaged_repeatedly() {
  let mut hero = Hero::new();
  hero.damage(3);
  hero.damage(1);
  assert_eq!(hero.hp(), 1)
}

#[test]
fn its_hp_goes_negative_when_damaged_enough() {
  let mut hero = Hero::new();
  hero.damage(10);
  assert_eq!(hero.hp(), -5)
}
