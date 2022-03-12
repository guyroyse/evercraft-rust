use evercraft::*;

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
