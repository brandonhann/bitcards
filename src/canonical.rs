use crate::{
    model::{CardAction, CardType},
    sha256::Sha256,
};

const DOMAIN_V1: &[u8] = b"BITCARDS:CARDTYPE:V1";
const DOMAIN_V2: &[u8] = b"BITCARDS:CARDTYPE:V2";

#[must_use]
pub fn serialize(card: &CardType) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(match card.generator_version {
        1 => DOMAIN_V1,
        2 => DOMAIN_V2,
        version => panic!("unsupported canonical Card Type version {version}"),
    });
    u32_value(&mut out, card.generator_version);
    bytes(&mut out, &card.set_seed);
    bytes(&mut out, &card.card_type_seed);
    text(&mut out, &card.name);
    out.push(card.class as u8);
    out.push(card.rarity as u8);
    u32_value(&mut out, card.maximum_supply);
    out.extend_from_slice(&card.hit_points.to_be_bytes());
    out.push(card.deploy_cost);
    u32_value(
        &mut out,
        card.actions.len().try_into().expect("too many actions"),
    );
    for action in &card.actions {
        if card.generator_version == 1 {
            let CardAction::Attack {
                name,
                damage,
                cost,
                effect,
            } = action
            else {
                panic!("generator v1 cannot encode abilities");
            };
            text(&mut out, name);
            out.extend_from_slice(&damage.to_be_bytes());
            out.push(*cost);
            text(&mut out, effect);
            continue;
        }
        match action {
            CardAction::Attack {
                name,
                damage,
                cost,
                effect,
            } => {
                out.push(1);
                text(&mut out, name);
                out.extend_from_slice(&damage.to_be_bytes());
                out.push(*cost);
                text(&mut out, effect);
            }
            CardAction::Ability { name, effect } => {
                out.push(2);
                text(&mut out, name);
                text(&mut out, effect);
            }
        }
    }
    u32_value(
        &mut out,
        card.artwork.len().try_into().expect("too many art rows"),
    );
    for row in &card.artwork {
        text(&mut out, row);
    }
    out
}

#[must_use]
pub fn hash(card: &CardType) -> [u8; 32] {
    Sha256::digest(&serialize(card))
}
#[must_use]
pub fn hash_hex(hash: &[u8; 32]) -> String {
    hash.iter().map(|byte| format!("{byte:02x}")).collect()
}
fn u32_value(out: &mut Vec<u8>, value: u32) {
    out.extend_from_slice(&value.to_be_bytes());
}
fn bytes(out: &mut Vec<u8>, value: &[u8]) {
    u32_value(out, value.len().try_into().expect("field too large"));
    out.extend_from_slice(value);
}
fn text(out: &mut Vec<u8>, value: &str) {
    bytes(out, value.as_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::*;
    fn sample() -> CardType {
        CardType {
            generator_version: 1,
            set_seed: vec![1],
            card_type_seed: vec![2],
            name: "TEST".into(),
            class: CardClass::Glitch,
            rarity: Rarity::Rare,
            maximum_supply: 9,
            hit_points: 80,
            deploy_cost: 2,
            actions: vec![CardAction::Attack {
                name: "Splat".into(),
                damage: 20,
                cost: 1,
                effect: "None".into(),
            }],
            artwork: vec!["(o)".into()],
            hash: [0; 32],
        }
    }
    #[test]
    fn hash_excludes_hash_field_but_covers_semantics() {
        let card = sample();
        let original = hash(&card);
        let mut hash_only = card.clone();
        hash_only.hash = [9; 32];
        assert_eq!(original, hash(&hash_only));
        let mut changed = card;
        changed.hit_points += 10;
        assert_ne!(original, hash(&changed));
    }
    #[test]
    fn encoding_has_fixed_vector() {
        assert_eq!(
            hash_hex(&hash(&sample())),
            "f5b1c74f59ad227fa9e01f70381dc480c35aa64ba9aecc00c9dc3bf108428a5d"
        );
    }
}
