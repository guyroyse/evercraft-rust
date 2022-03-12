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
