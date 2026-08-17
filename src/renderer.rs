use crate::model::{CardClass, CardType};
/// Display dimensions are deliberately separate from canonical Card Type data.
pub const INNER_WIDTH: usize = 32;
pub const ART_HEIGHT: usize = 7;
pub const MAX_DISPLAY_SET_ID: u32 = 99;
pub const MAX_DISPLAY_SERIAL: u32 = 999_999;
pub const MAX_DISPLAY_CATALOG_SIZE: u16 = 999;

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
    for (index, attack) in card.attacks.iter().take(2).enumerate() {
        if index != 0 {
            add_shaded(&mut lines, "", card.class, colored, preview);
        }
        add_shaded(
            &mut lines,
            columns(&attack.name, &attack.damage.to_string()),
            card.class,
            colored,
            preview,
        );
        let wrapped = wrap(&format!("CHG {} • {}", attack.cost, attack.effect));
        add_shaded(&mut lines, &wrapped[0], card.class, colored, preview);
        add_shaded(
            &mut lines,
            wrapped.get(1).map_or("", String::as_str),
            card.class,
            colored,
            preview,
        );
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
                "★".repeat(
                    preview.map_or(card.rarity.stars() as usize, |preview| match preview {
                        RarityPreview::Common => 1,
                        RarityPreview::Rare | RarityPreview::ReverseHolo => 2,
                        RarityPreview::Gold | RarityPreview::SuperRare => 3,
                    })
                ),
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
            "\x1b[48;2;36;0;8m",
            "\x1b[48;2;48;0;12m",
            "\x1b[48;2;60;0;16m",
        ][band],
        CardClass::Daemon => [
            "\x1b[48;2;28;8;36m",
            "\x1b[48;2;38;10;48m",
            "\x1b[48;2;48;12;60m",
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
        CardClass::Glitch => "\x1b[48;5;52m",
        CardClass::Daemon => "\x1b[48;5;53m",
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
        CardClass::Glitch => ["\x1b[48;5;196m", "\x1b[48;5;202m", "\x1b[48;5;203m"][band],
        CardClass::Daemon => ["\x1b[48;5;129m", "\x1b[48;5;165m"][band],
        CardClass::Virus => ["\x1b[48;5;40m", "\x1b[48;5;82m"][band],
        CardClass::Bug => ["\x1b[48;5;208m", "\x1b[48;5;214m"][band],
        CardClass::Null => ["\x1b[48;5;250m", "\x1b[48;5;255m"][band],
    }
}

fn reverse_holo_text_color(class: CardClass) -> &'static str {
    match class {
        CardClass::Robot => "\x1b[38;2;0;30;35m",
        CardClass::Glitch => "\x1b[38;2;45;0;0m",
        CardClass::Daemon => "\x1b[38;2;30;0;40m",
        CardClass::Virus => "\x1b[38;2;0;35;10m",
        CardClass::Bug => "\x1b[38;2;45;28;0m",
        CardClass::Null => "\x1b[38;2;25;25;25m",
    }
}
fn panel_color(class: CardClass, column: usize) -> &'static str {
    match class {
        CardClass::Robot => "\x1b[96m",
        CardClass::Glitch => ["\x1b[91m", "\x1b[91m", "\x1b[93m", "\x1b[95m"][column % 4],
        CardClass::Daemon => "\x1b[95m",
        CardClass::Virus => "\x1b[92m",
        CardClass::Bug => "\x1b[38;5;208m",
        CardClass::Null => "\x1b[90m",
    }
}
fn primary_color(class: CardClass) -> &'static str {
    match class {
        CardClass::Robot => "\x1b[96m",
        CardClass::Glitch => "\x1b[91m",
        CardClass::Daemon => "\x1b[95m",
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
    use crate::generator::CardGenerator;
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
        assert!([196, 202, 203].into_iter().all(|color| {
            glitch_reverse
                .iter()
                .any(|background| *background == format!("\x1b[48;5;{color}m"))
        }));
        assert!(!glitch_reverse.contains(&"\x1b[48;5;201m"));
    }

    #[test]
    fn colored_information_background_continues_through_text_and_empty_cells() {
        let mut lines = Vec::new();
        add_shaded(&mut lines, "T ", CardClass::Glitch, true, None);
        assert!(lines[0].contains(&format!(
            "{}\x1b[91mT\x1b[0m{} ",
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
            (CardClass::Daemon, "\x1b[95m"),
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
        assert!(output.contains("\x1b[48;5;52m"));
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
