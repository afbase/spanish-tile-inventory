use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct TileInventory {
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(rename = "Street Sign")]
    pub street_sign: String,
    #[serde(rename = "Street Address")]
    pub street_address: String,
    #[serde(rename = "Sign Condition")]
    pub sign_condition: String,
    #[serde(rename = "Number of Tiles Damaged")]
    pub number_of_tiles_damaged: u32,
    pub latitude: f64,
    pub longitude: f64,
    #[serde(rename = "Photo 1", deserialize_with = "deserialize_optional_path")]
    pub photo_1: Option<PathBuf>,
    #[serde(rename = "Photo 2", deserialize_with = "deserialize_optional_path")]
    pub photo_2: Option<PathBuf>,
    #[serde(rename = "Photo 3", deserialize_with = "deserialize_optional_path")]
    pub photo_3: Option<PathBuf>,
    #[serde(rename = "Photo 4", deserialize_with = "deserialize_optional_path")]
    pub photo_4: Option<PathBuf>,
    #[serde(rename = "Photo 5", deserialize_with = "deserialize_optional_path")]
    pub photo_5: Option<PathBuf>,
}

fn deserialize_optional_path<'de, D>(deserializer: D) -> Result<Option<PathBuf>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    Ok(s.filter(|s| !s.is_empty()).map(PathBuf::from))
}