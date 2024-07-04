use csv::ReaderBuilder;
use data::inventory::TileInventory;
use reqwest::blocking::Client;
use serde_json::Value;

pub fn parse_csv(csv_data: &[u8]) -> Result<Vec<TileInventory>, Box<dyn std::error::Error>> {
    let mut reader = ReaderBuilder::new().from_reader(csv_data);
    let mut inventory: Vec<TileInventory> = Vec::new();

    for result in reader.deserialize() {
        let mut record: TileInventory = result?;
        let (lat, lon) = geocode(&record.street_address)?;
        record.latitude = lat;
        record.longitude = lon;
        inventory.push(record);
    }

    Ok(inventory)
}

fn geocode(address: &str) -> Result<(f64, f64), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = format!(
        "https://nominatim.openstreetmap.org/search?format=json&q={},New Orleans",
        address
    );
    let json_blob = client.get(&url).send()?.text()?;
    let response: Value = serde_json::from_str(&json_blob)?;

    if let Some(first_result) = response.as_array().and_then(|arr| arr.first()) {
        let lat = first_result["lat"].as_str().unwrap().parse()?;
        let lon = first_result["lon"].as_str().unwrap().parse()?;
        Ok((lat, lon))
    } else {
        Err("No results found for geocoding".into())
    }
}