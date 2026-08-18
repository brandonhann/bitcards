//! Draft-only Command and Charge card previews.
//!
//! Nothing in this module is canonical protocol data.

use crate::model::CardClass;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportKind {
    QuickPatch,
    Purge,
    Download,
    IndexSearch,
    HotSwap,
    ForceRoute,
    Firewall,
    SafeMode,
    RebootCharge,
    BurstCharge,
    RobotCharge,
    GlitchCharge,
    DaemonCharge,
    VirusCharge,
    BugCharge,
    NullCharge,
}

impl SupportKind {
    pub const COMMANDS: [Self; 10] = [
        Self::QuickPatch,
        Self::Purge,
        Self::Download,
        Self::IndexSearch,
        Self::HotSwap,
        Self::ForceRoute,
        Self::Firewall,
        Self::SafeMode,
        Self::RebootCharge,
        Self::BurstCharge,
    ];

    pub const CHARGES: [Self; 6] = [
        Self::RobotCharge,
        Self::GlitchCharge,
        Self::DaemonCharge,
        Self::VirusCharge,
        Self::BugCharge,
        Self::NullCharge,
    ];

    pub const fn is_charge(self) -> bool {
        self.charge_class().is_some()
    }

    pub const fn charge_class(self) -> Option<CardClass> {
        match self {
            Self::RobotCharge => Some(CardClass::Robot),
            Self::GlitchCharge => Some(CardClass::Glitch),
            Self::DaemonCharge => Some(CardClass::Daemon),
            Self::VirusCharge => Some(CardClass::Virus),
            Self::BugCharge => Some(CardClass::Bug),
            Self::NullCharge => Some(CardClass::Null),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupportPreview {
    pub kind: SupportKind,
    pub name: &'static str,
    pub effect: &'static str,
    pub artwork: Vec<String>,
}

pub fn generate(kind: SupportKind) -> SupportPreview {
    let (name, effect) = match kind {
        SupportKind::QuickPatch => ("QUICK PATCH", "Heal 20 damage from your Active BitCard."),
        SupportKind::Purge => (
            "PURGE",
            "Remove one harmful effect from your Active BitCard.",
        ),
        SupportKind::Download => ("DOWNLOAD", "Draw two cards."),
        SupportKind::IndexSearch => (
            "INDEX SEARCH",
            "Look at the top three cards of your deck. Put one in your hand and the rest on the bottom in any order.",
        ),
        SupportKind::HotSwap => (
            "HOT SWAP",
            "Switch your Active BitCard with one of your Benched BitCards.",
        ),
        SupportKind::ForceRoute => (
            "FORCE ROUTE",
            "If possible, your opponent switches their Active BitCard with a Benched BitCard of their choice.",
        ),
        SupportKind::Firewall => (
            "FIREWALL",
            "The next attack against your Active BitCard during your opponent's next turn deals 20 less damage.",
        ),
        SupportKind::SafeMode => (
            "SAFE MODE",
            "Your Active BitCard cannot be switched by your opponent until your next turn.",
        ),
        SupportKind::RebootCharge => ("REBOOT CHARGE", "Refresh one exhausted Charge."),
        SupportKind::BurstCharge => (
            "BURST CHARGE",
            "Your next attack this turn costs one less Charge, to a minimum of one.",
        ),
        SupportKind::RobotCharge => ("ROBOT CHARGE", "Provides Robot Charge."),
        SupportKind::GlitchCharge => ("GLITCH CHARGE", "Provides Glitch Charge."),
        SupportKind::DaemonCharge => ("DAEMON CHARGE", "Provides Daemon Charge."),
        SupportKind::VirusCharge => ("VIRUS CHARGE", "Provides Virus Charge."),
        SupportKind::BugCharge => ("BUG CHARGE", "Provides Bug Charge."),
        SupportKind::NullCharge => ("NULL CHARGE", "Provides neutral Charge."),
    };
    let artwork = if let Some(class) = kind.charge_class() {
        charge_symbol_art(class)
    } else {
        command_terminal_art(kind)
    };
    SupportPreview {
        kind,
        name,
        effect,
        artwork,
    }
}

fn command_terminal_art(kind: SupportKind) -> Vec<String> {
    let rows = match kind {
        SupportKind::QuickPatch => [
            "┌───────────────────────────┐",
            "│ > patch --active          │",
            "│ scanning damage...        │",
            "│ [██████████████████] 100% │",
            "│ damage repaired: 20       │",
            "│ status: COMPLETE          │",
            "└───────────────────────────┘",
        ],
        SupportKind::Purge => [
            "┌───────────────────────────┐",
            "│ > purge --active          │",
            "│ locating harmful effect...│",
            "│ effect removed            │",
            "│ integrity check: PASS     │",
            "│ status: CLEAN             │",
            "└───────────────────────────┘",
        ],
        SupportKind::Download => [
            "┌───────────────────────────┐",
            "│ > fetch --cards 2         │",
            "│ searching data cache...   │",
            "│ [██████████████████] 100% │",
            "│ cards received: 2         │",
            "│ status: COMPLETE          │",
            "└───────────────────────────┘",
        ],
        SupportKind::IndexSearch => [
            "┌───────────────────────────┐",
            "│ > index --top 3           │",
            "│ scanning deck index...    │",
            "│ results: [1] [2] [3]      │",
            "│ selection: READY          │",
            "│ status: WAITING           │",
            "└───────────────────────────┘",
        ],
        SupportKind::HotSwap => [
            "┌───────────────────────────┐",
            "│ > swap active bench-1     │",
            "│ mapping card slots...     │",
            "│ ACTIVE <-----> BENCH-1    │",
            "│ positions updated         │",
            "│ status: COMPLETE          │",
            "└───────────────────────────┘",
        ],
        SupportKind::ForceRoute => [
            "┌───────────────────────────┐",
            "│ > route --opponent        │",
            "│ requesting new target...  │",
            "│ ACTIVE -----> BENCH       │",
            "│ opponent must select      │",
            "│ status: PENDING           │",
            "└───────────────────────────┘",
        ],
        SupportKind::Firewall => [
            "┌───────────────────────────┐",
            "│ > firewall --enable       │",
            "│ filtering incoming...     │",
            "│ blocked: ████████████████ │",
            "│ active card protected     │",
            "│ status: ONLINE            │",
            "└───────────────────────────┘",
        ],
        SupportKind::SafeMode => [
            "┌───────────────────────────┐",
            "│ > safe-mode --active      │",
            "│ locking active slot...    │",
            "│ external swaps: DENIED    │",
            "│ protection: ONE TURN      │",
            "│ status: LOCKED            │",
            "└───────────────────────────┘",
        ],
        SupportKind::RebootCharge => [
            "┌───────────────────────────┐",
            "│ > reboot --charge         │",
            "│ cycling charge state...   │",
            "│ EXHAUSTED -----> READY    │",
            "│ charge refreshed          │",
            "│ status: ONLINE            │",
            "└───────────────────────────┘",
        ],
        SupportKind::BurstCharge => [
            "┌───────────────────────────┐",
            "│ > burst --next-attack     │",
            "│ lowering charge cost...   │",
            "│ cost modifier: -1         │",
            "│ minimum attack cost: 1    │",
            "│ status: READY             │",
            "└───────────────────────────┘",
        ],
        _ => unreachable!("Charge cards use symbol artwork"),
    };
    rows.into_iter().map(str::to_owned).collect()
}

fn charge_symbol_art(class: CardClass) -> Vec<String> {
    class.symbol().chars().map(charge_symbol_glyph).fold(
        vec![String::new(); 7],
        |mut rows, glyph| {
            for (row, segment) in rows.iter_mut().zip(glyph) {
                if !row.is_empty() {
                    row.push_str("   ");
                }
                row.push_str(segment);
            }
            rows
        },
    )
}

fn charge_symbol_glyph(character: char) -> [&'static str; 7] {
    match character {
        '[' => [
            "██████   ",
            "██       ",
            "██       ",
            "██       ",
            "██       ",
            "██       ",
            "██████   ",
        ],
        ']' => [
            "   ██████",
            "       ██",
            "       ██",
            "       ██",
            "       ██",
            "       ██",
            "   ██████",
        ],
        '/' => [
            "       ██",
            "      ██ ",
            "     ██  ",
            "    ██   ",
            "   ██    ",
            "  ██     ",
            " ██      ",
        ],
        '<' => [
            "      ██ ",
            "    ██   ",
            "  ██     ",
            "██       ",
            "  ██     ",
            "    ██   ",
            "      ██ ",
        ],
        '>' => [
            " ██      ",
            "   ██    ",
            "     ██  ",
            "       ██",
            "     ██  ",
            "   ██    ",
            " ██      ",
        ],
        '*' => [
            "█   █   █",
            " █  █  █ ",
            "  █████  ",
            "█████████",
            "  █████  ",
            " █  █  █ ",
            "█   █   █",
        ],
        '{' => [
            "    █████",
            "  ██     ",
            "  ██     ",
            "██       ",
            "  ██     ",
            "  ██     ",
            "    █████",
        ],
        '}' => [
            "█████    ",
            "     ██  ",
            "     ██  ",
            "       ██",
            "     ██  ",
            "     ██  ",
            "█████    ",
        ],
        '(' => [
            "     ███ ",
            "   ██    ",
            "  ██     ",
            " ██      ",
            "  ██     ",
            "   ██    ",
            "     ███ ",
        ],
        ')' => [
            " ███     ",
            "    ██   ",
            "     ██  ",
            "      ██ ",
            "     ██  ",
            "    ██   ",
            " ███     ",
        ],
        _ => panic!("unsupported Charge symbol character {character}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn previews_are_deterministic_and_cover_every_kind() {
        for kind in SupportKind::COMMANDS
            .into_iter()
            .chain(SupportKind::CHARGES)
        {
            let first = generate(kind);
            assert_eq!(first, generate(kind));
            assert_eq!(first.artwork.len(), 7);
            if kind.is_charge() {
                assert!(first.artwork.iter().all(|row| row.chars().count() == 21));
            } else {
                assert!(first.artwork.iter().all(|row| row.chars().count() <= 32));
            }
        }
    }

    #[test]
    fn each_charge_type_has_one_fixed_design() {
        for kind in SupportKind::CHARGES {
            assert_eq!(generate(kind), generate(kind));
        }
    }

    #[test]
    fn all_ten_commands_are_fixed_and_uniquely_named() {
        let mut names = Vec::new();
        for kind in SupportKind::COMMANDS {
            let first = generate(kind);
            assert_eq!(first, generate(kind));
            assert!(!names.contains(&first.name));
            names.push(first.name);
        }
        assert_eq!(names.len(), 10);
    }
}
