use bitcards::{
    canonical::{hash_hex, serialize},
    generator::CardGenerator,
    model::{CardClass, TYPE_ADVANTAGE_DAMAGE_BONUS},
};

#[test]
fn every_class_is_generated_and_deterministic() {
    let generator = CardGenerator::default();
    let mut seen = Vec::new();
    for seed in 0..=255u8 {
        let first = generator.generate(1, &[0; 32], &[seed]).unwrap();
        let second = generator.generate(1, &[0; 32], &[seed]).unwrap();
        assert_eq!(serialize(&first), serialize(&second));
        assert_eq!(first.hash, second.hash);
        if !seen.contains(&first.class) {
            seen.push(first.class);
        }
    }
    seen.sort_by_key(|class| *class as u8);
    assert_eq!(seen, CardClass::ALL);
}

#[test]
fn changing_either_seed_changes_the_card_type_hash() {
    let generator = CardGenerator::default();
    let base = generator.generate(1, &[0; 32], &[9]).unwrap();
    let changed_set = generator.generate(1, &[1; 32], &[9]).unwrap();
    let changed_type = generator.generate(1, &[0; 32], &[10]).unwrap();
    assert_ne!(base.hash, changed_set.hash);
    assert_ne!(base.hash, changed_type.hash);
}

#[test]
fn artwork_contains_no_ascii_letters() {
    let generator = CardGenerator::default();
    for seed in 0..=255u8 {
        let card = generator.generate(1, &[0; 32], &[seed]).unwrap();
        assert!(
            card.artwork
                .iter()
                .all(|row| !row.chars().any(|c| c.is_ascii_alphabetic()))
        );
    }
}

#[test]
fn matchup_matrix_is_symmetric_and_null_is_neutral() {
    for attacker in CardClass::ALL {
        for defender in CardClass::ALL {
            let bonus = attacker.damage_bonus_against(defender);
            assert!(bonus == 0 || bonus == TYPE_ADVANTAGE_DAMAGE_BONUS);
            if bonus > 0 {
                assert_eq!(defender.weak_against(), Some(attacker));
            }
        }
        assert_eq!(attacker.damage_bonus_against(CardClass::Null), 0);
        assert_eq!(CardClass::Null.damage_bonus_against(attacker), 0);
    }
}

#[test]
fn generator_has_a_locked_version_one_vector() {
    let card = CardGenerator::default()
        .generate(1, &[0; 32], &[3])
        .unwrap();
    assert_eq!(card.class, CardClass::Robot);
    assert_eq!(card.name, "CHROMECORE");
    assert_eq!(
        hash_hex(&card.hash),
        "58de84523f07c6e1369d5ff055882d9390365bdae8ae18c72a1975b5788fbefa"
    );
}

#[test]
fn reported_bug_pair_is_not_a_near_clone() {
    let generator = CardGenerator::default();
    let first = generator.generate(1, &[0; 32], &[1]).unwrap();
    let second = generator.generate(1, &[0; 32], &[2]).unwrap();
    assert_eq!(first.class, CardClass::Bug);
    assert_eq!(second.class, CardClass::Bug);
    let mut intersection = 0usize;
    let mut union = 0usize;
    for row in 0..7 {
        let left: Vec<_> = first.artwork[row].chars().collect();
        let right: Vec<_> = second.artwork[row].chars().collect();
        for column in 0..left.len().max(right.len()) {
            let left_filled = left.get(column).is_some_and(|c| *c != ' ');
            let right_filled = right.get(column).is_some_and(|c| *c != ' ');
            intersection += usize::from(left_filled && right_filled);
            union += usize::from(left_filled || right_filled);
        }
    }
    assert!(intersection * 100 / union < 65);
}
