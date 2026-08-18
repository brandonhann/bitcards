use crate::model::{CardAction, CardClass, CardType};
use crate::support_preview::{SupportKind, SupportPreview};
/// Display dimensions are deliberately separate from canonical Card Type data.
pub const INNER_WIDTH: usize = 32;
pub const ART_HEIGHT: usize = 7;
pub const CARD_HEIGHT: usize = 24;
pub const MAX_DISPLAY_SET_ID: u32 = 99;
pub const MAX_DISPLAY_SERIAL: u32 = 999_999;
pub const MAX_DISPLAY_CATALOG_SIZE: u16 = 999;

/// Display-only copy finish for fixed support cards.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportFinish {
    Standard,
    Rare,
    SuperRare,
}

impl SupportFinish {
    const fn stars(self) -> &'static str {
        match self {
            Self::Standard => "★",
            Self::Rare => "★★",
            Self::SuperRare => "★★★",
        }
    }
}

#[must_use]
pub fn render_support_preview(card: &SupportPreview, colored: bool) -> String {
    render_support_preview_with_identity(card, colored, 1, 1, 1)
}

#[must_use]
pub fn render_support_preview_with_identity(
    card: &SupportPreview,
    colored: bool,
    catalog_number: u16,
    set_id: u32,
    serial: u32,
) -> String {
    render_support_preview_with_identity_and_finish(
        card,
        colored,
        catalog_number,
        set_id,
        serial,
        SupportFinish::Standard,
    )
}

#[must_use]
pub fn render_support_preview_with_identity_and_finish(
    card: &SupportPreview,
    colored: bool,
    catalog_number: u16,
    set_id: u32,
    serial: u32,
    finish: SupportFinish,
) -> String {
    assert!((1..=MAX_DISPLAY_CATALOG_SIZE).contains(&catalog_number));
    assert!((1..=MAX_DISPLAY_SET_ID).contains(&set_id));
    assert!((1..=MAX_DISPLAY_SERIAL).contains(&serial));
    let identity = footer(
        &format!("{catalog_number:03} {}", finish.stars()),
        &format!("SET {set_id:02}"),
        &format!("#{serial:06}"),
    );

    if card.kind.is_charge() {
        return render_charge_preview(card, colored, &identity, finish);
    }

    let mut rows = Vec::with_capacity(22);
    rows.push(columns(card.name, "COMMAND"));
    rows.push("─".repeat(INNER_WIDTH));
    rows.push(String::new());
    let artwork = command_artwork(&card.artwork, finish);
    rows.extend(center_artwork(&artwork));
    rows.push(String::new());
    rows.push("─".repeat(INNER_WIDTH));
    rows.push("EFFECT".into());
    rows.extend(wrap(card.effect));
    rows.resize(21, String::new());
    rows.push(identity);

    render_support_rows(card.kind, colored, rows, false, finish)
}

fn render_charge_preview(
    card: &SupportPreview,
    colored: bool,
    identity: &str,
    finish: SupportFinish,
) -> String {
    let class = card.kind.charge_class().expect("Charge card has a class");
    let mut rows = Vec::with_capacity(22);
    rows.push(columns(card.name, class.symbol()));
    rows.push("─".repeat(INNER_WIDTH));
    let padding = 19usize.saturating_sub(card.artwork.len());
    rows.extend((0..padding / 2).map(|_| String::new()));
    rows.extend(center_artwork(&card.artwork));
    rows.extend((0..padding.div_ceil(2)).map(|_| String::new()));
    rows.push(identity.into());
    render_support_rows(card.kind, colored, rows, true, finish)
}

fn command_artwork(artwork: &[String], finish: SupportFinish) -> Vec<String> {
    if finish != SupportFinish::SuperRare {
        return artwork.to_vec();
    }
    artwork
        .iter()
        .map(|row| {
            row.chars()
                .map(|character| match character {
                    '┌' => '╔',
                    '┐' => '╗',
                    '└' => '╚',
                    '┘' => '╝',
                    '─' => '═',
                    '│' => '║',
                    '>' => '#',
                    _ => character,
                })
                .collect()
        })
        .collect()
}

fn render_support_rows(
    kind: SupportKind,
    colored: bool,
    rows: Vec<String>,
    full_art: bool,
    finish: SupportFinish,
) -> String {
    let color = support_border_color();
    let mut lines = Vec::with_capacity(CARD_HEIGHT);
    lines.push(format!("╔{}╗", "═".repeat(INNER_WIDTH)));
    for (row_index, row) in rows.into_iter().take(22).enumerate() {
        if row == "─".repeat(INNER_WIDTH) {
            if colored {
                lines.push(format!(
                    "\x1b[48;5;16m{color}╟{}╢\x1b[0m",
                    "─".repeat(INNER_WIDTH)
                ));
            } else {
                lines.push(format!("╟{}╢", "─".repeat(INNER_WIDTH)));
            }
            continue;
        }
        let row = fixed_width(&row);
        if colored {
            let content_style = if full_art && (2..=20).contains(&row_index) {
                charge_art_style(kind, finish, row_index)
            } else if !full_art && (2..=10).contains(&row_index) {
                command_art_style(finish, row_index)
            } else if !full_art {
                command_panel_style(kind, finish)
            } else if full_art {
                charge_panel_style(kind, finish)
            } else {
                support_panel_style(kind)
            };
            lines.push(format!(
                "\x1b[48;5;16m{color}║\x1b[0m{content_style}{row}\x1b[0m\x1b[48;5;16m{color}║\x1b[0m"
            ));
        } else {
            lines.push(format!("║{row}║"));
        }
    }
    lines.push(format!("╚{}╝", "═".repeat(INNER_WIDTH)));
    if colored {
        let top = format!("\x1b[48;5;16m{color}{}\x1b[0m", lines[0]);
        let bottom = format!("\x1b[48;5;16m{color}{}\x1b[0m", lines[23]);
        lines[0] = top;
        lines[23] = bottom;
    }
    lines.join("\n")
}

fn charge_art_style(kind: SupportKind, finish: SupportFinish, row: usize) -> &'static str {
    let class = kind.charge_class().expect("full art is Charge-only");
    match finish {
        SupportFinish::Standard => support_art_style(kind),
        SupportFinish::Rare => match (class, row % 2) {
            (CardClass::Robot, 0) => "\x1b[48;5;17m\x1b[38;5;117m",
            (CardClass::Robot, _) => "\x1b[48;5;18m\x1b[38;5;117m",
            (CardClass::Glitch, 0) => "\x1b[48;5;53m\x1b[38;5;213m",
            (CardClass::Glitch, _) => "\x1b[48;5;54m\x1b[38;5;213m",
            (CardClass::Daemon, 0) => "\x1b[48;5;52m\x1b[38;5;203m",
            (CardClass::Daemon, _) => "\x1b[48;5;88m\x1b[38;5;203m",
            (CardClass::Virus, 0) => "\x1b[48;5;22m\x1b[38;5;120m",
            (CardClass::Virus, _) => "\x1b[48;5;28m\x1b[38;5;120m",
            (CardClass::Bug, 0) => "\x1b[48;5;58m\x1b[38;5;222m",
            (CardClass::Bug, _) => "\x1b[48;5;94m\x1b[38;5;222m",
            (CardClass::Null, 0) => "\x1b[48;5;232m\x1b[38;5;255m",
            (CardClass::Null, _) => "\x1b[48;5;236m\x1b[38;5;255m",
        },
        SupportFinish::SuperRare => match class {
            CardClass::Robot => "\x1b[48;5;45m\x1b[38;5;16m",
            CardClass::Glitch => "\x1b[48;5;135m\x1b[38;5;16m",
            CardClass::Daemon => "\x1b[48;5;196m\x1b[38;5;16m",
            CardClass::Virus => "\x1b[48;5;46m\x1b[38;5;16m",
            CardClass::Bug => "\x1b[48;5;214m\x1b[38;5;16m",
            CardClass::Null => "\x1b[48;5;255m\x1b[38;5;16m",
        },
    }
}

fn charge_panel_style(kind: SupportKind, finish: SupportFinish) -> &'static str {
    match finish {
        SupportFinish::Standard => support_panel_style(kind),
        SupportFinish::Rare => charge_art_style(kind, finish, 2),
        SupportFinish::SuperRare => charge_art_style(kind, finish, 2),
    }
}

