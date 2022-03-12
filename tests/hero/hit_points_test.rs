use evercraft::*;

#[test]
fn it_has_default_hit_points() {
  let hero = Hero::new();
  assert_eq!(hero.hit_points(), 5);
}

#[test]
fn its_hit_points_go_down_when_damaged() {
  let mut hero = Hero::new();
  hero.damage(3);
  assert_eq!(hero.hit_points(), 2)
}

#[test]
fn its_hit_points_go_down_when_damaged_repeatedly() {
  let mut hero = Hero::new();
  hero.damage(3);
  hero.damage(1);
  assert_eq!(hero.hit_points(), 1)
}

#[test]
fn its_hit_points_go_negative_when_damaged_enough() {
  let mut hero = Hero::new();
  hero.damage(10);
  assert_eq!(hero.hit_points(), -5)
}
