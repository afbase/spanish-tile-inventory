use csv::ReaderBuilder;
use data::inventory::TileInventory;

pub fn parse_csv(csv_data: &[u8]) -> Result<Vec<TileInventory>, csv::Error> {
    let mut reader = ReaderBuilder::new().from_reader(csv_data);
    let mut inventory: Vec<TileInventory> = Vec::new();

    for result in reader.deserialize() {
        let record: TileInventory = result?;
        inventory.push(record);
    }

    Ok(inventory)
}