fn command_art_style(finish: SupportFinish, row: usize) -> &'static str {
    match finish {
        SupportFinish::Standard => "\x1b[48;5;16m\x1b[97m",
        SupportFinish::Rare if row % 2 == 0 => "\x1b[48;5;234m\x1b[38;5;255m",
        SupportFinish::Rare => "\x1b[48;5;236m\x1b[38;5;250m",
        SupportFinish::SuperRare if row % 2 == 0 => "\x1b[48;5;250m\x1b[38;5;16m",
        SupportFinish::SuperRare => "\x1b[48;5;255m\x1b[38;5;16m",
    }
}

fn command_panel_style(kind: SupportKind, finish: SupportFinish) -> &'static str {
    match finish {
        SupportFinish::Standard => support_panel_style(kind),
        SupportFinish::Rare => "\x1b[48;5;234m\x1b[38;5;255m",
        SupportFinish::SuperRare => "\x1b[48;5;250m\x1b[38;5;16m",
    }
}

fn support_art_style(kind: SupportKind) -> &'static str {
    match kind.charge_class().expect("full art is Charge-only") {
        CardClass::Robot => "\x1b[48;5;17m\x1b[96m",
        CardClass::Glitch => "\x1b[48;5;53m\x1b[95m",
        CardClass::Daemon => "\x1b[48;5;52m\x1b[91m",
        CardClass::Virus => "\x1b[48;5;22m\x1b[92m",
        CardClass::Bug => "\x1b[48;5;58m\x1b[38;5;214m",
        CardClass::Null => "\x1b[48;5;16m\x1b[38;5;255m",
    }
}

fn center_artwork(artwork: &[String]) -> Vec<String> {
    let rows: Vec<Vec<char>> = artwork.iter().map(|row| row.chars().collect()).collect();
    let left = rows
        .iter()
        .flat_map(|row| {
            row.iter()
                .enumerate()
                .filter(|(_, character)| **character != ' ')
                .map(|(column, _)| column)
        })
        .min()
        .unwrap_or(0);
    let right = rows
        .iter()
        .flat_map(|row| {
            row.iter()
                .enumerate()
                .filter(|(_, character)| **character != ' ')
                .map(|(column, _)| column)
        })
        .max()
        .unwrap_or(left);
    let width = right.saturating_sub(left) + 1;
    let padding = INNER_WIDTH.saturating_sub(width) / 2;
    rows.into_iter()
        .map(|row| {
            let mut centered = " ".repeat(padding);
            centered.extend(row.get(left..=right).unwrap_or(&[]));
            centered
        })
        .collect()
}

fn support_panel_style(kind: SupportKind) -> &'static str {
    if let Some(class) = kind.charge_class() {
        return match class {
            CardClass::Robot => "\x1b[48;5;17m\x1b[96m",
            CardClass::Glitch => "\x1b[48;5;53m\x1b[95m",
            CardClass::Daemon => "\x1b[48;5;52m\x1b[91m",
            CardClass::Virus => "\x1b[48;5;22m\x1b[92m",
            CardClass::Bug => "\x1b[48;5;58m\x1b[38;5;208m",
            CardClass::Null => "\x1b[48;5;16m\x1b[97m",
        };
    }
    match kind {
        SupportKind::QuickPatch
        | SupportKind::Purge
        | SupportKind::Download
        | SupportKind::IndexSearch
        | SupportKind::HotSwap
        | SupportKind::ForceRoute
        | SupportKind::Firewall
        | SupportKind::SafeMode
        | SupportKind::RebootCharge
        | SupportKind::BurstCharge => "\x1b[48;5;236m\x1b[38;5;252m",
        _ => unreachable!("Charge kinds returned above"),
    }
}

const fn support_border_color() -> &'static str {
    "\x1b[97m"
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CatalogPosition {
    pub number: u16,
    pub total: u16,
}

impl Default for CatalogPosition {
    fn default() -> Self {
        Self {
            number: 1,
            total: 100,
        }
    }
}

/// Temporary display-only treatments for evaluating a three-tier rarity model.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RarityPreview {
    Common,
    Rare,
    ReverseHolo,
    Gold,
    SuperRare,
}

#[must_use]
pub fn render(card: &CardType) -> String {
    render_with_serial(card, None)
}

/// Renders an owned copy when its permanent, type-local serial is known.
/// Unminted Card Type previews intentionally omit the serial.
#[must_use]
pub fn render_with_serial(card: &CardType, serial: Option<u32>) -> String {
    render_with_identity(card, None, serial)
}

/// Renders the Set and copy identity used on a complete card face.
#[must_use]
pub fn render_with_identity(card: &CardType, set_id: Option<u32>, serial: Option<u32>) -> String {
    render_identity(card, set_id, serial, CatalogPosition::default(), false)
}

/// ANSI-colored display variant. Canonical card data is unchanged.
#[must_use]
pub fn render_with_identity_colored(
    card: &CardType,
    set_id: Option<u32>,
    serial: Option<u32>,
) -> String {
    render_identity(card, set_id, serial, CatalogPosition::default(), true)
}

#[must_use]
pub fn render_with_catalog_identity(
    card: &CardType,
    set_id: Option<u32>,
    serial: Option<u32>,
    catalog: CatalogPosition,
    colored: bool,
) -> String {
    render_identity(card, set_id, serial, catalog, colored)
}

/// Temporary colored rarity mockup; never used for hashing or generation.
#[must_use]
pub fn render_rarity_preview(
    card: &CardType,
    set_id: Option<u32>,
    serial: Option<u32>,
    preview: RarityPreview,
) -> String {
    render_catalog_preview(card, set_id, serial, CatalogPosition::default(), preview)
}

#[must_use]
pub fn render_catalog_preview(
    card: &CardType,
    set_id: Option<u32>,
    serial: Option<u32>,
    catalog: CatalogPosition,
    preview: RarityPreview,
) -> String {
    render_identity_with_preview(card, set_id, serial, catalog, true, Some(preview))
}

