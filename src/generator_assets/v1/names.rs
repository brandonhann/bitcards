use crate::model::CardClass;

pub const fn parts(class: CardClass) -> (&'static [&'static str], &'static [&'static str]) {
    match class {
        CardClass::Robot => (
            &["BYTE", "VOLT", "CHROME", "NANO"],
            &["BOT", "CORE", "DROID", "UNIT"],
        ),
        CardClass::Glitch => (
            &["STATIC", "BROKEN", "JITTER", "WARP"],
            &["SHIFT", "NOISE", "TEAR", "ECHO"],
        ),
        CardClass::Daemon => (
            &["DARK", "SILENT", "NIGHT", "HIDDEN"],
            &["WATCH", "WRAITH", "SHADE", "WARD"],
        ),
        CardClass::Virus => (
            &["SPORE", "MUTANT", "FATAL", "RED"],
            &["WORM", "PLAGUE", "STRAIN", "BITE"],
        ),
        CardClass::Bug => (
            &["TINY", "STACK", "CODE", "CACHE"],
            &["MITE", "BEETLE", "CRAWLER", "MANTIS"],
        ),
        CardClass::Null => (
            &["ZERO", "EMPTY", "PALE", "VOID"],
            &["FORM", "ECHO", "SHADE", "THING"],
        ),
    }
}
