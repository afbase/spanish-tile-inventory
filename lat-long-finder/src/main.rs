use structopt::StructOpt;
use std::path::PathBuf;
use std::error::Error;
use csv::{Reader, Writer};
use data::inventory::TileInventory;
use utils::csv_parser::geocode_inventory;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str), short = "i", long = "in")]
    input: PathBuf,
    #[structopt(parse(from_os_str), short = "o", long = "out")]
    output: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::from_args();

    let mut reader = Reader::from_path(&args.input)?;
    let mut writer = Writer::from_path(&args.output)?;

    // Read all records into memory
    let mut records: Vec<TileInventory> = reader.deserialize().collect::<Result<_, _>>()?;

    // Geocode all addresses
    geocode_inventory(&mut records).await?;

    // Write the header
    let mut headers = reader.headers()?.clone();
    if !headers.iter().any(|h| h == "latitude") {
        headers.push_field("latitude");
    }
    if !headers.iter().any(|h| h == "longitude") {
        headers.push_field("longitude");
    }
    writer.write_record(&headers)?;

    // Write the records with latitude and longitude
    for record in records {
        writer.serialize(record)?;
    }

    writer.flush()?;
    println!("Processing complete. Output written to {:?}", args.output);

    Ok(())
}