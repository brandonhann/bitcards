use bitcards_art_lab::ArtComponent;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

fn main() {
    if let Err(message) = run() {
        eprintln!("error: {message}");
        std::process::exit(2);
    }
}
fn run() -> Result<(), String> {
    let arguments: Vec<String> = env::args().skip(1).collect();
    match arguments.first().map(String::as_str) {
        Some("validate") if arguments.len() > 1 => {
            for path in &arguments[1..] {
                let art = load(path)?;
                println!(
                    "valid: {} ({}x{}, {} anchors)",
                    art.id,
                    art.width,
                    art.height,
                    art.anchors.len()
                );
            }
            Ok(())
        }
        Some("show") if arguments.len() == 2 => {
            let art = load(&arguments[1])?;
            println!("{}\n{}", art.id, art.render());
            Ok(())
        }
        Some("gallery") if arguments.len() == 2 => gallery(Path::new(&arguments[1])),
        _ => {
            Err("usage: bitcards-art-lab <validate FILE... | show FILE | gallery DIRECTORY>".into())
        }
    }
}
fn load(path: impl AsRef<Path>) -> Result<ArtComponent, String> {
    let path = path.as_ref();
    let source =
        fs::read_to_string(path).map_err(|error| format!("{}: {error}", path.display()))?;
    ArtComponent::parse(&source).map_err(|error| format!("{}: {error}", path.display()))
}
fn gallery(directory: &Path) -> Result<(), String> {
    let mut paths: Vec<PathBuf> = fs::read_dir(directory)
        .map_err(|error| format!("{}: {error}", directory.display()))?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|extension| extension == "bca"))
        .collect();
    paths.sort();
    if paths.is_empty() {
        return Err(format!("{} contains no .bca files", directory.display()));
    }
    for path in paths {
        let art = load(&path)?;
        println!("\n=== {} ===\n{}", art.id, art.render());
    }
    Ok(())
}
