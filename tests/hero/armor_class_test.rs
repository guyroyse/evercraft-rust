use evercraft::*;

#[test]
fn it_has_default_armor_class() {
  let hero = Hero::new();
  assert_eq!(hero.armor_class(), 10);
}
