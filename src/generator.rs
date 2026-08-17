use crate::{
    canonical,
    creature::{
        BugGenerator, CreatureDesign, CreatureGenerator, DaemonGenerator, GlitchGenerator,
        NullGenerator, RobotGenerator, VirusGenerator,
    },
    generator_assets::v1::names,
    hash_stream::{HashStream, SeedError},
    model::*,
};

pub const CURRENT_GENERATOR_VERSION: u32 = 1;

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
    creatures: Vec<Box<dyn CreatureGenerator>>,
}
impl Default for CardGenerator<'static> {
    fn default() -> Self {
        Self::new(&VERSION_1_RARITY_RULES)
    }
}

impl<'a> CardGenerator<'a> {
    pub fn new(rules: &'a [RarityRule]) -> Self {
        Self {
            rules,
            creatures: vec![
                Box::new(RobotGenerator),
                Box::new(GlitchGenerator),
                Box::new(DaemonGenerator),
                Box::new(VirusGenerator),
                Box::new(BugGenerator),
                Box::new(NullGenerator),
            ],
        }
    }
    pub fn with_creatures(mut self, creatures: Vec<Box<dyn CreatureGenerator>>) -> Self {
        self.creatures = creatures;
        self
    }
    pub fn generate(
        &self,
        version: u32,
        set_seed: &[u8],
        card_type_seed: &[u8],
    ) -> Result<CardType, GenerateError> {
        if version != CURRENT_GENERATOR_VERSION {
            return Err(GenerateError::UnsupportedVersion(version));
        }
        if self.rules.is_empty() || self.creatures.is_empty() {
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
        let drawing_index = usize::from(class as u8 - 1);
        let creature = &self.creatures[drawing_index];
        let rarity = choose_rarity(&mut stream, self.rules)?;
        let design = creature.generate(&mut stream);
        let name = generate_name(&mut stream, class);
        let bonus = u16::from(rarity.rarity.stars() - 1) * 10;
        let hit_points = 50 + bonus + stream.next_bounded(5).expect("bound") as u16 * 10;
        let cost = 1 + stream.next_bounded(3).expect("bound") as u8 + rarity.rarity.stars() / 3;
        let attacks = attacks(&mut stream, &design, rarity.rarity, cost);
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
            attacks,
            artwork: design.artwork,
            hash: [0; 32],
        };
        card.hash = canonical::hash(&card);
        Ok(card)
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
fn attacks(
    stream: &mut HashStream,
    design: &CreatureDesign,
    rarity: Rarity,
    cost: u8,
) -> Vec<Attack> {
    let base = 10 + u16::from(rarity.stars() - 1) * 5;
    let first = base + stream.next_bounded(3).unwrap() as u16 * 10;
    let second = base + 20 + stream.next_bounded(4).unwrap() as u16 * 10;
    vec![
        Attack {
            name: design.attack_word.into(),
            damage: first,
            cost,
            effect: format!("Gain 10 damage after using {}.", design.ability_word),
        },
        Attack {
            name: design.ability_word.into(),
            damage: second,
            cost: cost + 1,
            effect: "The foe's next attack deals 10 less damage.".into(),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn repeat_generation_is_byte_identical() {
        let generator = CardGenerator::default();
        let a = generator.generate(1, &[0; 32], &[1]).unwrap();
        let b = generator.generate(1, &[0; 32], &[1]).unwrap();
        assert_eq!(a, b);
        assert_eq!(canonical::serialize(&a), canonical::serialize(&b));
    }
    #[test]
    fn rarity_rules_are_configurable() {
        let rules = [RarityRule {
            rarity: Rarity::Mythic,
            selection_weight: 1,
            maximum_supply: 7,
        }];
        let card = CardGenerator::new(&rules)
            .generate(1, &[0; 32], &[1])
            .unwrap();
        assert_eq!(card.rarity, Rarity::Mythic);
        assert_eq!(card.maximum_supply, 7);
    }
    #[test]
    fn unsupported_version_is_rejected() {
        assert_eq!(
            CardGenerator::default()
                .generate(2, &[1], &[1])
                .unwrap_err(),
            GenerateError::UnsupportedVersion(2)
        );
    }
}
