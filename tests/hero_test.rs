use evercraft::*;

#[test]
fn it_has_a_default_name() {
  let hero = Hero::new();
  assert_eq!(hero.name(), "");
}

#[test]
fn it_has_a_mutable_name() {
  let mut hero = Hero::new();
  hero.set_name("Bob".into());
  assert_eq!(hero.name(), "Bob");
}

#[test]
fn it_has_a_default_alignment() {
  let hero = Hero::new();
  assert!(matches!(hero.alignment(), Alignment::Neutral));
}

#[test]
fn it_can_be_good() {
  let mut hero = Hero::new();
  hero.set_alignment(Alignment::Good);
  assert!(matches!(hero.alignment(), Alignment::Good));
}

#[test]
fn it_can_be_neutral() {
  let mut hero = Hero::new();
  hero.set_alignment(Alignment::Good);
  hero.set_alignment(Alignment::Neutral);
  assert!(matches!(hero.alignment(), Alignment::Neutral));
}

#[test]
fn it_can_be_evil() {
  let mut hero = Hero::new();
  hero.set_alignment(Alignment::Evil);
  assert!(matches!(hero.alignment(), Alignment::Evil));
}

#[test]
fn it_has_default_armor_class() {
  let hero = Hero::new();
  assert_eq!(hero.armor_class(), 10);
}

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
