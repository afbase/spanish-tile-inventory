use csv::{Reader, ReaderBuilder, Writer};
use data::inventory::TileInventory;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CsvError {
    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to deserialize record at line {line}: {source}")]
    Deserialize { line: usize, source: csv::Error },
}

pub fn parse_csv_str(input_bytes: &[u8]) -> Result<Vec<TileInventory>, CsvError> {
    let mut reader = ReaderBuilder::new()
    .has_headers(true)
    .from_reader(input_bytes);
    let mut inventory = Vec::new();

    for (index, result) in reader.deserialize::<TileInventory>().enumerate() {
        match result {
            Ok(record) => inventory.push(record),
            Err(err) => {
                return Err(CsvError::Deserialize {
                    line: index + 2, // +2 because index is 0-based and we want to count the header row
                    source: err,
                });
            }
        }
    }

    Ok(inventory)
}

pub fn parse_csv<P: AsRef<Path>>(input_path: P) -> Result<Vec<TileInventory>, CsvError> {
    let mut reader = Reader::from_path(input_path)?;
    let mut inventory = Vec::new();

    for (index, result) in reader.deserialize::<TileInventory>().enumerate() {
        match result {
            Ok(record) => inventory.push(record),
            Err(err) => {
                return Err(CsvError::Deserialize {
                    line: index + 2, // +2 because index is 0-based and we want to count the header row
                    source: err,
                });
            }
        }
    }

    Ok(inventory)
}

pub fn write_csv<P: AsRef<Path>>(
    output_path: P,
    inventory: &[TileInventory],
) -> Result<(), CsvError> {
    let mut writer = Writer::from_path(output_path)?;

    // Write the header
    writer.write_record([
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

#[cfg(feature = "no-wasm")]
pub mod geocoding {
    use super::*;
    use reqwest::Client;
    use serde::Deserialize;
    use std::time::Duration;
    use tokio::time::sleep;

    #[derive(Debug, Deserialize)]
    struct NominatimResponse {
        lat: String,
        lon: String,
    }

    #[derive(Error, Debug)]
    pub enum GeocodingError {
        #[error("Request error: {0}")]
        Request(#[from] reqwest::Error),
        #[error("Failed to parse latitude or longitude")]
        ParseCoordinate,
    }

    pub async fn geocode(address: &str) -> Result<(Option<f64>, Option<f64>), GeocodingError> {
        let client = Client::new();
        let url = format!(
            "https://nominatim.openstreetmap.org/search?format=json&q={}&limit=1",
            urlencoding::encode(address)
        );
        println!("{}", url);

        let request = client
            .get(&url)
            .header("User-Agent", "TileInventoryApp/1.0");

        let response = request.send().await?;
        let response: Vec<NominatimResponse> = response.json().await?;
        if let Some(result) = response.first() {
            let lat = result
                .lat
                .parse()
                .map_err(|_| GeocodingError::ParseCoordinate)?;
            let lon = result
                .lon
                .parse()
                .map_err(|_| GeocodingError::ParseCoordinate)?;
            Ok((Some(lat), Some(lon)))
        } else {
            Ok((None, None))
        }
    }

    pub async fn geocode_inventory(inventory: &mut [TileInventory]) -> Result<(), GeocodingError> {
        for item in inventory.iter_mut() {
            if item.latitude.is_none() || item.longitude.is_none() {
                let address = format!("{}, New Orleans", item.street_address);
                let (lat, lon) = geocode(&address).await?;
                println!("Street: {}, Lat/Long:{:?}/{:?}", address, lat, lon);
                item.latitude = lat;
                item.longitude = lon;

                // Respect Nominatim's usage policy (max 1 request per second)
                sleep(Duration::from_secs(1)).await;
            }
        }

        Ok(())
    }
}