/// Renders a fixed promotional copy with the Promo-only `✦` footer mark.
#[must_use]
pub fn render_promo_preview(
    card: &CardType,
    set_id: u32,
    serial: u32,
    promo_number: u16,
) -> String {
    assert!((1..=10).contains(&promo_number));
    assert!((1..=MAX_DISPLAY_SET_ID).contains(&set_id));
    assert!((1..=MAX_DISPLAY_SERIAL).contains(&serial));
    let identity = footer(
        &format!("{promo_number:03} ✦"),
        &format!("SET {set_id:02}"),
        &format!("#{serial:06}"),
    );
    let centered_artwork: Vec<Vec<char>> = center_artwork(&card.artwork)
        .into_iter()
        .map(|row| fixed_width(&row).chars().collect())
        .collect();
    let mut artwork = vec![vec![' '; INNER_WIDTH]; 2];
    artwork.extend(centered_artwork);
    artwork.truncate(21);
    artwork.resize(21, vec![' '; INNER_WIDTH]);
    let doge_source = artwork.clone();
    let mut eye_cells = vec![vec![false; INNER_WIDTH]; 21];
    mark_all_promo_patterns(&doge_source[7], &mut eye_cells[7], &['░', '░', '░']);
    let mut foreground = vec![vec![false; INNER_WIDTH]; 21];
    let mut coverage = vec![vec![false; INNER_WIDTH]; 21];
    overlay_promo_text(
        &mut artwork[0],
        &mut foreground[0],
        &mut coverage[0],
        &columns(&card.name, &format!("HP: {}", card.hit_points)),
    );
    overlay_promo_text(
        &mut artwork[1],
        &mut foreground[1],
        &mut coverage[1],
        &columns(
            &format!(
                "{} {}",
                card.class.symbol(),
                card.class.name().to_uppercase()
            ),
            "",
        ),
    );
    for (slot, action) in card.actions.iter().take(2).enumerate() {
        let heading_row = 11 + slot * 4;
        let (heading, detail) = match action {
            CardAction::Attack {
                name,
                damage,
                cost,
                effect,
            } => (
                columns(name, &damage.to_string()),
                format!("{} {cost} {effect}", attack_charge_symbol(card.class)),
            ),
            CardAction::Ability { name, effect } => (name.clone(), effect.clone()),
        };
        overlay_promo_text(
            &mut artwork[heading_row],
            &mut foreground[heading_row],
            &mut coverage[heading_row],
            &heading,
        );
        for (offset, detail_row) in wrap(&detail).into_iter().take(2).enumerate() {
            overlay_promo_text(
                &mut artwork[heading_row + 1 + offset],
                &mut foreground[heading_row + 1 + offset],
                &mut coverage[heading_row + 1 + offset],
                &detail_row,
            );
        }
    }

    let border = "\x1b[48;5;16m\x1b[97m";
    let mut lines = Vec::with_capacity(CARD_HEIGHT);
    lines.push(format!("{border}╔{}╗\x1b[0m", "═".repeat(INNER_WIDTH)));
    for (row_index, ((((row, mask), covered), source), eyes)) in artwork
        .into_iter()
        .zip(foreground)
        .zip(coverage)
        .zip(doge_source)
        .zip(eye_cells)
        .enumerate()
    {
        let mut content = String::new();
        for (column, ((((character, foreground), covered), source), eye)) in row
            .into_iter()
            .zip(mask)
            .zip(covered)
            .zip(source)
            .zip(eyes)
            .enumerate()
        {
            content.push_str(if covered {
                promo_overlay_style(source, foreground)
            } else if foreground {
                unreachable!("foreground text is always covered")
            } else if eye {
                "\x1b[48;5;16m\x1b[1;97m"
            } else if source != ' ' {
                "\x1b[48;5;16m\x1b[38;5;180m"
            } else {
                "\x1b[48;5;16m\x1b[38;5;240m"
            });
            let visible_character = if !foreground
                && !covered
                && source == ' '
                && character == ' '
                && (row_index * 3 + column) % 7 == 0
            {
                '·'
            } else {
                character
            };
            content.push(visible_character);
            content.push_str("\x1b[0m");
        }
        lines.push(format!("{border}║\x1b[0m{content}{border}║\x1b[0m"));
    }
    let footer_content = promo_star_texture(&identity, 22);
    let mut colored_footer = String::new();
    for character in footer_content.chars() {
        colored_footer.push_str(if character == '·' {
            "\x1b[48;5;16m\x1b[38;5;240m"
        } else {
            "\x1b[48;5;16m\x1b[1;97m"
        });
        colored_footer.push(character);
        colored_footer.push_str("\x1b[0m");
    }
    lines.push(format!("{border}║\x1b[0m{colored_footer}{border}║\x1b[0m"));
    lines.push(format!("{border}╚{}╝\x1b[0m", "═".repeat(INNER_WIDTH)));
    lines.join("\n")
}

fn overlay_promo_text(
    row: &mut [char],
    foreground: &mut [bool],
    coverage: &mut [bool],
    text: &str,
) {
    let characters: Vec<_> = text.chars().take(INNER_WIDTH).collect();
    for (index, character) in characters.iter().copied().enumerate() {
        if character != ' ' {
            row[index] = character;
            foreground[index] = true;
            coverage[index] = true;
        }
    }
}

fn mark_all_promo_patterns(row: &[char], mask: &mut [bool], pattern: &[char]) {
    for start in 0..=row.len().saturating_sub(pattern.len()) {
        if &row[start..start + pattern.len()] == pattern {
            mask[start..start + pattern.len()].fill(true);
        }
    }
}

fn promo_overlay_style(_source: char, foreground: bool) -> &'static str {
    assert!(foreground, "Promo overlay coverage must contain text");
    "\x1b[48;5;16m\x1b[1;97m"
}

fn promo_star_texture(value: &str, row: usize) -> String {
    let characters: Vec<_> = value.chars().collect();
    characters
        .iter()
        .enumerate()
        .map(|(column, character)| {
            let word_separator = *character == ' '
                && column > 0
                && column + 1 < characters.len()
                && characters[column - 1] != ' '
                && characters[column + 1] != ' ';
            if *character == ' ' && !word_separator && (row * 3 + column) % 7 == 0 {
                '·'
            } else {
                *character
            }
        })
        .collect()
}

fn render_identity(
    card: &CardType,
    set_id: Option<u32>,
    serial: Option<u32>,
    catalog: CatalogPosition,
    colored: bool,
) -> String {
    render_identity_with_preview(card, set_id, serial, catalog, colored, None)
}

fn render_identity_with_preview(
    card: &CardType,
    set_id: Option<u32>,
    serial: Option<u32>,
    catalog: CatalogPosition,
    colored: bool,
    preview: Option<RarityPreview>,
) -> String {
    assert!(
        catalog.number >= 1 && catalog.number <= catalog.total,
        "catalog number must be between 1 and its total"
    );
    assert!(
        catalog.total <= MAX_DISPLAY_CATALOG_SIZE,
        "catalog total must not exceed {MAX_DISPLAY_CATALOG_SIZE}"
    );
    let mut lines = vec![format!("╔{}╗", "═".repeat(INNER_WIDTH))];
    add_shaded(
        &mut lines,
        columns(&card.name, &format!("HP: {}", card.hit_points)),
        card.class,
        colored,
        preview,
    );
    add_shaded(
        &mut lines,
        format!("{} {}", card.class.symbol(), card.class.name()),
        card.class,
        colored,
        preview,
    );
    separator(&mut lines, card.class, colored, preview);
    add_art_row(&mut lines, "", card.class, colored, preview);
    let artwork: Vec<_> = card.artwork.iter().take(ART_HEIGHT).collect();
    let padding = ART_HEIGHT.saturating_sub(artwork.len());
    for _ in 0..padding / 2 {
        add_art_row(&mut lines, "", card.class, colored, preview);
    }
    for row in artwork {
        add_art_row(
            &mut lines,
            center(row.trim_end()),
            card.class,
            colored,
            preview,
        );
    }
    for _ in 0..padding.div_ceil(2) {
        add_art_row(&mut lines, "", card.class, colored, preview);
    }
    add_art_row(&mut lines, "", card.class, colored, preview);
    separator(&mut lines, card.class, colored, preview);
    let mut action_rows = Vec::with_capacity(7);
    for (index, action) in card.actions.iter().take(2).enumerate() {
        if index != 0 {
            action_rows.push(String::new());
        }
        let (heading, detail) = match action {
            CardAction::Attack {
                name,
                damage,
                cost,
                effect,
            } => (
                columns(name, &damage.to_string()),
                format!("{} {cost} {effect}", attack_charge_symbol(card.class)),
            ),
            CardAction::Ability { name, effect } => (columns(name, "ABILITY"), effect.clone()),
        };
        action_rows.push(heading);
        let wrapped = wrap(&detail);
        action_rows.push(wrapped.first().cloned().unwrap_or_default());
        action_rows.push(wrapped.get(1).cloned().unwrap_or_default());
    }
    action_rows.resize(7, String::new());
    for row in action_rows.into_iter().take(7) {
        add_shaded(&mut lines, row, card.class, colored, preview);
    }
    let serial = serial.map(|value| {
        assert!(
            (1..=MAX_DISPLAY_SERIAL).contains(&value),
            "card serial must be between 1 and {MAX_DISPLAY_SERIAL}"
        );
        format!("#{value:06}")
    });
    let set = set_id.map(|set_id| {
        assert!(
            (1..=MAX_DISPLAY_SET_ID).contains(&set_id),
            "Set ID must be between 1 and {MAX_DISPLAY_SET_ID}"
        );
        format!("SET {set_id:02}")
    });
    add_shaded(&mut lines, "", card.class, colored, preview);
    add_shaded(
        &mut lines,
        footer(
            &format!(
                "{:03} {}",
                catalog.number,
                "★".repeat(preview.map_or(card.rarity.stars() as usize, |preview| {
                    match preview {
                        RarityPreview::Common => 1,
                        RarityPreview::Rare | RarityPreview::ReverseHolo => 2,
                        RarityPreview::Gold | RarityPreview::SuperRare => 3,
                    }
                })),
            ),
            set.as_deref().unwrap_or(""),
            serial.as_deref().unwrap_or(""),
        ),
        card.class,
        colored,
        preview,
    );
    lines.push(format!("╚{}╝", "═".repeat(INNER_WIDTH)));
    match preview {
        Some(RarityPreview::Gold) => color_gold_frame(&mut lines),
        Some(RarityPreview::SuperRare) => color_multicolor_frame(&mut lines),
        _ if colored => color_black_frame(&mut lines),
        _ => {}
    }
    lines.join("\n")
}

