use evercraft::*;

#[test]
fn it_has_default_score() {
  let ability = Ability::new();
  assert_eq!(ability.score(), 10);
}

#[test]
fn it_has_default_modifier() {
  let ability = Ability::new();
  assert_eq!(ability.modifier(), 0);
}

#[test]
fn it_has_a_mutable_score() {
  let mut ability = Ability::new();

  let result = ability.set_score(14).ok();

  assert_eq!(result.unwrap(), ());
  assert_eq!(ability.score(), 14);
}

#[test]
fn it_complains_when_score_is_0() {
  let mut ability = Ability::new();

  let result = ability.set_score(0);

  match result {
    Ok(()) => panic!("Expected Error"),
    Err(error) => assert_eq!(error.message, "Ability score must be between 1 and 20 inclusive")
  };

  assert_eq!(ability.score(), 10);
}

#[test]
fn it_complains_when_score_is_over_20() {
  let mut ability = Ability::new();

  let result = ability.set_score(21);

  match result {
    Ok(()) => panic!("Expected Error"),
    Err(error) => assert_eq!(error.message, "Ability score must be between 1 and 20 inclusive")
  };

  assert_eq!(ability.score(), 10);
}

#[test]
fn it_changes_the_modifier_when_the_score_changes() {
  let mut ability = Ability::new();

  let scores_and_modifiers: [(u8, i8); 20] = [
    (1, -5), (2, -4), (3, -4), (4, -3), (5, -3), (6, -2), (7, -2), (8, -1), (9, -1), (10, 0),
    (11, 0), (12, 1), (13, 1), (14, 2), (15, 2), (16, 3), (17, 3), (18, 4), (19, 4), (20, 5)];
  
  for (score, modifier) in scores_and_modifiers {
    ability.set_score(score).ok();
    assert_eq!(ability.modifier(), modifier);
  }
}
