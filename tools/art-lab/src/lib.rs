use std::{collections::HashSet, fmt};

pub const TRANSPARENT_CELL: char = '`';
pub const MAX_WIDTH: usize = 32;
pub const MAX_HEIGHT: usize = 16;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Anchor {
    pub name: String,
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtComponent {
    pub id: String,
    pub width: usize,
    pub height: usize,
    pub anchors: Vec<Anchor>,
    pub rows: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub line: usize,
    pub message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "line {}: {}", self.line, self.message)
    }
}

impl std::error::Error for ParseError {}

impl ArtComponent {
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        if input.contains('\r') {
            return Err(error(
                1,
                "carriage returns are forbidden; use LF line endings",
            ));
        }
        let lines: Vec<&str> = input
            .strip_suffix('\n')
            .unwrap_or(input)
            .split('\n')
            .collect();
        if lines.first() != Some(&"BITCARDS-ART 1") {
            return Err(error(1, "expected 'BITCARDS-ART 1'"));
        }
        let id = field(lines.get(1), 2, "id")?;
        if !valid_name(id) {
            return Err(error(
                2,
                "id must contain only lowercase ASCII, digits, '.' or '-'",
            ));
        }
        let size = field(lines.get(2), 3, "size")?;
        let mut dimensions = size.split(' ');
        let width = number(dimensions.next(), 3, "width")?;
        let height = number(dimensions.next(), 3, "height")?;
        if dimensions.next().is_some()
            || width == 0
            || height == 0
            || width > MAX_WIDTH
            || height > MAX_HEIGHT
        {
            return Err(error(3, "size must be two positive integers within 32x16"));
        }

        let mut anchors = Vec::new();
        let mut names = HashSet::new();
        let mut cursor = 3;
        while lines.get(cursor) != Some(&"---") {
            let line_number = cursor + 1;
            let line = lines
                .get(cursor)
                .ok_or_else(|| error(line_number, "missing '---' separator"))?;
            let value = line
                .strip_prefix("anchor ")
                .ok_or_else(|| error(line_number, "expected anchor or '---'"))?;
            let fields: Vec<&str> = value.split(' ').collect();
            if fields.len() != 3 || !valid_name(fields[0]) {
                return Err(error(line_number, "anchor must be: anchor <name> <x> <y>"));
            }
            let x = number(fields.get(1).copied(), line_number, "anchor x")?;
            let y = number(fields.get(2).copied(), line_number, "anchor y")?;
            if x >= width || y >= height {
                return Err(error(line_number, "anchor is outside the component grid"));
            }
            if !names.insert(fields[0]) {
                return Err(error(line_number, "duplicate anchor name"));
            }
            anchors.push(Anchor {
                name: fields[0].into(),
                x,
                y,
            });
            cursor += 1;
        }
        cursor += 1;
        let rows = &lines[cursor..];
        if rows.len() != height {
            return Err(error(
                cursor + 1,
                format!("expected {height} art rows, found {}", rows.len()),
            ));
        }
        for (offset, row) in rows.iter().enumerate() {
            if row.chars().count() != width {
                return Err(error(
                    cursor + offset + 1,
                    format!("expected width {width}, found {}", row.chars().count()),
                ));
            }
            if let Some(character) = row
                .chars()
                .find(|character| !valid_art_character(*character))
            {
                return Err(error(
                    cursor + offset + 1,
                    format!("invalid art character '{character}'"),
                ));
            }
        }
        Ok(Self {
            id: id.into(),
            width,
            height,
            anchors,
            rows: rows.iter().map(|row| (*row).into()).collect(),
        })
    }

    #[must_use]
    pub fn render(&self) -> String {
        self.rows
            .iter()
            .map(|row| row.replace(TRANSPARENT_CELL, " "))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

fn field<'a>(line: Option<&&'a str>, number: usize, name: &str) -> Result<&'a str, ParseError> {
    line.and_then(|line| line.strip_prefix(&format!("{name} ")))
        .filter(|value| !value.is_empty())
        .ok_or_else(|| error(number, format!("expected '{name} <value>'")))
}
fn number(value: Option<&str>, line: usize, name: &str) -> Result<usize, ParseError> {
    value
        .ok_or_else(|| error(line, format!("missing {name}")))?
        .parse()
        .map_err(|_| error(line, format!("{name} must be an unsigned integer")))
}
fn valid_name(value: &str) -> bool {
    !value.is_empty()
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'.' || byte == b'-'
        })
}
fn valid_art_character(value: char) -> bool {
    value == TRANSPARENT_CELL || (value.is_ascii_graphic() && !value.is_ascii_alphanumeric())
}
fn error(line: usize, message: impl Into<String>) -> ParseError {
    ParseError {
        line,
        message: message.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const VALID: &str =
        "BITCARDS-ART 1\nid bug.body.test\nsize 5 3\nanchor head 2 0\n---\n``*``\n`/|\\`\n/`|`\\\n";
    #[test]
    fn parses_and_renders_valid_component() {
        let art = ArtComponent::parse(VALID).unwrap();
        assert_eq!(art.id, "bug.body.test");
        assert_eq!(art.render(), "  *  \n /|\\ \n/ | \\");
    }
    #[test]
    fn rejects_wrong_width() {
        assert!(
            ArtComponent::parse(&VALID.replace("``*``", "`*``"))
                .unwrap_err()
                .message
                .contains("width")
        );
    }
    #[test]
    fn rejects_letters_in_artwork() {
        assert!(
            ArtComponent::parse(&VALID.replace("``*``", "``A``"))
                .unwrap_err()
                .message
                .contains("invalid art")
        );
    }
    #[test]
    fn rejects_duplicate_anchors() {
        assert!(
            ArtComponent::parse(&VALID.replace("---", "anchor head 1 1\n---"))
                .unwrap_err()
                .message
                .contains("duplicate")
        );
    }
}