fn add_art_row(
    lines: &mut Vec<String>,
    value: impl AsRef<str>,
    class: CardClass,
    colored: bool,
    preview: Option<RarityPreview>,
) {
    let value = fixed_width(value.as_ref());
    match preview {
        Some(RarityPreview::Rare) => {
            let row = lines.len();
            let mut rendered = String::new();
            for (column, character) in value.chars().enumerate() {
                rendered.push_str(holo_background(class, row, column));
                if character != ' ' {
                    rendered.push_str("\x1b[97m");
                }
                rendered.push(character);
                rendered.push_str("\x1b[0m");
            }
            lines.push(format!("║{rendered}║"));
        }
        Some(RarityPreview::Gold) => {
            let row = lines.len();
            let mut rendered = String::new();
            for (column, character) in value.chars().enumerate() {
                rendered.push_str(gold_art_background(row, column));
                if character != ' ' {
                    rendered.push_str("\x1b[97m");
                }
                rendered.push(character);
                rendered.push_str("\x1b[0m");
            }
            lines.push(format!("║{rendered}║"));
        }
        Some(RarityPreview::SuperRare) => {
            let row_index = lines.len();
            let mut row = String::new();
            for (column, character) in value.chars().enumerate() {
                row.push_str(super_rare_background(row_index, column));
                if character != ' ' {
                    row.push_str("\x1b[97m");
                }
                row.push(character);
                row.push_str("\x1b[0m");
            }
            lines.push(format!("║{row}║"));
        }
        _ if colored
            && (class == CardClass::Null
                || preview.is_none()
                || preview == Some(RarityPreview::Common)
                || preview == Some(RarityPreview::ReverseHolo)) =>
        {
            let mut rendered = String::new();
            for character in value.chars() {
                rendered.push_str("\x1b[48;5;16m");
                if character != ' ' {
                    rendered.push_str("\x1b[97m");
                }
                rendered.push(character);
                rendered.push_str("\x1b[0m");
            }
            lines.push(format!("║{rendered}║"));
        }
        _ => add(lines, value),
    }
}

fn holo_background(class: CardClass, row: usize, column: usize) -> &'static str {
    let band = ((column + row * 2) / 3) % 3;
    match class {
        CardClass::Robot => [
            "\x1b[48;2;0;24;32m",
            "\x1b[48;2;0;32;42m",
            "\x1b[48;2;0;40;52m",
        ][band],
        CardClass::Glitch => [
            "\x1b[48;2;28;8;36m",
            "\x1b[48;2;38;10;48m",
            "\x1b[48;2;48;12;60m",
        ][band],
        CardClass::Daemon => [
            "\x1b[48;2;36;0;8m",
            "\x1b[48;2;48;0;12m",
            "\x1b[48;2;60;0;16m",
        ][band],
        CardClass::Virus => [
            "\x1b[48;2;0;30;12m",
            "\x1b[48;2;0;40;16m",
            "\x1b[48;2;0;50;20m",
        ][band],
        CardClass::Bug => [
            "\x1b[48;2;36;24;0m",
            "\x1b[48;2;48;32;0m",
            "\x1b[48;2;60;40;0m",
        ][band],
        CardClass::Null => [
            "\x1b[48;2;12;12;12m",
            "\x1b[48;2;20;20;20m",
            "\x1b[48;2;28;28;28m",
        ][band],
    }
}

fn super_rare_background(row: usize, column: usize) -> &'static str {
    const BANDS: [&str; 6] = [
        "\x1b[48;2;0;40;45m",
        "\x1b[48;2;45;0;40m",
        "\x1b[48;2;45;32;0m",
        "\x1b[48;2;25;10;50m",
        "\x1b[48;2;35;0;45m",
        "\x1b[48;2;0;20;50m",
    ];
    BANDS[((column + row * 2) / 3) % BANDS.len()]
}

fn color_multicolor_frame(lines: &mut [String]) {
    let last_row = lines.len() - 1;
    for (row_index, line) in lines.iter_mut().enumerate() {
        let characters: Vec<_> = line.chars().collect();
        let mut colored = String::new();
        if row_index == 0 || row_index == last_row {
            for (column, character) in characters.into_iter().enumerate() {
                colored.push_str("\x1b[48;5;16m");
                colored.push_str(rainbow_foreground(column));
                colored.push(character);
                colored.push_str("\x1b[0m");
            }
        } else {
            let color = rainbow_foreground(row_index);
            colored.push_str("\x1b[48;5;16m");
            colored.push_str(color);
            colored.push(characters[0]);
            colored.push_str("\x1b[0m");
            colored.extend(characters[1..characters.len() - 1].iter());
            colored.push_str("\x1b[48;5;16m");
            colored.push_str(color);
            colored.push(*characters.last().unwrap());
            colored.push_str("\x1b[0m");
        }
        *line = colored;
    }
}

fn rainbow_foreground(column: usize) -> &'static str {
    const COLORS: [&str; 6] = [
        "\x1b[38;5;45m",
        "\x1b[38;5;201m",
        "\x1b[38;5;220m",
        "\x1b[38;5;93m",
        "\x1b[38;5;129m",
        "\x1b[38;5;39m",
    ];
    COLORS[column % COLORS.len()]
}

fn rainbow_text_foreground(column: usize) -> &'static str {
    const COLORS: [&str; 6] = [
        "\x1b[38;5;37m",
        "\x1b[38;5;165m",
        "\x1b[38;5;178m",
        "\x1b[38;5;99m",
        "\x1b[38;5;127m",
        "\x1b[38;5;33m",
    ];
    COLORS[column % COLORS.len()]
}

fn color_gold_frame(lines: &mut [String]) {
    let last_row = lines.len() - 1;
    for (row_index, line) in lines.iter_mut().enumerate() {
        let characters: Vec<_> = line.chars().collect();
        let mut colored = String::new();
        if row_index == 0 || row_index == last_row {
            colored.push_str("\x1b[48;5;16m\x1b[38;5;220m");
            colored.extend(characters);
            colored.push_str("\x1b[0m");
        } else {
            colored.push_str("\x1b[48;5;16m\x1b[38;5;220m");
            colored.push(characters[0]);
            colored.push_str("\x1b[0m");
            colored.extend(characters[1..characters.len() - 1].iter());
            colored.push_str("\x1b[48;5;16m\x1b[38;5;220m");
            colored.push(*characters.last().unwrap());
            colored.push_str("\x1b[0m");
        }
        *line = colored;
    }
}

fn color_black_frame(lines: &mut [String]) {
    let last_row = lines.len() - 1;
    for (row_index, line) in lines.iter_mut().enumerate() {
        if row_index == 0 || row_index == last_row {
            *line = format!("\x1b[48;5;16m\x1b[97m{line}\x1b[0m");
        } else {
            let inner = &line['║'.len_utf8()..line.len() - '║'.len_utf8()];
            *line = format!("\x1b[48;5;16m\x1b[97m║\x1b[0m{inner}\x1b[48;5;16m\x1b[97m║\x1b[0m");
        }
    }
}

/// The universal card back. It contains no card-specific or ownership data.
#[must_use]
pub fn render_back() -> String {
    let mut canvas = vec![vec![' '; INNER_WIDTH]; 22];
    for (row_index, row) in canvas.iter_mut().enumerate() {
        for (column, cell) in row.iter_mut().enumerate() {
            *cell = if (row_index + column) % 2 == 0 {
                '░'
            } else {
                '▒'
            };
        }
    }
    for (relative_row, row) in canvas.iter_mut().enumerate().take(19).skip(3) {
        let diamond_row = relative_row - 3;
        let distance = diamond_row.min(15 - diamond_row);
        let left = 15 - distance * 2;
        let right = 16 + distance * 2;
        for cell in row.iter_mut().take(right).skip(left + 1) {
            *cell = ' ';
        }
        if diamond_row < 8 {
            row[left] = '/';
            row[right] = '\\';
        } else {
            row[left] = '\\';
            row[right] = '/';
        }
    }
    let logo = [
        "█▀▄ ███ ███",
        "█▀▄  █   █ ",
        "█▄▀ ███  █ ",
        "▄██  █  █▀▄ █▀▄ ▄██",
        "█   █▀█ █▀▄ █ █ ▀▄ ",
        "▀██ █ █ █ █ █▄▀ ██▀",
    ];
    for (offset, text) in logo.into_iter().enumerate() {
        let row = if offset < 3 {
            7 + offset
        } else {
            11 + offset - 3
        };
        let start = (INNER_WIDTH - text.chars().count()) / 2;
        place(&mut canvas[row], start, text);
    }

    let mut lines = vec![format!("╔{}╗", "═".repeat(INNER_WIDTH))];
    for row in canvas {
        add(&mut lines, row.into_iter().collect::<String>());
    }
    lines.push(format!("╚{}╝", "═".repeat(INNER_WIDTH)));
    lines.join("\n")
}

