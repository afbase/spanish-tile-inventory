pub mod csv_parser;

#[cfg(feature = "no-wasm")]
pub use csv_parser::geocoding;