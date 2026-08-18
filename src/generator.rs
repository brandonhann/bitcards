use crate::{
    canonical,
    creature::{
        BugGenerator, CreatureDesign, CreatureGenerator, DaemonGenerator, GlitchGenerator,
        NullGenerator, RobotGenerator, VirusGenerator,
    },
    generator_assets::{v1::names, v2::actions as action_assets},
    hash_stream::{HashStream, SeedError},
    model::*,
};

pub const CURRENT_GENERATOR_VERSION: u32 = 2;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GenerateError {
    Seed(SeedError),
    UnsupportedVersion(u32),
    EmptyRules,
    InvalidRules,
}
impl From<SeedError> for GenerateError {
    fn from(value: SeedError) -> Self {
        Self::Seed(value)
    }
}

pub struct CardGenerator<'a> {
    rules: &'a [RarityRule],
}
impl Default for CardGenerator<'static> {
    fn default() -> Self {
        Self::new(&DEFAULT_RARITY_RULES)
    }
}

impl<'a> CardGenerator<'a> {
    pub fn new(rules: &'a [RarityRule]) -> Self {
        Self { rules }
    }
    pub fn generate(
        &self,
        version: u32,
        set_seed: &[u8],
        card_type_seed: &[u8],
    ) -> Result<CardType, GenerateError> {
        if !(1..=CURRENT_GENERATOR_VERSION).contains(&version) {
            return Err(GenerateError::UnsupportedVersion(version));
        }
        if self.rules.is_empty() {
            return Err(GenerateError::EmptyRules);
        }
        if self
            .rules
            .iter()
            .any(|r| r.selection_weight == 0 || r.maximum_supply == 0)
        {
            return Err(GenerateError::InvalidRules);
        }
        let mut stream = HashStream::new(version, set_seed, card_type_seed)?;
        let class = CardClass::ALL[index(&mut stream, CardClass::ALL.len())];
        let creature = creature_generator(class);
        let rarity = choose_rarity(&mut stream, self.rules)?;
        let design = creature.generate(&mut stream);
        let name = generate_name(&mut stream, class);
        let bonus = u16::from(rarity.rarity.stars() - 1) * 10;
        let hit_points = 50 + bonus + stream.next_bounded(5).expect("bound") as u16 * 10;
        let cost = 1 + stream.next_bounded(3).expect("bound") as u8 + rarity.rarity.stars() / 3;
        let actions = if version == 1 {
            actions_v1(&mut stream, &design, rarity.rarity, cost)
        } else {
            actions_v2(&mut stream, class, rarity.rarity, cost)
        };
        let mut card = CardType {
            generator_version: version,
            set_seed: set_seed.to_vec(),
            card_type_seed: card_type_seed.to_vec(),
            name,
            class,
            rarity: rarity.rarity,
            maximum_supply: rarity.maximum_supply,
            hit_points,
            deploy_cost: cost,
            actions,
            artwork: design.artwork,
            hash: [0; 32],
        };
        card.hash = canonical::hash(&card);
        Ok(card)
    }
}

fn creature_generator(class: CardClass) -> &'static dyn CreatureGenerator {
    match class {
        CardClass::Robot => &RobotGenerator,
        CardClass::Glitch => &GlitchGenerator,
        CardClass::Daemon => &DaemonGenerator,
        CardClass::Virus => &VirusGenerator,
        CardClass::Bug => &BugGenerator,
        CardClass::Null => &NullGenerator,
    }
}

fn index(stream: &mut HashStream, len: usize) -> usize {
    stream
        .next_bounded(len.try_into().expect("choice table too large"))
        .expect("nonzero table") as usize
}
fn choose_rarity(
    stream: &mut HashStream,
    rules: &[RarityRule],
) -> Result<RarityRule, GenerateError> {
    let total: u64 = rules.iter().map(|r| u64::from(r.selection_weight)).sum();
    let total: u32 = total.try_into().map_err(|_| GenerateError::InvalidRules)?;
    let mut roll = stream.next_bounded(total).expect("positive total");
    for rule in rules {
        if roll < rule.selection_weight {
            return Ok(*rule);
        }
        roll -= rule.selection_weight;
    }
    Err(GenerateError::InvalidRules)
}
fn generate_name(stream: &mut HashStream, class: CardClass) -> String {
    let (prefix, suffix) = names::parts(class);
    format!(
        "{}{}",
        prefix[index(stream, prefix.len())],
        suffix[index(stream, suffix.len())]
    )
}
fn actions_v1(
    stream: &mut HashStream,
    design: &CreatureDesign,
    rarity: Rarity,
    cost: u8,
) -> Vec<CardAction> {
    let base = 10 + u16::from(rarity.stars() - 1) * 5;
    let first = base + stream.next_bounded(3).unwrap() as u16 * 10;
    let second = base + 20 + stream.next_bounded(4).unwrap() as u16 * 10;
    vec![
        CardAction::Attack {
            name: design.attack_word.into(),
            damage: first,
            cost,
            effect: format!("Gain 10 damage after using {}.", design.ability_word),
        },
        CardAction::Attack {
            name: design.ability_word.into(),
            damage: second,
            cost: cost + 1,
            effect: "The foe's next attack deals 10 less damage.".into(),
        },
    ]
}

