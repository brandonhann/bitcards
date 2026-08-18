//! Fixed, hand-authored promotional card previews.
//!
//! These definitions are gameplay and visual drafts, not activated protocol cards.

use crate::{
    canonical,
    model::{CardAction, CardClass, CardType, Rarity},
};

pub const PROMO_COPY_SUPPLY: u32 = 1_000;

#[must_use]
pub fn set_one_promos() -> Vec<CardType> {
    vec![netdoge()]
}

fn netdoge() -> CardType {
    let mut card = CardType {
        generator_version: 2,
        set_seed: b"BITCARDS:PROMO:SET:01".to_vec(),
        card_type_seed: b"PROMO:01:NETDOGE".to_vec(),
        name: "NETDOGE".into(),
        class: CardClass::Null,
        rarity: Rarity::Common,
        maximum_supply: PROMO_COPY_SUPPLY,
        hit_points: 80,
        deploy_cost: 2,
        actions: vec![
            CardAction::Ability {
                name: "MUCH WOW".into(),
                effect: "When you deploy this BitCard, draw one card.".into(),
            },
            CardAction::Attack {
                name: "MOON BARK".into(),
                damage: 30,
                cost: 2,
                effect: "No added effect.".into(),
            },
        ],
        artwork: [
            "         ▄              ▄    ",
            "        ▌▒█           ▄▀▒▌   ",
            "        ▌▒▒█        ▄▀▒▒▒▐   ",
            "       ▐▄█▒▒▀▀▀▀▄▄▄▀▒▒▒▒▒▐   ",
            "     ▄▄▀▒▒▒▒▒▒▒▒▒▒▒█▒▒▄█▒▐   ",
            "   ▄▀▒▒▒░░░▒▒▒░░░▒▒▒▀██▀▒▌   ",
            "  ▐▒▒▒▄▄▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▀▄▒▌  ",
            "  ▌░░▌█▀▒▒▒▒▒▄▀█▄▒▒▒▒▒▒▒█▒▐  ",
            " ▐░░░▒▒▒▒▒▒▒▒▌██▀▒▒░░░▒▒▒▀▄▌ ",
            " ▌░▒▒▒▒▒▒▒▒▒▒▒▒▒▒░░░░░░▒▒▒▒▌ ",
            "▌▒▒▒▄██▄▒▒▒▒▒▒▒▒░░░░░░░░▒▒▒▐ ",
            "▐▒▒▐▄█▄█▌▒▒▒▒▒▒▒▒▒▒░▒░▒░▒▒▒▒▌",
            "▐▒▒▐▀▐▀▒▒▒▒▒▒▒▒▒▒▒▒▒░▒░▒░▒▒▐ ",
            " ▌▒▒▀▄▄▄▄▄▄▒▒▒▒▒▒▒▒░▒░▒░▒▒▒▌ ",
            " ▐▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒░▒░▒▒▄▒▒▐  ",
            "  ▀▄▒▒▒▒▒▒▒▒▒▒▒▒▒░▒░▒▄▒▒▒▒▌  ",
            "    ▀▄▒▒▒▒▒▒▒▒▒▒▄▄▄▀▒▒▒▒▄▀   ",
            "      ▀▄▄▄▄▄▄▀▀▀▒▒▒▒▒▄▄▀     ",
            "         ▀▀▀▀▀▀▀▀▀▀▀▀        ",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect(),
        hash: [0; 32],
    };
    card.hash = canonical::hash(&card);
    card
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn netdoge_is_fixed_balanced_and_supply_capped() {
        let first = set_one_promos();
        let second = set_one_promos();
        assert_eq!(first, second);
        assert_eq!(first.len(), 1);
        let card = &first[0];
        assert_eq!(card.name, "NETDOGE");
        assert_eq!(card.class, CardClass::Null);
        assert_eq!(card.maximum_supply, 1_000);
        assert_eq!(card.artwork.len(), 19);
        assert_eq!(card.actions.len(), 2);
        assert_ne!(card.hash, [0; 32]);
    }
}
