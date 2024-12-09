use std::str::FromStr;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum SwitchPuzzleTiles {
    Blank,
    #[default]
    Ground,
}

impl FromStr for SwitchPuzzleTiles {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inferred_tile_type = match s.to_lowercase().as_str() {
            "blank" => SwitchPuzzleTiles::Blank,
            "ground" => SwitchPuzzleTiles::Ground,
            _ => {
                return Err(String::from(
                    "SwitchPuzzleTiles from_str: Invalid tile type given.",
                ))
            }
        };

        Ok(inferred_tile_type)
    }
}
