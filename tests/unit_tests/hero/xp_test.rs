use evercraft::*;

#[test]
fn it_has_default_xp() {
  let hero = Hero::new();
  assert_eq!(hero.xp(), 0);
}

#[test]
fn it_goes_up_when_xp_is_gained() {
  let mut hero = Hero::new();
  hero.add_xp(500);
  assert_eq!(hero.xp(), 500);
}
