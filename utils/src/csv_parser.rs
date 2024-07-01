use csv::ReaderBuilder;
use data::inventory::TileInventory;
use wasm_bindgen::JsValue;

pub fn parse_csv(csv_data: &str) -> Result<Vec<TileInventory>, JsValue> {
    let mut reader = ReaderBuilder::new().from_reader(csv_data.as_bytes());
    let mut inventory: Vec<TileInventory> = Vec::new();

    for result in reader.deserialize() {
        let record: TileInventory = result.map_err(|e| JsValue::from_str(&e.to_string()))?;
        inventory.push(record);
    }

    Ok(inventory)
}