#[must_use]
pub fn side_by_side(left: &str, right: &str) -> String {
    left.lines()
        .zip(right.lines())
        .map(|(left, right)| format!("{left}   {right}"))
        .collect::<Vec<_>>()
        .join("\n")
}
fn add(lines: &mut Vec<String>, value: impl AsRef<str>) {
    let mut chars: String = value.as_ref().chars().take(INNER_WIDTH).collect();
    let length = chars.chars().count();
    chars.push_str(&" ".repeat(INNER_WIDTH - length));
    lines.push(format!("║{chars}║"));
}
fn add_shaded(
    lines: &mut Vec<String>,
    value: impl AsRef<str>,
    class: CardClass,
    colored: bool,
    preview: Option<RarityPreview>,
) {
    let row = lines.len();
    let value: Vec<_> = fixed_width(value.as_ref()).chars().collect();
    let mut shaded = String::new();
    for (index, character) in value.iter().enumerate() {
        let is_word_separator = *character == ' '
            && index > 0
            && index + 1 < value.len()
            && value[index - 1] != ' '
            && value[index + 1] != ' ';
        let reverse_holo = preview == Some(RarityPreview::ReverseHolo);
        if colored && reverse_holo {
            shaded.push_str(reverse_holo_background(class, row, index));
            if *character != ' ' {
                shaded.push_str(reverse_holo_text_color(class));
            }
            shaded.push(*character);
            shaded.push_str("\x1b[0m");
            continue;
        }
        if colored && class == CardClass::Null && preview != Some(RarityPreview::Gold) {
            shaded.push_str("\x1b[48;5;16m");
            if *character == ' ' && !is_word_separator {
                let texture = panel_texture(class, row, index);
                if texture != ' ' {
                    shaded.push_str("\x1b[90m");
                }
                shaded.push(texture);
            } else {
                if *character != ' ' {
                    shaded.push_str(premium_text_style(preview));
                    shaded.push_str(match preview {
                        Some(RarityPreview::SuperRare) => rainbow_text_foreground(index),
                        _ => "\x1b[97m",
                    });
                }
                shaded.push(*character);
            }
            shaded.push_str("\x1b[0m");
            continue;
        }
        if colored && (class != CardClass::Null || preview == Some(RarityPreview::Gold)) {
            shaded.push_str(information_background(class, row, index, preview));
            if *character != ' ' {
                shaded.push_str(premium_text_style(preview));
                shaded.push_str(match preview {
                    Some(RarityPreview::Gold) => "\x1b[38;5;16m",
                    Some(RarityPreview::SuperRare) => rainbow_text_foreground(index),
                    _ => primary_color(class),
                });
            }
            shaded.push(*character);
            shaded.push_str("\x1b[0m");
            continue;
        }
        if *character == ' ' && (class != CardClass::Null || !is_word_separator) {
            let texture = panel_texture(class, row, index);
            if colored {
                shaded.push_str(match preview {
                    Some(RarityPreview::Gold) => "\x1b[38;5;178m",
                    _ => panel_color(class, index),
                });
                shaded.push(texture);
                shaded.push_str("\x1b[0m");
            } else {
                shaded.push(texture);
            }
        } else if colored && *character != ' ' {
            shaded.push_str(premium_text_style(preview));
            shaded.push_str(match preview {
                Some(RarityPreview::Gold) => "\x1b[38;5;220m",
                _ => primary_color(class),
            });
            shaded.push(*character);
            shaded.push_str("\x1b[0m");
        } else {
            shaded.push(*character);
        }
    }
    lines.push(format!("║{shaded}║"));
}

fn information_background(
    class: CardClass,
    row: usize,
    column: usize,
    preview: Option<RarityPreview>,
) -> &'static str {
    match preview {
        Some(RarityPreview::Gold) => gold_background(row, column),
        _ => class_background(class, row, column),
    }
}

fn gold_background(row: usize, column: usize) -> &'static str {
    const GOLD: [&str; 3] = ["\x1b[48;5;100m", "\x1b[48;5;136m", "\x1b[48;5;178m"];
    GOLD[((column + row * 2) / 3) % GOLD.len()]
}

fn gold_art_background(row: usize, column: usize) -> &'static str {
    const DARK_GOLD: [&str; 3] = [
        "\x1b[48;2;50;38;0m",
        "\x1b[48;2;65;50;0m",
        "\x1b[48;2;80;62;0m",
    ];
    DARK_GOLD[((column + row * 2) / 3) % DARK_GOLD.len()]
}

fn premium_text_style(preview: Option<RarityPreview>) -> &'static str {
    match preview {
        Some(RarityPreview::Gold) => "\x1b[1m",
        Some(RarityPreview::SuperRare) => "\x1b[1;3m",
        _ => "",
    }
}

fn class_background(class: CardClass, _row: usize, _column: usize) -> &'static str {
    match class {
        CardClass::Robot => "\x1b[48;5;17m",
        CardClass::Glitch => "\x1b[48;5;53m",
        CardClass::Daemon => "\x1b[48;5;52m",
        CardClass::Virus => "\x1b[48;5;22m",
        CardClass::Bug => "\x1b[48;5;58m",
        CardClass::Null => "",
    }
}

fn reverse_holo_background(class: CardClass, row: usize, column: usize) -> &'static str {
    let band = match class {
        CardClass::Glitch => (column / 2 + row * 2) % 3,
        CardClass::Bug => (row / 2 + column / 2) % 2,
        CardClass::Null => (row * 11 + column * 5) % 2,
        _ => (row + column) % 2,
    };
    match class {
        CardClass::Robot => ["\x1b[48;5;45m", "\x1b[48;5;51m"][band],
        CardClass::Glitch => ["\x1b[48;5;129m", "\x1b[48;5;165m", "\x1b[48;5;201m"][band],
        CardClass::Daemon => ["\x1b[48;5;196m", "\x1b[48;5;203m"][band],
        CardClass::Virus => ["\x1b[48;5;40m", "\x1b[48;5;82m"][band],
        CardClass::Bug => ["\x1b[48;5;208m", "\x1b[48;5;214m"][band],
        CardClass::Null => ["\x1b[48;5;250m", "\x1b[48;5;255m"][band],
    }
}

