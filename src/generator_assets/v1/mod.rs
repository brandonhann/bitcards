pub mod names;
pub mod palettes;

pub const ROBOT_TEMPLATES: &str = include_str!("templates/robot.txt");

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{canonical::hash_hex, sha256::Sha256};

    #[test]
    fn embedded_robot_templates_are_locked() {
        assert_eq!(ROBOT_TEMPLATES.split("\n---\n").count(), 8);
        assert_eq!(
            hash_hex(&Sha256::digest(ROBOT_TEMPLATES.as_bytes())),
            "45102cfc0c0a8458e326a739b56a0c213b169f0aa574b24ccdb4365dce35554b"
        );
    }
}
