use csv::{Reader, Writer};
use data::inventory::TileInventory;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::Duration;
use thiserror::Error;
use tokio::time::sleep;

#[derive(Debug, Deserialize)]
struct NominatimResponse {
    lat: String,
    lon: String,
}

#[derive(Error, Debug)]
pub enum CsvParserError {
    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),
    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to parse latitude or longitude")]
    ParseCoordinate,
    #[error("Failed to deserialize record at line {line}: {source}")]
    Deserialize {
        line: usize,
        source: csv::Error,
    },
}

pub async fn geocode(address: &str) -> Result<(Option<f64>, Option<f64>), CsvParserError> {
    let client = Client::new();
    let url = format!(
        "https://nominatim.openstreetmap.org/search?format=json&q={},New Orleans&limit=1",
        urlencoding::encode(address)
    );

    let request = client
        .get(&url)
        .header("User-Agent", "TileInventoryApp/1.0");

    let response = request.send().await?;
    let response: Vec<NominatimResponse> = response.json().await?;

    if let Some(result) = response.first() {
        let lat = result.lat.parse().map_err(|_| CsvParserError::ParseCoordinate)?;
        let lon = result.lon.parse().map_err(|_| CsvParserError::ParseCoordinate)?;
        Ok((Some(lat), Some(lon)))
    } else {
        Ok((None, None))
    }
}

pub async fn geocode_inventory(inventory: &mut [TileInventory]) -> Result<(), CsvParserError> {
    for item in inventory.iter_mut() {
        if item.latitude.is_none() || item.longitude.is_none() {
            let address = format!("{}, New Orleans", item.street_address);
            let (lat, lon) = geocode(&address).await?;
            item.latitude = lat;
            item.longitude = lon;

            // Respect Nominatim's usage policy (max 1 request per second)
            sleep(Duration::from_secs(1)).await;
        }
    }

    Ok(())
}

pub fn parse_csv<P: AsRef<Path>>(input_path: P) -> Result<Vec<TileInventory>, CsvParserError> {
    let mut reader = Reader::from_path(input_path)?;
    let mut inventory = Vec::new();

    for (index, result) in reader.deserialize::<TileInventory>().enumerate() {
        match result {
            Ok(record) => inventory.push(record),
            Err(err) => {
                return Err(CsvParserError::Deserialize {
                    line: index + 2, // +2 because index is 0-based and we want to count the header row
                    source: err,
                })
            }
        }
    }

    Ok(inventory)
}

pub fn write_csv<P: AsRef<Path>>(
    output_path: P,
    inventory: &[TileInventory],
) -> Result<(), CsvParserError> {
    let mut writer = Writer::from_path(output_path)?;

    // Write the header
    writer.write_record(&[
        "ID",
        "Street Sign",
        "Street Address",
        "Sign Condition",
        "Number of Tiles Damaged",
        "Photo 1",
        "Photo 2",
        "Photo 3",
        "Photo 4",
        "Photo 5",
        "latitude",
        "longitude",
    ])?;

    // Write the records
    for record in inventory {
        writer.serialize(record)?;
    }

    writer.flush()?;
    Ok(())
}