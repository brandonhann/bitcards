use crate::model::CardClass;

pub struct ActionWords {
    pub attacks: &'static [&'static str],
    pub abilities: &'static [&'static str],
}

const ROBOT_ATTACKS: &[&str] = &[
    "Servo Slam",
    "Pulse Cannon",
    "Steel Ram",
    "Arc Punch",
    "Gear Grind",
    "Laser Sweep",
    "Chrome Crush",
    "Circuit Bash",
];
const ROBOT_ABILITIES: &[&str] = &[
    "Reboot",
    "Overclock",
    "Auto-Repair",
    "Target Lock",
    "Hard Reset",
    "Armor Cache",
];
const GLITCH_ATTACKS: &[&str] = &[
    "Pixel Tear",
    "Frame Skip",
    "Bit Fracture",
    "Desync",
    "Static Cut",
    "Raster Spike",
    "Signal Break",
    "Screen Burn",
];
const GLITCH_ABILITIES: &[&str] = &[
    "Rollback",
    "Corrupt",
    "Packet Loss",
    "Phase Shift",
    "Buffer Skip",
    "False Frame",
];
const DAEMON_ATTACKS: &[&str] = &[
    "Shadow Fork",
    "Root Claw",
    "Hex Pulse",
    "Spectral Ping",
    "Kernel Haunt",
    "Dark Process",
    "Phantom Call",
    "Night Thread",
];
const DAEMON_ABILITIES: &[&str] = &[
    "Persist",
    "Background Task",
    "Root Access",
    "Silent Watch",
    "Hidden Port",
    "Night Service",
];
const VIRUS_ATTACKS: &[&str] = &[
    "Code Splice",
    "Payload",
    "Infect",
    "Byte Rot",
    "Trojan Bite",
    "Worm Strike",
    "Memory Leak",
    "Mutation",
];
const VIRUS_ABILITIES: &[&str] = &[
    "Replicate",
    "Quarantine",
    "Exploit",
    "Incubate",
    "Backdoor",
    "Rapid Spread",
];
const BUG_ATTACKS: &[&str] = &[
    "Pincer Crash",
    "Swarm",
    "Shell Bash",
    "Stack Crawl",
    "Antenna Jab",
    "Wing Buzz",
    "Hard Lock",
    "Syntax Sting",
];
const BUG_ABILITIES: &[&str] = &[
    "Burrow",
    "Patch Dodge",
    "Crash Loop",
    "Hide in Code",
    "Shell Guard",
    "Debug",
];
const NULL_ATTACKS: &[&str] = &[
    "Zero Out",
    "Blank",
    "Void Pulse",
    "Erase",
    "Dead Signal",
    "Empty Hit",
    "Cold Boot",
    "Quiet End",
];
const NULL_ABILITIES: &[&str] = &[
    "Silence",
    "Vacuum",
    "Nothingness",
    "Zero State",
    "No Response",
    "Empty Slot",
];

pub const ATTACK_EFFECTS: &[&str] = &[
    "Add 10 damage when this class has advantage.",
    "The foe's next attack deals 10 less damage.",
    "Deal 10 more if this was Benched last turn.",
    "Deal 10 more when this card has 30 HP or less.",
    "Move this card to the Bench after attacking.",
    "The foe cannot retreat on its next turn.",
    "Your next attack costs 1 less Charge.",
    "Discard 1 stored Charge after this attack.",
];

pub const ABILITY_EFFECTS: &[&str] = &[
    "Once per turn, reduce the next hit by 10.",
    "On deploy, gain 1 Charge.",
    "While Active, retreat costs 1 less Charge.",
    "When Benched, this card recovers 10 HP once.",
    "On knockout, your next card gains 1 Charge.",
    "The first attack against this card deals 10 less.",
    "Once per battle, move this card to the Bench.",
    "When deployed, reveal the foe's top card.",
];

pub const fn words(class: CardClass) -> ActionWords {
    match class {
        CardClass::Robot => ActionWords {
            attacks: ROBOT_ATTACKS,
            abilities: ROBOT_ABILITIES,
        },
        CardClass::Glitch => ActionWords {
            attacks: GLITCH_ATTACKS,
            abilities: GLITCH_ABILITIES,
        },
        CardClass::Daemon => ActionWords {
            attacks: DAEMON_ATTACKS,
            abilities: DAEMON_ABILITIES,
        },
        CardClass::Virus => ActionWords {
            attacks: VIRUS_ATTACKS,
            abilities: VIRUS_ABILITIES,
        },
        CardClass::Bug => ActionWords {
            attacks: BUG_ATTACKS,
            abilities: BUG_ABILITIES,
        },
        CardClass::Null => ActionWords {
            attacks: NULL_ATTACKS,
            abilities: NULL_ABILITIES,
        },
    }
}
