use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct TileInventory {
    pub id: u32,
    pub street_sign: String,
    pub street_address: String,
    pub sign_condition: String,
    pub number_of_tiles_damaged: u32,
    pub latitude: f64,
    pub longitude: f64,
    #[serde(deserialize_with = "deserialize_optional_path")]
    pub photo_1: Option<PathBuf>,
    #[serde(deserialize_with = "deserialize_optional_path")]
    pub photo_2: Option<PathBuf>,
    #[serde(deserialize_with = "deserialize_optional_path")]
    pub photo_3: Option<PathBuf>,
    #[serde(deserialize_with = "deserialize_optional_path")]
    pub photo_4: Option<PathBuf>,
    #[serde(deserialize_with = "deserialize_optional_path")]
    pub photo_5: Option<PathBuf>,
}

fn deserialize_optional_path<'de, D>(deserializer: D) -> Result<Option<PathBuf>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    Ok(s.filter(|s| !s.is_empty()).map(PathBuf::from))
}
