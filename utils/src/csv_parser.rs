use csv::{Reader, Writer};
use data::inventory::TileInventory;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::Path;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Deserialize)]
struct NominatimResponse {
    lat: String,
    lon: String,
}

pub async fn geocode(address: &str) -> Result<(Option<f64>, Option<f64>), Box<dyn Error>> {
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
        let lat = result.lat.parse().ok();
        let lon = result.lon.parse().ok();
        Ok((lat, lon))
    } else {
        Ok((None, None))
    }
}

pub async fn geocode_inventory(inventory: &mut [TileInventory]) -> Result<(), Box<dyn Error>> {
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

pub fn parse_csv<P: AsRef<Path>>(input_path: P) -> Result<Vec<TileInventory>, Box<dyn Error>> {
    let mut reader = Reader::from_path(input_path)?;
    let records: Result<Vec<TileInventory>, _> = reader.deserialize().collect();
    Ok(records?)
}

pub fn write_csv<P: AsRef<Path>>(
    output_path: P,
    inventory: &[TileInventory],
) -> Result<(), Box<dyn Error>> {
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