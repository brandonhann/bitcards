#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Rarity {
    Common = 1,
    Uncommon,
    Rare,
    Legendary,
    Mythic,
}

impl Rarity {
    pub const fn stars(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CardClass {
    Robot = 1,
    Glitch,
    Daemon,
    Virus,
    Bug,
    Null,
}

impl CardClass {
    pub const ALL: [Self; 6] = [
        Self::Robot,
        Self::Glitch,
        Self::Daemon,
        Self::Virus,
        Self::Bug,
        Self::Null,
    ];
    pub const fn name(self) -> &'static str {
        match self {
            Self::Robot => "Robot",
            Self::Glitch => "Glitch",
            Self::Daemon => "Daemon",
            Self::Virus => "Virus",
            Self::Bug => "Bug",
            Self::Null => "Null",
        }
    }
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::Robot => "[]",
            Self::Glitch => "//",
            Self::Daemon => "<>",
            Self::Virus => "**",
            Self::Bug => "{}",
            Self::Null => "()",
        }
    }
    pub const fn strong_against(self) -> Option<Self> {
        match self {
            Self::Robot => Some(Self::Virus),
            Self::Virus => Some(Self::Daemon),
            Self::Daemon => Some(Self::Glitch),
            Self::Glitch => Some(Self::Bug),
            Self::Bug => Some(Self::Robot),
            Self::Null => None,
        }
    }
    pub const fn weak_against(self) -> Option<Self> {
        match self {
            Self::Robot => Some(Self::Bug),
            Self::Virus => Some(Self::Robot),
            Self::Daemon => Some(Self::Virus),
            Self::Glitch => Some(Self::Daemon),
            Self::Bug => Some(Self::Glitch),
            Self::Null => None,
        }
    }
    pub const fn damage_bonus_against(self, defender: Self) -> u16 {
        if matches!(self.strong_against(), Some(target) if target as u8 == defender as u8) {
            TYPE_ADVANTAGE_DAMAGE_BONUS
        } else {
            0
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attack {
    pub name: String,
    pub damage: u16,
    pub cost: u8,
    pub effect: String,
}

pub const CHARGE_GAIN_PER_TURN: u8 = 1;
pub const MAXIMUM_CHARGE: u8 = 6;
pub const TYPE_ADVANTAGE_DAMAGE_BONUS: u16 = 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RarityRule {
    pub rarity: Rarity,
    pub selection_weight: u32,
    pub maximum_supply: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CardType {
    pub generator_version: u32,
    pub set_seed: Vec<u8>,
    pub card_type_seed: Vec<u8>,
    pub name: String,
    pub class: CardClass,
    pub rarity: Rarity,
    pub maximum_supply: u32,
    pub hit_points: u16,
    pub deploy_cost: u8,
    pub attacks: Vec<Attack>,
    pub artwork: Vec<String>,
    pub hash: [u8; 32],
}

pub const VERSION_1_RARITY_RULES: [RarityRule; 5] = [
    RarityRule {
        rarity: Rarity::Common,
        selection_weight: 5_800,
        maximum_supply: 10_000,
    },
    RarityRule {
        rarity: Rarity::Uncommon,
        selection_weight: 2_700,
        maximum_supply: 4_000,
    },
    RarityRule {
        rarity: Rarity::Rare,
        selection_weight: 1_100,
        maximum_supply: 1_000,
    },
    RarityRule {
        rarity: Rarity::Legendary,
        selection_weight: 350,
        maximum_supply: 200,
    },
    RarityRule {
        rarity: Rarity::Mythic,
        selection_weight: 50,
        maximum_supply: 25,
    },
];

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rarity_is_protocol_meaningful() {
        for pair in VERSION_1_RARITY_RULES.windows(2) {
            assert!(pair[0].selection_weight > pair[1].selection_weight);
            assert!(pair[0].maximum_supply > pair[1].maximum_supply);
        }
    }
    #[test]
    fn specialized_classes_form_a_fair_closed_loop() {
        for class in CardClass::ALL
            .into_iter()
            .filter(|class| *class != CardClass::Null)
        {
            let target = class.strong_against().unwrap();
            assert_eq!(target.weak_against(), Some(class));
            assert_eq!(class.damage_bonus_against(target), 10);
        }
        assert_eq!(CardClass::Null.strong_against(), None);
        assert_eq!(CardClass::Null.weak_against(), None);
    }
}
