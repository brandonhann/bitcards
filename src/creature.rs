use crate::{
    generator_assets::v1::{ROBOT_TEMPLATES, palettes},
    hash_stream::HashStream,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatureDesign {
    pub artwork: Vec<String>,
    pub attack_word: &'static str,
    pub ability_word: &'static str,
}

pub trait CreatureGenerator: Send + Sync {
    fn generate(&self, stream: &mut HashStream) -> CreatureDesign;
}

fn choose<T: Copy>(stream: &mut HashStream, values: &[T]) -> T {
    values[stream
        .next_bounded(values.len() as u32)
        .expect("non-empty choices") as usize]
}

fn vary_silhouette(stream: &mut HashStream, rows: Vec<String>) -> Vec<String> {
    const EXTENSIONS: &[(&str, &str)] = &[
        ("", ""),
        ("<", ">"),
        ("--", "--"),
        ("/", "\\"),
        ("~<", ">~"),
        ("[", "]"),
    ];
    rows.into_iter()
        .enumerate()
        .map(|(row, value)| {
            if (1..=5).contains(&row) {
                let extension = choose(stream, EXTENSIONS);
                format!("{}{}{}", extension.0, value.trim(), extension.1)
            } else {
                value
            }
        })
        .collect()
}

macro_rules! art {
    ($($row:expr),* $(,)?) => { vec![$($row.to_string()),*] };
}

pub struct DragonGenerator;
impl CreatureGenerator for DragonGenerator {
    fn generate(&self, stream: &mut HashStream) -> CreatureDesign {
        let horns = choose(stream, &["^^", "//", "\\\\"]);
        let eyes = choose(stream, palettes::CREATURE_EYES);
        let breath = choose(stream, &["~~~>", "***>", "===>"]);
        CreatureDesign {
            artwork: art![
                format!("          {horns}"),
                format!(r"      ___/{eyes} {eyes}\___"),
                format!(r"  ___/     ^     \__{breath}"),
                r" <___    /\     __/",
                r"     \__====___/",
                r"       / /  \ \",
                r"      /_/    \_\"
            ],
            attack_word: choose(stream, &["Claw", "Flare", "Wing"]),
            ability_word: choose(stream, &["Roar", "Scorch", "Soar"]),
        }
    }
}

pub struct BlobGenerator;
impl CreatureGenerator for BlobGenerator {
    fn generate(&self, stream: &mut HashStream) -> CreatureDesign {
        let shape = stream.next_bounded(4).expect("shape variants");
        let eye = choose(stream, palettes::CREATURE_EYES);
        let mouth = choose(stream, palettes::BLOB_MOUTHS);
        let bump = choose(stream, palettes::BLOB_BUMPS);
        let arms = choose(stream, palettes::BLOB_ARMS);
        let artwork = match shape {
            0 => art![
                format!("         .-{bump}-."),
                "       .'        '._",
                format!("      /   {eye}    {eye}   \\"),
                format!("   {} |              | {}", arms.0, arms.1),
                format!("     |     {mouth}      |"),
                r"      \   _    _   /",
                r"       '._/ \__/ '"
            ],
            1 => art![
                format!("          .{bump}."),
                "         /      \\",
                format!("        | {eye}  {eye} |"),
                format!("     {} |      | {}", arms.0, arms.1),
                format!("        |  {mouth} |"),
                "        |      |",
                "        '------'"
            ],
            2 => art![
                format!("       .--{bump}--."),
                "    .-'            '-.",
                format!(" {} /    {eye}      {eye}    \\ {}", arms.0, arms.1),
                "  |                  |",
                format!("  |       {mouth}        |"),
                "   '._            _.'",
                "      '----------'"
            ],
            _ => art![
                "",
                format!("          {eye}  {eye}"),
                format!("    {} .--{bump}--. {}", arms.0, arms.1),
                "   .-'              '-.",
                format!("  /        {mouth}         \\"),
                " /____________________\\",
                "   **            **"
            ],
        };
        CreatureDesign {
            artwork,
            attack_word: choose(stream, &["Splat", "Bounce", "Gloop"]),
            ability_word: choose(stream, &["Divide", "Ooze", "Absorb"]),
        }
    }
}

pub struct RobotGenerator;
impl CreatureGenerator for RobotGenerator {
    fn generate(&self, stream: &mut HashStream) -> CreatureDesign {
        let archetype = stream.next_bounded(8).expect("robot archetypes") as usize;
        let antenna = choose(stream, palettes::ANTENNAS);
        let eyes = choose(stream, palettes::EYES);
        let joint = choose(stream, palettes::JOINTS);
        let feet = choose(stream, palettes::FEET);
        let panel: String = (0..7)
            .map(|_| choose(stream, palettes::PANEL_GLYPHS))
            .collect();
        let template = ROBOT_TEMPLATES
            .split("\n---\n")
            .nth(archetype)
            .expect("eight versioned templates");
        let artwork = template
            .lines()
            .map(|row| {
                row.replace("{antenna}", antenna)
                    .replace("{eyes}", eyes)
                    .replace("{joint}", joint)
                    .replace("{panel}", &panel)
                    .replace("{left_foot}", feet.0)
                    .replace("{right_foot}", feet.1)
            })
            .collect();
        let artwork = vary_silhouette(stream, artwork);
        CreatureDesign {
            artwork,
            attack_word: choose(stream, &["Pulse", "Ram", "Zap"]),
            ability_word: choose(stream, &["Reboot", "Scan", "Overclock"]),
        }
    }
}

pub struct GlitchGenerator;
impl CreatureGenerator for GlitchGenerator {
    fn generate(&self, stream: &mut HashStream) -> CreatureDesign {
        let glyphs = ['/', '\\', '#', '*', '+', '=', ':'];
        let artwork = (0..7)
            .map(|row| {
                let indent = stream.next_bounded(7).unwrap() as usize;
                let width = 10 + stream.next_bounded(12).unwrap() as usize;
                let mut value = " ".repeat(indent);
                for column in 0..width {
                    let visible = stream.next_bounded(100).unwrap() < 48 || column == width / 2;
                    value.push(if visible {
                        choose(stream, &glyphs)
                    } else {
                        ' '
                    });
                }
                if row == 3 {
                    value.push_str(" <#> ");
                }
                value
            })
            .collect();
        CreatureDesign {
            artwork,
            attack_word: choose(stream, &["Fracture", "Desync", "Jitter"]),
            ability_word: choose(stream, &["Corrupt", "Scramble", "Rollback"]),
        }
    }
}

pub struct DaemonGenerator;
impl CreatureGenerator for DaemonGenerator {
    fn generate(&self, stream: &mut HashStream) -> CreatureDesign {
        let eye = choose(stream, &["*", "+", "@", "^"]);
        let aura = choose(stream, &["~", ".", "'", ":"]);
        let crown = choose(stream, &["/\\", "^^", "(*)", "<>"]);
        let fringe: String = (0..15)
            .map(|_| choose(stream, &['/', '\\', '~', '^', '_']))
            .collect();
        let artwork = art![
            format!("         {crown}"),
            format!("      {aura}.-''''-.{aura}"),
            format!("     /  {eye}    {eye}  \\"),
            format!("  {aura}<|     --     |>{aura}"),
            "     |  .----.  |",
            "      \\ |  | /",
            format!("       {fringe}")
        ];
        CreatureDesign {
            artwork: vary_silhouette(stream, artwork),
            attack_word: choose(stream, &["Haunt", "Phase", "Whisper"]),
            ability_word: choose(stream, &["Monitor", "Persist", "Fork"]),
        }
    }
}

pub struct VirusGenerator;
impl CreatureGenerator for VirusGenerator {
    fn generate(&self, stream: &mut HashStream) -> CreatureDesign {
        let glyph = choose(stream, &['*', '#', '@', '+']);
        let eye = choose(stream, &['.', ':', '*', '+']);
        let bases = [3usize, 9, 15, 19, 15, 9, 3];
        let artwork = bases
            .into_iter()
            .enumerate()
            .map(|(row, base)| {
                let width = (base + stream.next_bounded(5).unwrap() as usize).min(23);
                let mut body = String::new();
                if stream.next_bounded(2).unwrap() == 0 {
                    body.push('-');
                }
                body.push(if row == 0 { '^' } else { '/' });
                for column in 0..width {
                    body.push(
                        if row == 3 && (column == width / 3 || column == width * 2 / 3) {
                            eye
                        } else if stream.next_bounded(4).unwrap() == 0 {
                            '.'
                        } else {
                            glyph
                        },
                    );
                }
                body.push(if row == 6 { '^' } else { '\\' });
                if stream.next_bounded(2).unwrap() == 0 {
                    body.push('-');
                }
                body
            })
            .collect();
        CreatureDesign {
            artwork,
            attack_word: choose(stream, &["Infect", "Mutate", "Splice"]),
            ability_word: choose(stream, &["Replicate", "Quarantine", "Exploit"]),
        }
    }
}

pub struct BugGenerator;
impl CreatureGenerator for BugGenerator {
    fn generate(&self, stream: &mut HashStream) -> CreatureDesign {
        let eye = choose(stream, &['*', '+', '@']);
        let shell = choose(stream, &['#', '=', '+', '*']);
        let mut canvas = [[' '; 21]; 7];
        let center = 10usize;
        let antenna_span = 3 + stream.next_bounded(6).unwrap() as usize;
        canvas[0][center - antenna_span] = '\\';
        canvas[0][center + antenna_span] = '/';
        canvas[0][center - 1] = '\\';
        canvas[0][center + 1] = '/';
        let head_radius = 2 + stream.next_bounded(3).unwrap() as usize;
        for (column, slot) in canvas[1]
            .iter_mut()
            .enumerate()
            .take(center + head_radius + 1)
            .skip(center - head_radius)
        {
            *slot = if column == center - head_radius {
                '/'
            } else if column == center + head_radius {
                '\\'
            } else {
                shell
            };
        }
        canvas[1][center - 1] = eye;
        canvas[1][center + 1] = eye;
        for (row, density) in [(2usize, 42u32), (3, 68), (4, 50)] {
            for offset in 3..=9usize {
                if stream.next_bounded(100).unwrap() < density {
                    let glyph = choose(stream, &[shell, '~', '=', '.', ':']);
                    canvas[row][center - offset] = glyph;
                    canvas[row][center + offset] = glyph;
                }
            }
            let body_radius = 1 + stream.next_bounded(3).unwrap() as usize;
            for (column, slot) in canvas[row]
                .iter_mut()
                .enumerate()
                .take(center + body_radius + 1)
                .skip(center - body_radius)
            {
                *slot = if column == center - body_radius || column == center + body_radius {
                    '|'
                } else {
                    choose(stream, &[shell, '.', ':'])
                };
            }
        }
        for offset in 2..=9usize {
            if stream.next_bounded(100).unwrap() < 58 {
                canvas[5][center - offset] = choose(stream, &['/', '<', '_']);
                canvas[5][center + offset] = choose(stream, &['\\', '>', '_']);
            }
        }
        let tail_radius = 1 + stream.next_bounded(4).unwrap() as usize;
        canvas[6][center - tail_radius] = '/';
        canvas[6][center] = shell;
        canvas[6][center + tail_radius] = '\\';
        let artwork = canvas
            .into_iter()
            .map(|row| row.into_iter().collect())
            .collect();
        CreatureDesign {
            artwork,
            attack_word: choose(stream, &["Crawl", "Pincer", "Swarm"]),
            ability_word: choose(stream, &["Crash", "Patch", "Burrow"]),
        }
    }
}

pub struct NullGenerator;
impl CreatureGenerator for NullGenerator {
    fn generate(&self, stream: &mut HashStream) -> CreatureDesign {
        let glyphs = ['.', ':', '+', '*', '#', '@'];
        let artwork = (0..7)
            .map(|row| {
                let mut half = Vec::with_capacity(10);
                for column in 0..10 {
                    let threshold = 25 + ((row + column) % 4) * 12;
                    half.push(if stream.next_bounded(100).unwrap() < threshold {
                        choose(stream, &glyphs)
                    } else {
                        ' '
                    });
                }
                let center = choose(stream, &['|', ':', '*', '+']);
                let mut value: String = half.iter().collect();
                value.push(center);
                value.extend(half.iter().rev());
                value
            })
            .collect();
        CreatureDesign {
            artwork,
            attack_word: choose(stream, &["Erase", "Zero", "Blank"]),
            ability_word: choose(stream, &["Silence", "Reset", "Vacuum"]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn drawing_algorithms_are_distinct_and_deterministic() {
        let algorithms: [&dyn CreatureGenerator; 6] = [
            &RobotGenerator,
            &GlitchGenerator,
            &DaemonGenerator,
            &VirusGenerator,
            &BugGenerator,
            &NullGenerator,
        ];
        let mut art = HashSet::new();
        for algorithm in algorithms {
            let mut first_stream = HashStream::new(1, &[0; 32], &[7]).unwrap();
            let mut second_stream = HashStream::new(1, &[0; 32], &[7]).unwrap();
            let first = algorithm.generate(&mut first_stream);
            assert_eq!(first, algorithm.generate(&mut second_stream));
            art.insert(first.artwork);
        }
        assert_eq!(art.len(), 6);
    }

    #[test]
    fn large_sample_has_no_collisions_and_broad_silhouette_diversity() {
        let algorithms: [&dyn CreatureGenerator; 6] = [
            &RobotGenerator,
            &GlitchGenerator,
            &DaemonGenerator,
            &VirusGenerator,
            &BugGenerator,
            &NullGenerator,
        ];
        for algorithm in algorithms {
            let mut artworks = HashSet::new();
            let mut silhouettes = HashSet::new();
            for seed in 0..512u16 {
                let mut stream = HashStream::new(1, &[0; 32], &seed.to_be_bytes()).unwrap();
                let rows = algorithm.generate(&mut stream).artwork;
                silhouettes.insert(
                    rows.iter()
                        .map(|row| {
                            row.chars()
                                .map(|c| if c == ' ' { ' ' } else { '#' })
                                .collect::<String>()
                        })
                        .collect::<Vec<_>>(),
                );
                artworks.insert(rows);
            }
            assert_eq!(artworks.len(), 512);
            assert!(
                silhouettes.len() > 170,
                "only {} silhouettes",
                silhouettes.len()
            );
        }
    }
}
