use std::path::PathBuf;
use structopt::StructOpt;
use utils::csv_parser::{parse_csv, write_csv, CsvError};
use utils::geocoding::{geocode_inventory, GeocodingError};

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str), short = "i", long = "in")]
    input: PathBuf,
    #[structopt(parse(from_os_str), short = "o", long = "out")]
    output: PathBuf,
}

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

#[derive(thiserror::Error, Debug)]
enum AppError {
    #[error("CSV error: {0}")]
    Csv(#[from] CsvError),
    #[error("Geocoding error: {0}")]
    Geocoding(#[from] GeocodingError),
}

async fn run() -> Result<(), AppError> {
    let args = Cli::from_args();

    println!("Reading CSV from {:?}", args.input);
    let mut inventory = parse_csv(&args.input)?;
    println!("Successfully read {} records", inventory.len());

    println!("Geocoding addresses...");
    geocode_inventory(&mut inventory).await?;
    println!("Geocoding complete");

    println!("Writing results to {:?}", args.output);
    write_csv(&args.output, &inventory)?;
    println!("Processing complete. Output written to {:?}", args.output);

    Ok(())
}
