use std::error::Error;

use csv::ReaderBuilder;
use data::inventory::TileInventory;
use reqwest::Client;
use serde_json::Value;

pub fn parse_csv(csv_data: &[u8]) -> Result<Vec<TileInventory>, csv::Error> {
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .from_reader(csv_data);
    
    let mut inventory: Vec<TileInventory> = Vec::new();

    for result in reader.deserialize() {
        let record: TileInventory = result?;
        inventory.push(record);
    }

    Ok(inventory)
}

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

    let response: Vec<NominatimResponse> = client
        .get(&url)
        .header("User-Agent", "TileInventoryApp/1.0")
        .send()
        .await?
        .json()
        .await?;

    if let Some(result) = response.first() {
        let lat = result.lat.parse().ok();
        let lon = result.lon.parse().ok();
        Ok((lat, lon))
    } else {
        Ok((None, None))
    }
}

pub async fn geocode_inventory(inventory: &mut [TileInventory]) -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    for item in inventory.iter_mut() {
        if item.latitude.is_none() || item.longitude.is_none() {
            let address = format!("{}, New Orleans", item.street_address);
            let (lat, lon) = geocode(&address).await?;
            item.latitude = lat;
            item.longitude = lon;

            // Respect Nominatim's usage policy (max 1 request per second)
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }

    Ok(())
}
// pub fn parse_csv(csv_data: &[u8]) -> Result<Vec<TileInventory>, Box<dyn std::error::Error>> {
//     let mut reader = ReaderBuilder::new().from_reader(csv_data);
//     let mut inventory: Vec<TileInventory> = Vec::new();
//
//     for result in reader.deserialize() {
//         let mut record: TileInventory = result?;
//         let (lat, lon) = geocode(&record.street_address)?;
//         record.latitude = lat;
//         record.longitude = lon;
//         inventory.push(record);
//     }
//
//     Ok(inventory)
// }
//
// fn geocode(address: &str) -> Result<(f64, f64), Box<dyn std::error::Error>> {
//     let client = reqwest::blocking::Client::new();
//     let url = format!(
//         "https://nominatim.openstreetmap.org/search?format=json&q={},New Orleans",
//         address
//     );
//     let json_blob = client.get(url).send()?.text()?;
//     let response: Value = serde_json::from_str(&json_blob)?;
//
//     if let Some(first_result) = response.as_array().and_then(|arr| arr.first()) {
//         let lat = first_result["lat"].as_str().unwrap().parse()?;
//         let lon = first_result["lon"].as_str().unwrap().parse()?;
//         Ok((lat, lon))
//     } else {
//         Err("No results found for geocoding".into())
//     }
// }