fn reverse_holo_text_color(class: CardClass) -> &'static str {
    match class {
        CardClass::Robot => "\x1b[38;2;0;30;35m",
        CardClass::Glitch => "\x1b[38;2;30;0;40m",
        CardClass::Daemon => "\x1b[38;2;45;0;0m",
        CardClass::Virus => "\x1b[38;2;0;35;10m",
        CardClass::Bug => "\x1b[38;2;45;28;0m",
        CardClass::Null => "\x1b[38;2;25;25;25m",
    }
}
fn panel_color(class: CardClass, column: usize) -> &'static str {
    match class {
        CardClass::Robot => "\x1b[96m",
        CardClass::Glitch => [
            "\x1b[38;5;129m",
            "\x1b[38;5;165m",
            "\x1b[38;5;201m",
            "\x1b[95m",
        ][column % 4],
        CardClass::Daemon => "\x1b[91m",
        CardClass::Virus => "\x1b[92m",
        CardClass::Bug => "\x1b[38;5;208m",
        CardClass::Null => "\x1b[90m",
    }
}
fn primary_color(class: CardClass) -> &'static str {
    match class {
        CardClass::Robot => "\x1b[96m",
        CardClass::Glitch => "\x1b[95m",
        CardClass::Daemon => "\x1b[91m",
        CardClass::Virus => "\x1b[92m",
        CardClass::Bug => "\x1b[38;5;208m",
        CardClass::Null => "\x1b[97m",
    }
}
fn panel_texture(class: CardClass, row: usize, column: usize) -> char {
    match class {
        CardClass::Robot => '░',
        CardClass::Glitch => ['░', '▒', '▓', '▒', '░', '▓', '░', '▒'][column % 8],
        CardClass::Daemon => '▒',
        CardClass::Virus => {
            if (row + column) % 2 == 0 {
                '▓'
            } else {
                '▒'
            }
        }
        CardClass::Bug => {
            if (row / 2 + column / 2) % 2 == 0 {
                '░'
            } else {
                '▒'
            }
        }
        CardClass::Null => {
            if (row * 11 + column * 5) % 7 == 0 {
                '·'
            } else {
                ' '
            }
        }
    }
}
fn fixed_width(value: &str) -> String {
    let mut chars: String = value.chars().take(INNER_WIDTH).collect();
    chars.push_str(&" ".repeat(INNER_WIDTH - chars.chars().count()));
    chars
}
fn separator(
    lines: &mut Vec<String>,
    class: CardClass,
    colored: bool,
    preview: Option<RarityPreview>,
) {
    if !colored {
        lines.push(format!("╟{}╢", "─".repeat(INNER_WIDTH)));
        return;
    }

    let mut divider = String::new();
    for column in 0..INNER_WIDTH {
        divider.push_str("\x1b[48;5;16m");
        divider.push_str(match preview {
            Some(RarityPreview::Gold) => "\x1b[38;5;220m",
            Some(RarityPreview::SuperRare) => rainbow_foreground(column),
            Some(RarityPreview::ReverseHolo) => primary_color(class),
            _ if class == CardClass::Null => "\x1b[97m",
            _ => primary_color(class),
        });
        divider.push_str("─\x1b[0m");
    }
    lines.push(format!("╟{divider}╢"));
}
fn center(value: &str) -> String {
    let len = value.chars().count();
    if len >= INNER_WIDTH {
        value.chars().take(INNER_WIDTH).collect()
    } else {
        format!("{}{}", " ".repeat((INNER_WIDTH - len) / 2), value)
    }
}
fn columns(left: &str, right: &str) -> String {
    let left_len = left.chars().count();
    let right_len = right.chars().count();
    if left_len + right_len < INNER_WIDTH {
        format!(
            "{left}{}{right}",
            " ".repeat(INNER_WIDTH - left_len - right_len)
        )
    } else {
        format!(
            "{} {right}",
            left.chars()
                .take(INNER_WIDTH.saturating_sub(right_len + 1))
                .collect::<String>()
        )
    }
}

const fn attack_charge_symbol(class: CardClass) -> &'static str {
    match class {
        CardClass::Null => "◇",
        _ => class.symbol(),
    }
}