fn actions_v2(
    stream: &mut HashStream,
    class: CardClass,
    rarity: Rarity,
    deploy_cost: u8,
) -> Vec<CardAction> {
    let words = action_assets::words(class);
    let base_damage = 10 + u16::from(rarity.stars() - 1) * 5;
    let layout = stream.next_bounded(100).expect("layout bound");

    if layout < 20 {
        return vec![make_attack(
            stream,
            words.attacks,
            base_damage + 30,
            deploy_cost.saturating_add(1).min(MAXIMUM_CHARGE),
        )];
    }

    if layout < 55 {
        let ability = CardAction::Ability {
            name: words.abilities[index(stream, words.abilities.len())].into(),
            effect: action_assets::ABILITY_EFFECTS
                [index(stream, action_assets::ABILITY_EFFECTS.len())]
            .into(),
        };
        let attack = make_attack(stream, words.attacks, base_damage + 10, deploy_cost);
        return vec![ability, attack];
    }

    let first_index = index(stream, words.attacks.len());
    let mut second_index = index(stream, words.attacks.len() - 1);
    if second_index >= first_index {
        second_index += 1;
    }
    vec![
        make_attack_named(stream, words.attacks[first_index], base_damage, deploy_cost),
        make_attack_named(
            stream,
            words.attacks[second_index],
            base_damage + 20,
            deploy_cost.saturating_add(1).min(MAXIMUM_CHARGE),
        ),
    ]
}

fn make_attack(stream: &mut HashStream, names: &[&str], base_damage: u16, cost: u8) -> CardAction {
    let name = names[index(stream, names.len())];
    make_attack_named(stream, name, base_damage, cost)
}

fn make_attack_named(
    stream: &mut HashStream,
    name: &str,
    base_damage: u16,
    cost: u8,
) -> CardAction {
    CardAction::Attack {
        name: name.into(),
        damage: base_damage + stream.next_bounded(3).expect("damage bound") as u16 * 10,
        cost,
        effect: action_assets::ATTACK_EFFECTS[index(stream, action_assets::ATTACK_EFFECTS.len())]
            .into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn repeat_generation_is_byte_identical() {
        let generator = CardGenerator::default();
        let a = generator
            .generate(CURRENT_GENERATOR_VERSION, &[0; 32], &[1])
            .unwrap();
        let b = generator
            .generate(CURRENT_GENERATOR_VERSION, &[0; 32], &[1])
            .unwrap();
        assert_eq!(a, b);
        assert_eq!(canonical::serialize(&a), canonical::serialize(&b));
    }

    #[test]
    fn version_two_generates_every_action_layout() {
        let generator = CardGenerator::default();
        let mut one_attack = false;
        let mut ability_and_attack = false;
        let mut two_attacks = false;
        for seed in 0..=255u8 {
            let card = generator
                .generate(CURRENT_GENERATOR_VERSION, &[0; 32], &[seed])
                .unwrap();
            match card.actions.as_slice() {
                [CardAction::Attack { .. }] => one_attack = true,
                [CardAction::Ability { .. }, CardAction::Attack { .. }] => {
                    ability_and_attack = true;
                }
                [CardAction::Attack { .. }, CardAction::Attack { .. }] => two_attacks = true,
                actions => panic!("unexpected action layout: {actions:?}"),
            }
        }
        assert!(one_attack && ability_and_attack && two_attacks);
    }
    #[test]
    fn rarity_rules_are_configurable() {
        let rules = [RarityRule {
            rarity: Rarity::SuperRare,
            selection_weight: 1,
            maximum_supply: 7,
        }];
        let card = CardGenerator::new(&rules)
            .generate(1, &[0; 32], &[1])
            .unwrap();
        assert_eq!(card.rarity, Rarity::SuperRare);
        assert_eq!(card.maximum_supply, 7);
    }
    #[test]
    fn unsupported_version_is_rejected() {
        assert_eq!(
            CardGenerator::default()
                .generate(CURRENT_GENERATOR_VERSION + 1, &[1], &[1])
                .unwrap_err(),
            GenerateError::UnsupportedVersion(CURRENT_GENERATOR_VERSION + 1)
        );
    }
}
