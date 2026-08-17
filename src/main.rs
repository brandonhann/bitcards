use bitcards::{
    canonical::hash_hex,
    generator::{CURRENT_GENERATOR_VERSION, CardGenerator},
    model::CardClass,
    renderer,
};
use std::io::IsTerminal;

fn main() {
    if let Err(message) = run() {
        eprintln!("error: {message}");
        std::process::exit(2);
    }
}
fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().map(String::as_str) == Some("card")
        && args.get(1).map(String::as_str) == Some("gallery")
    {
        return run_gallery(&args[2..]);
    }
    if args.first().map(String::as_str) != Some("card")
        || args.get(1).map(String::as_str) != Some("generate")
    {
        return Err(usage());
    }
    let mut seed = None;
    let mut set_seed = None;
    let mut type_seed = None;
    let mut metadata = false;
    let mut color = None;
    let mut rarity_preview = None;
    let mut finish_preview = None;
    let mut set_id = 1;
    let mut serial = 1;
    let mut card_number = 1_u16;
    let mut set_size = 100_u16;
    let mut version = CURRENT_GENERATOR_VERSION;
    let mut index = 2;
    while index < args.len() {
        match args[index].as_str() {
            "--seed" => seed = Some(value(&args, &mut index)?),
            "--set-seed" => set_seed = Some(value(&args, &mut index)?),
            "--type-seed" => type_seed = Some(value(&args, &mut index)?),
            "--version" => {
                version = value(&args, &mut index)?
                    .parse()
                    .map_err(|_| "version must be an integer".to_string())?
            }
            "--metadata" => metadata = true,
            "--color" => color = Some(true),
            "--no-color" => color = Some(false),
            "--rarity-preview" => {
                rarity_preview = Some(match value(&args, &mut index)?.as_str() {
                    "common" => renderer::RarityPreview::Common,
                    "rare" => renderer::RarityPreview::Rare,
                    "super-rare" => renderer::RarityPreview::SuperRare,
                    _ => return Err("rarity preview must be common, rare, or super-rare".into()),
                });
            }
            "--finish-preview" => {
                finish_preview = Some(match value(&args, &mut index)?.as_str() {
                    "standard" => renderer::RarityPreview::Common,
                    "holo" => renderer::RarityPreview::Rare,
                    "reverse-holo" => renderer::RarityPreview::ReverseHolo,
                    "gold" => renderer::RarityPreview::Gold,
                    "rainbow-holo" => renderer::RarityPreview::SuperRare,
                    _ => {
                        return Err(
                            "finish preview must be standard, holo, reverse-holo, gold, or rainbow-holo"
                                .into(),
                        );
                    }
                });
            }
            "--serial" => {
                let parsed: u32 = value(&args, &mut index)?
                    .parse()
                    .map_err(|_| "serial must be an integer".to_string())?;
                if !(1..=renderer::MAX_DISPLAY_SERIAL).contains(&parsed) {
                    return Err(format!(
                        "serial must be between 1 and {}",
                        renderer::MAX_DISPLAY_SERIAL
                    ));
                }
                serial = parsed;
            }
            "--set-id" => {
                let parsed: u32 = value(&args, &mut index)?
                    .parse()
                    .map_err(|_| "Set ID must be an integer".to_string())?;
                if !(1..=renderer::MAX_DISPLAY_SET_ID).contains(&parsed) {
                    return Err(format!(
                        "Set ID must be between 1 and {}",
                        renderer::MAX_DISPLAY_SET_ID
                    ));
                }
                set_id = parsed;
            }
            "--card-number" => {
                card_number = value(&args, &mut index)?
                    .parse()
                    .map_err(|_| "card number must be an integer".to_string())?;
            }
            "--set-size" => {
                set_size = value(&args, &mut index)?
                    .parse()
                    .map_err(|_| "set size must be an integer".to_string())?;
            }
            other => return Err(format!("unknown argument: {other}\n{}", usage())),
        }
        index += 1;
    }
    let (set, kind) = match (seed, set_seed, type_seed) {
        (Some(seed), None, None) => (vec![0; 32], parse_hex(&seed)?),
        (None, Some(set), Some(kind)) => (parse_hex(&set)?, parse_hex(&kind)?),
        _ => return Err(usage()),
    };
    let card = CardGenerator::default()
        .generate(version, &set, &kind)
        .map_err(|error| format!("generation failed: {error:?}"))?;
    let use_color = std::env::var_os("NO_COLOR").is_none()
        && color.unwrap_or_else(|| std::io::stdout().is_terminal());
    let preview = finish_preview.or(rarity_preview);
    if card_number == 0 || card_number > set_size {
        return Err("card number must be between 1 and the Set size".into());
    }
    if set_size > renderer::MAX_DISPLAY_CATALOG_SIZE {
        return Err(format!(
            "Set size must not exceed {}",
            renderer::MAX_DISPLAY_CATALOG_SIZE
        ));
    }
    let catalog = renderer::CatalogPosition {
        number: card_number,
        total: set_size,
    };
    let front = if let (true, Some(preview)) = (use_color, preview) {
        renderer::render_catalog_preview(&card, Some(set_id), Some(serial), catalog, preview)
    } else {
        renderer::render_with_catalog_identity(
            &card,
            Some(set_id),
            Some(serial),
            catalog,
            use_color,
        )
    };
    println!(
        "{}",
        renderer::side_by_side(&front, &renderer::render_back())
    );
    println!("Card Type hash: {}", hash_hex(&card.hash));
    if metadata {
        println!("Generator: v{}", card.generator_version);
        println!("Maximum supply: {}", card.maximum_supply);
        println!("Set seed: {}", bytes_hex(&card.set_seed));
        println!("Type seed: {}", bytes_hex(&card.card_type_seed));
    }
    Ok(())
}
fn run_gallery(args: &[String]) -> Result<(), String> {
    let base_seed = match args {
        [] => fresh_gallery_seed(),
        [flag, value] if flag == "--seed" => parse_hex(value)?,
        _ => return Err("usage: bitcards card gallery [--seed <hex>]".into()),
    };
    println!("Gallery seed: {}", bytes_hex(&base_seed));
    let generator = CardGenerator::default();
    let mut cards = vec![None; CardClass::ALL.len()];
    for counter in 0_u32..10_000 {
        let mut type_seed = base_seed.clone();
        type_seed.extend_from_slice(&counter.to_be_bytes());
        let card = generator
            .generate(CURRENT_GENERATOR_VERSION, &[0; 32], &type_seed)
            .map_err(|error| format!("gallery generation failed: {error:?}"))?;
        let index = CardClass::ALL
            .iter()
            .position(|class| *class == card.class)
            .expect("generated class is public");
        cards[index].get_or_insert(card);
        if cards.iter().all(Option::is_some) {
            break;
        }
    }
    if cards.iter().any(Option::is_none) {
        return Err("could not find every card class within 10,000 deterministic seeds".into());
    }
    let finishes = [
        ("STANDARD", renderer::RarityPreview::Common),
        ("HOLO", renderer::RarityPreview::Rare),
        ("REVERSE HOLO", renderer::RarityPreview::ReverseHolo),
        ("GOLD", renderer::RarityPreview::Gold),
        ("RAINBOW HOLO", renderer::RarityPreview::SuperRare),
    ];
    for card in cards.into_iter().flatten() {
        println!(
            "\n{} {}",
            card.class.symbol(),
            card.class.name().to_uppercase()
        );
        let rendered: Vec<_> = finishes
            .iter()
            .map(|(label, finish)| {
                (
                    *label,
                    renderer::render_rarity_preview(&card, Some(1), Some(1), *finish),
                )
            })
            .collect();
        print_gallery_row(&rendered[..3]);
        print_gallery_row(&rendered[3..]);
    }
    Ok(())
}

