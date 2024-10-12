use evercraft::*;

#[test]
fn it_is_alive() {
  let hero = Hero::new();
  assert_eq!(hero.alive(), true);
}

#[test]
fn it_is_still_alive_when_damaged() {
  let mut hero = Hero::new();
  hero.damage(3);
  assert_eq!(hero.alive(), true);
}

#[test]
fn it_is_dead_when_damaged_to_0() {
  let mut hero = Hero::new();
  hero.damage(5);
  assert_eq!(hero.alive(), false);
}

#[test]
fn it_is_dead_when_damaged_below_0() {
  let mut hero = Hero::new();
  hero.damage(10);
  assert_eq!(hero.alive(), false);
}