fn footer(left: &str, middle: &str, right: &str) -> String {
    let mut row = vec![' '; INNER_WIDTH];
    place(&mut row, 0, left);
    let middle_len = middle.chars().count();
    place(&mut row, (INNER_WIDTH - middle_len) / 2, middle);
    let right_len = right.chars().count();
    place(&mut row, INNER_WIDTH - right_len, right);
    row.into_iter().collect()
}
fn place(row: &mut [char], start: usize, value: &str) {
    for (index, character) in value.chars().enumerate() {
        row[start + index] = character;
    }
}
fn wrap(text: &str) -> Vec<String> {
    let mut rows = vec![String::new()];
    for word in text.split_whitespace() {
        let row = rows.last_mut().unwrap();
        if !row.is_empty() && row.chars().count() + 1 + word.chars().count() > INNER_WIDTH {
            rows.push(word.into());
        } else {
            if !row.is_empty() {
                row.push(' ');
            }
            row.push_str(word);
        }
    }
    rows
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::{CURRENT_GENERATOR_VERSION, CardGenerator};
    #[test]
    fn rendering_is_fixed_size_utf8() {
        let card = CardGenerator::default()
            .generate(1, &[0; 32], &[3])
            .unwrap();
        let output = render(&card);
        let lines: Vec<_> = output.lines().collect();
        assert_eq!(lines.len(), 24);
        assert!(lines.iter().all(|line| line.chars().count() == 34));
        assert!(output.contains('★'));
        assert_eq!(lines[0], format!("╔{}╗", "═".repeat(INNER_WIDTH)));
        assert_eq!(
            lines[lines.len() - 1],
            format!("╚{}╝", "═".repeat(INNER_WIDTH))
        );
    }

    #[test]
    fn every_action_layout_keeps_the_card_fixed_height() {
        let generator = CardGenerator::default();
        let mut seen_lengths = std::collections::HashSet::new();
        for seed in 0..=255u8 {
            let card = generator
                .generate(CURRENT_GENERATOR_VERSION, &[0; 32], &[seed])
                .unwrap();
            seen_lengths.insert(card.actions.len());
            let output = render(&card);
            assert_eq!(output.lines().count(), 24);
            assert!(output.lines().all(|line| line.chars().count() == 34));
        }
        assert_eq!(seen_lengths, std::collections::HashSet::from([1, 2]));
    }

    #[test]
    fn generated_action_text_never_overflows_its_reserved_rows() {
        let generator = CardGenerator::default();
        for seed in 0..=4095u16 {
            let card = generator
                .generate(CURRENT_GENERATOR_VERSION, &[0; 32], &seed.to_be_bytes())
                .unwrap();
            for action in &card.actions {
                let (heading, detail) = match action {
                    CardAction::Attack {
                        name,
                        damage,
                        cost,
                        effect,
                    } => (
                        columns(name, &damage.to_string()),
                        format!("{} {cost} {effect}", attack_charge_symbol(card.class)),
                    ),
                    CardAction::Ability { name, effect } => {
                        (columns(name, "ABILITY"), effect.clone())
                    }
                };
                assert_eq!(heading.chars().count(), INNER_WIDTH);
                assert!(
                    wrap(&detail).len() <= 2,
                    "action text overflowed for seed {seed}: {detail}"
                );
            }
        }
    }

    #[test]
    fn attack_costs_use_class_symbols_and_explicit_numbers() {
        for class in CardClass::ALL {
            let symbol = attack_charge_symbol(class);
            let detail = format!("{symbol} {} Effect", 3);
            assert!(detail.starts_with(symbol));
            assert!(detail.contains(" 3 "));
            assert!(!detail.contains("CHG"));
            assert!(!detail.contains('•'));
        }
        assert_eq!(attack_charge_symbol(CardClass::Null), "◇");
        assert_ne!(
            attack_charge_symbol(CardClass::Null),
            CardClass::Null.symbol()
        );
    }

    #[test]
    fn support_previews_match_the_standard_card_dimensions() {
        for kind in SupportKind::COMMANDS
            .into_iter()
            .chain(SupportKind::CHARGES)
        {
            let card = crate::support_preview::generate(kind);
            let output = render_support_preview(&card, false);
            assert_eq!(output.lines().count(), 24);
            assert!(output.lines().all(|line| line.chars().count() == 34));
            assert_eq!(
                output.lines().filter(|line| line.starts_with('╟')).count(),
                if kind.is_charge() { 1 } else { 2 }
            );
            assert!(!output.contains("DRAFT PREVIEW"));
            assert!(output.contains("SET 01"));
            assert!(output.contains("#000001"));
        }
    }

    #[test]
    fn promo_footer_uses_its_own_mark_instead_of_rarity_stars() {
        let card = crate::promo_preview::set_one_promos().remove(0);
        let output = render_promo_preview(&card, 1, 1, 1);
        let footer = output.lines().nth(22).unwrap();
        assert!(footer.contains('✦'));
        assert!(!footer.contains('★'));
        assert_eq!(output.lines().count(), 24);
    }

    #[test]
    fn command_finishes_are_visual_only_and_use_unique_terminal_styles() {
        let card = crate::support_preview::generate(SupportKind::QuickPatch);
        let standard = render_support_preview_with_identity_and_finish(
            &card,
            false,
            1,
            1,
            1,
            SupportFinish::Standard,
        );
        let rare = render_support_preview_with_identity_and_finish(
            &card,
            true,
            1,
            1,
            1,
            SupportFinish::Rare,
        );
        let super_rare_plain = render_support_preview_with_identity_and_finish(
            &card,
            false,
            1,
            1,
            1,
            SupportFinish::SuperRare,
        );
        let super_rare = render_support_preview_with_identity_and_finish(
            &card,
            true,
            1,
            1,
            1,
            SupportFinish::SuperRare,
        );

        assert!(standard.contains("001 ★ "));
        assert!(rare.contains("001 ★★ "));
        assert!(rare.contains("\x1b[48;5;234m\x1b[38;5;255m"));
        assert!(super_rare_plain.contains("001 ★★★ "));
        assert!(super_rare_plain.contains("╔═══════════════════════════╗"));
        assert!(super_rare_plain.contains("# patch --active"));
        assert!(super_rare.contains("\x1b[48;5;250m\x1b[38;5;16m"));
        for output in [standard, super_rare_plain] {
            assert_eq!(output.lines().count(), 24);
            assert!(output.lines().all(|line| line.chars().count() == 34));
        }
    }

    #[test]
    fn all_standard_commands_share_one_gray_panel_palette() {
        for kind in SupportKind::COMMANDS {
            let card = crate::support_preview::generate(kind);
            let output = render_support_preview_with_identity_and_finish(
                &card,
                true,
                1,
                1,
                1,
                SupportFinish::Standard,
            );
            assert!(output.contains("\x1b[48;5;236m\x1b[38;5;252m"));
        }
    }

    #[test]
    fn charge_finishes_keep_identity_and_use_pulse_and_overcharged_styles() {
        let card = crate::support_preview::generate(SupportKind::RobotCharge);
        let rare = render_support_preview_with_identity_and_finish(
            &card,
            true,
            1,
            1,
            1,
            SupportFinish::Rare,
        );
        let super_rare = render_support_preview_with_identity_and_finish(
            &card,
            true,
            1,
            1,
            1,
            SupportFinish::SuperRare,
        );
        let rare_plain = render_support_preview_with_identity_and_finish(
            &card,
            false,
            1,
            1,
            1,
            SupportFinish::Rare,
        );
        let super_rare_plain = render_support_preview_with_identity_and_finish(
            &card,
            false,
            1,
            1,
            1,
            SupportFinish::SuperRare,
        );

        assert!(rare.contains("\x1b[48;5;17m\x1b[38;5;117m"));
        assert!(rare.contains("\x1b[48;5;18m\x1b[38;5;117m"));
        assert!(super_rare.contains("\x1b[48;5;45m\x1b[38;5;16m"));
        assert!(rare_plain.contains("001 ★★ "));
        assert!(super_rare_plain.contains("001 ★★★ "));
        for output in [rare_plain, super_rare_plain] {
            assert_eq!(output.lines().count(), 24);
            assert!(output.lines().all(|line| line.chars().count() == 34));
        }
    }

    #[test]
    fn owned_copy_shows_fixed_width_serial_without_a_footer_divider() {
        let card = CardGenerator::default()
            .generate(1, &[0; 32], &[3])
            .unwrap();
        let output = render_with_serial(&card, Some(81));
        let rows: Vec<_> = output.lines().collect();
        assert!(rows[rows.len() - 2].contains("#000081"));
        assert!(!rows[rows.len() - 3].starts_with('╟'));
    }

    #[test]
    fn complete_card_face_shows_set_and_serial() {
        let card = CardGenerator::default()
            .generate(1, &[0; 32], &[3])
            .unwrap();
        let output = render_with_identity(&card, Some(2), Some(81));
        let rows: Vec<_> = output.lines().collect();
        let footer: Vec<_> = rows[rows.len() - 2].chars().collect();
        assert_eq!(footer[14..20].iter().collect::<String>(), "SET░02");
        assert_eq!(footer[26..33].iter().collect::<String>(), "#000081");
        assert_ne!(rows[rows.len() - 3], format!("║{}║", " ".repeat(32)));
    }

    #[test]
    fn catalog_position_is_distinct_from_copy_serial() {
        let card = CardGenerator::default()
            .generate(1, &[0; 32], &[3])
            .unwrap();
        let output = render_with_catalog_identity(
            &card,
            Some(2),
            Some(81),
            CatalogPosition {
                number: 25,
                total: 100,
            },
            false,
        );
        let rows: Vec<_> = output.lines().collect();
        assert!(!rows[1].contains("025"));
        let footer = rows[rows.len() - 2];
        assert!(footer.contains("025"));
        assert!(!footer.contains("/100"));
        assert!(footer.contains('★'));
        assert!(footer.contains("#000081"));
    }

    #[test]
    fn premium_text_treatments_are_fixed_width_display_only() {
        assert_eq!(premium_text_style(Some(RarityPreview::Gold)), "\x1b[1m");
        assert_eq!(
            premium_text_style(Some(RarityPreview::SuperRare)),
            "\x1b[1;3m"
        );
        assert_eq!(premium_text_style(Some(RarityPreview::Rare)), "");
    }

    #[test]
    fn universal_back_matches_front_dimensions() {
        let output = render_back();
        let rows: Vec<_> = output.lines().collect();
        assert_eq!(rows.len(), 24);
        assert!(rows.iter().all(|row| row.chars().count() == 34));
    }

    #[test]
    fn every_class_has_a_distinct_panel_texture() {
        let textures: Vec<String> = CardClass::ALL
            .into_iter()
            .map(|class| {
                (0..4)
                    .flat_map(|row| (0..INNER_WIDTH).map(move |column| (row, column)))
                    .map(|(row, column)| panel_texture(class, row, column))
                    .collect()
            })
            .collect();
        for (index, texture) in textures.iter().enumerate() {
            assert!(!textures[..index].contains(texture));
        }
    }

    #[test]
    fn textures_flow_through_text_spaces_except_for_null() {
        let mut robot = Vec::new();
        add_shaded(&mut robot, "A B", CardClass::Robot, false, None);
        assert!(robot[0].contains("A░B"));

        let mut null = Vec::new();
        add_shaded(&mut null, "A B", CardClass::Null, false, None);
        assert!(null[0].contains("A B"));
    }

    #[test]
    fn finish_effects_respect_the_artwork_boundary() {
        let mut holo_lines = Vec::new();
        add_shaded(
            &mut holo_lines,
            "TEXT SAMPLE",
            CardClass::Robot,
            true,
            Some(RarityPreview::Rare),
        );
        assert!(holo_lines[0].contains(class_background(CardClass::Robot, 0, 0)));
        assert!(!holo_lines[0].contains(holo_background(CardClass::Robot, 0, 0)));

        let mut reverse_lines = Vec::new();
        add_shaded(
            &mut reverse_lines,
            "TEXT SAMPLE",
            CardClass::Robot,
            true,
            Some(RarityPreview::ReverseHolo),
        );
        assert!(reverse_lines[0].contains(reverse_holo_background(CardClass::Robot, 0, 0)));
        assert!(
            reverse_lines[0].contains(&format!("{}T", reverse_holo_text_color(CardClass::Robot)))
        );
        assert!(
            !['░', '▒', '▓']
                .into_iter()
                .any(|glyph| reverse_lines[0].contains(glyph))
        );

        for preview in [
            RarityPreview::Rare,
            RarityPreview::Gold,
            RarityPreview::SuperRare,
        ] {
            let mut artwork = Vec::new();
            add_art_row(&mut artwork, "/", CardClass::Robot, true, Some(preview));
            assert!(artwork[0].contains("\x1b[97m/"));
        }
    }

    #[test]
    fn text_cells_in_colored_finishes_have_matching_backgrounds() {
        for (preview, background, style, foreground) in [
            (
                None,
                class_background(CardClass::Robot, 0, 0),
                "",
                "\x1b[96m",
            ),
            (
                Some(RarityPreview::Rare),
                class_background(CardClass::Robot, 0, 0),
                "",
                "\x1b[96m",
            ),
            (
                Some(RarityPreview::Gold),
                gold_background(0, 0),
                "\x1b[1m",
                "\x1b[38;5;16m",
            ),
            (
                Some(RarityPreview::SuperRare),
                class_background(CardClass::Robot, 0, 0),
                "\x1b[1;3m",
                rainbow_text_foreground(0),
            ),
        ] {
            let mut lines = Vec::new();
            add_shaded(&mut lines, "T", CardClass::Robot, true, preview);
            assert!(lines[0].contains(&format!("{background}{style}{foreground}T")));
        }

        let mut null = Vec::new();
        add_shaded(&mut null, "T", CardClass::Null, true, None);
        assert!(null[0].contains("\x1b[48;5;16m\x1b[97mT"));

        let mut null_gold = Vec::new();
        add_shaded(
            &mut null_gold,
            "T",
            CardClass::Null,
            true,
            Some(RarityPreview::Gold),
        );
        assert!(null_gold[0].contains(&format!("{}\x1b[1m\x1b[38;5;16mT", gold_background(0, 0))));

        let mut null_rainbow = Vec::new();
        add_shaded(
            &mut null_rainbow,
            "T",
            CardClass::Null,
            true,
            Some(RarityPreview::SuperRare),
        );
        assert!(null_rainbow[0].contains(&format!(
            "\x1b[48;5;16m\x1b[1;3m{}T",
            rainbow_text_foreground(0)
        )));

        let mut null_texture = Vec::new();
        add_shaded(&mut null_texture, "", CardClass::Null, true, None);
        assert!(null_texture[0].contains("\x1b[48;5;16m\x1b[90m·"));
    }

    #[test]
    fn busy_classes_have_distinct_high_contrast_reverse_holos() {
        for class in [CardClass::Glitch, CardClass::Virus, CardClass::Bug] {
            let mut standard = Vec::new();
            add_shaded(&mut standard, "TEXT", class, true, None);
            let mut reverse = Vec::new();
            add_shaded(
                &mut reverse,
                "TEXT",
                class,
                true,
                Some(RarityPreview::ReverseHolo),
            );

            assert_ne!(standard, reverse);
            assert!(reverse[0].contains(&format!("{}T", reverse_holo_text_color(class))));
            assert!(!standard[0].contains(reverse_holo_background(class, 0, 0)));
        }

        assert_eq!(class_background(CardClass::Bug, 0, 0), "\x1b[48;5;58m");
        assert_eq!(class_background(CardClass::Bug, 0, 2), "\x1b[48;5;58m");
        assert_eq!(primary_color(CardClass::Bug), "\x1b[38;5;208m");

        let glitch_reverse: Vec<_> = (0..6)
            .map(|column| reverse_holo_background(CardClass::Glitch, 0, column))
            .collect();
        assert!([129, 165, 201].into_iter().all(|color| {
            glitch_reverse
                .iter()
                .any(|background| *background == format!("\x1b[48;5;{color}m"))
        }));
        assert!(!glitch_reverse.contains(&"\x1b[48;5;196m"));
    }

    #[test]
    fn colored_information_background_continues_through_text_and_empty_cells() {
        let mut lines = Vec::new();
        add_shaded(&mut lines, "T ", CardClass::Glitch, true, None);
        assert!(lines[0].contains(&format!(
            "{}\x1b[95mT\x1b[0m{} ",
            information_background(CardClass::Glitch, 0, 0, None),
            information_background(CardClass::Glitch, 0, 1, None),
        )));
        assert!(
            !['░', '▒', '▓']
                .into_iter()
                .any(|glyph| lines[0].contains(glyph))
        );

        let mut rainbow_information = Vec::new();
        add_shaded(
            &mut rainbow_information,
            "TEXT",
            CardClass::Robot,
            true,
            Some(RarityPreview::SuperRare),
        );
        assert!(rainbow_information[0].contains(class_background(CardClass::Robot, 0, 0)));
        assert!(
            [24, 30, 53, 89, 54]
                .into_iter()
                .all(|color| !rainbow_information[0].contains(&format!("\x1b[48;5;{color}m")))
        );
    }

    #[test]
    fn standard_holo_and_rainbow_information_panels_are_solid() {
        for class in [CardClass::Glitch, CardClass::Virus, CardClass::Bug] {
            for preview in [
                None,
                Some(RarityPreview::Rare),
                Some(RarityPreview::SuperRare),
            ] {
                let mut lines = Vec::new();
                add_shaded(&mut lines, "TEXT", class, true, preview);
                assert_eq!(
                    lines[0].matches(class_background(class, 0, 0)).count(),
                    INNER_WIDTH
                );
            }
        }
    }

    #[test]
    fn colored_display_uses_expected_ansi_palette() {
        let mut card = CardGenerator::default()
            .generate(1, &[0; 32], &[3])
            .unwrap();
        for (class, color) in [
            (CardClass::Robot, "\x1b[96m"),
            (CardClass::Daemon, "\x1b[91m"),
            (CardClass::Virus, "\x1b[92m"),
            (CardClass::Bug, "\x1b[38;5;208m"),
            (CardClass::Null, "\x1b[97m"),
        ] {
            card.class = class;
            let output = render_with_identity_colored(&card, Some(1), Some(1));
            assert!(output.contains(color));
            assert!(output.contains("\x1b[0m"));
        }
        card.class = CardClass::Glitch;
        let output = render_with_identity_colored(&card, Some(1), Some(1));
        assert!(output.contains("\x1b[48;5;53m"));
        assert!(!output.contains("\x1b[48;5;88m"));
        let first_name_character = card.name.chars().next().unwrap();
        assert!(output.contains(&format!(
            "{}{}\x1b[0m",
            primary_color(card.class),
            first_name_character
        )));
        assert!(output.starts_with("\x1b[48;5;16m\x1b[97m╔"));
    }

    #[test]
    fn colored_frames_and_null_voids_do_not_inherit_terminal_background() {
        let mut card = CardGenerator::default()
            .generate(1, &[0; 32], &[3])
            .unwrap();
        card.class = CardClass::Null;
        let null = render_with_identity_colored(&card, Some(1), Some(1));
        let null_rows: Vec<_> = null.lines().collect();
        assert!(null_rows[0].starts_with("\x1b[48;5;16m\x1b[97m╔"));
        assert!(null_rows[4].contains("\x1b[48;5;16m"));
        assert!(null_rows[3].contains("\x1b[48;5;16m\x1b[97m─"));

        card.class = CardClass::Robot;
        let standard = render_rarity_preview(&card, Some(1), Some(1), RarityPreview::Common);
        assert!(
            standard
                .lines()
                .nth(4)
                .unwrap()
                .matches("\x1b[48;5;16m")
                .count()
                > 2
        );

        let reverse = render_rarity_preview(&card, Some(1), Some(1), RarityPreview::ReverseHolo);
        assert!(
            reverse
                .lines()
                .nth(4)
                .unwrap()
                .matches("\x1b[48;5;16m")
                .count()
                > 2
        );

        let holo = render_rarity_preview(&card, Some(1), Some(1), RarityPreview::Rare);
        assert!(
            holo.lines()
                .nth(3)
                .unwrap()
                .contains(&format!("\x1b[48;5;16m{}─", primary_color(card.class)))
        );
    }

    #[test]
    fn rarity_previews_are_display_only_colored_treatments() {
        let card = CardGenerator::default()
            .generate(1, &[0; 32], &[3])
            .unwrap();
        let rare = render_rarity_preview(&card, Some(1), Some(1), RarityPreview::Rare);
        let super_rare = render_rarity_preview(&card, Some(1), Some(1), RarityPreview::SuperRare);
        let reverse = render_rarity_preview(&card, Some(1), Some(1), RarityPreview::ReverseHolo);
        let gold = render_rarity_preview(&card, Some(1), Some(1), RarityPreview::Gold);
        assert!(rare.contains(holo_background(card.class, 4, 0)));
        assert!(rare.contains("\x1b[97m"));
        let first_name_character = card.name.chars().next().unwrap();
        assert!(rare.contains(&format!(
            "{}{}",
            primary_color(card.class),
            first_name_character
        )));
        assert!(
            [
                "\x1b[48;2;0;40;45m",
                "\x1b[48;2;45;0;40m",
                "\x1b[48;2;45;32;0m",
                "\x1b[48;2;25;10;50m",
                "\x1b[48;2;35;0;45m",
                "\x1b[48;2;0;20;50m",
            ]
            .into_iter()
            .all(|color| super_rare.contains(color))
        );
        assert!(super_rare.contains("\x1b[38;5;201m"));
        assert!(super_rare.contains("\x1b[48;5;16m\x1b[38;5;45m╔"));
        assert!(reverse.contains(reverse_holo_background(card.class, 1, 0)));
        let reverse_rows: Vec<_> = reverse.lines().collect();
        for row in [reverse_rows[3], reverse_rows[13]] {
            assert!(row.contains(&format!("\x1b[48;5;16m{}─", primary_color(card.class))));
        }
        assert!(
            [100, 136, 178]
                .into_iter()
                .all(|color| gold.contains(&format!("\x1b[48;5;{color}m")))
        );
        assert!(gold.contains("\x1b[38;5;16m"));
        assert!(gold.contains("\x1b[38;5;220m"));
    }
}