fn fresh_gallery_seed() -> Vec<u8> {
    use std::time::{SystemTime, UNIX_EPOCH};

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let mut seed = timestamp.to_be_bytes().to_vec();
    seed.extend_from_slice(&std::process::id().to_be_bytes());
    seed
}
fn print_gallery_row(cards: &[(&str, String)]) {
    println!(
        "{}",
        cards
            .iter()
            .map(|(label, _)| format!("{label:^34}"))
            .collect::<Vec<_>>()
            .join("   ")
    );
    let rows: Vec<Vec<_>> = cards
        .iter()
        .map(|(_, card)| card.lines().collect())
        .collect();
    for row in 0..24 {
        println!(
            "{}",
            rows.iter()
                .map(|card| card[row])
                .collect::<Vec<_>>()
                .join("   ")
        );
    }
}
fn value(args: &[String], index: &mut usize) -> Result<String, String> {
    *index += 1;
    args.get(*index)
        .cloned()
        .ok_or_else(|| "missing option value".into())
}
fn parse_hex(value: &str) -> Result<Vec<u8>, String> {
    if value.is_empty() || value.len() % 2 != 0 {
        return Err("seed must be non-empty, even-length hexadecimal".into());
    }
    value
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            let text = std::str::from_utf8(pair).expect("ASCII slice");
            u8::from_str_radix(text, 16)
                .map_err(|_| "seed contains non-hexadecimal characters".into())
        })
        .collect()
}
fn bytes_hex(value: &[u8]) -> String {
    value.iter().map(|byte| format!("{byte:02x}")).collect()
}
fn usage() -> String {
    "usage: bitcards card generate --seed <hex> [--finish-preview standard|holo|reverse-holo|gold|rainbow-holo] [--color|--no-color] [--set-id <1-99>] [--card-number <1-999>] [--set-size <1-999>] [--serial <1-999999>] [--version 1] [--metadata]\n   or: bitcards card generate --set-seed <hex> --type-seed <hex> [--finish-preview standard|holo|reverse-holo|gold|rainbow-holo] [--color|--no-color] [--set-id <1-99>] [--card-number <1-999>] [--set-size <1-999>] [--serial <1-999999>]".into()
}
