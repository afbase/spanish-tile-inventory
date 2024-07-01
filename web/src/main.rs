use yew::prelude::*;
use app::App;
use utils::csv_parser::parse_csv;
use std::str::from_utf8;

fn main() {
    let csv_data = from_utf8(include_bytes!("../../inventory.csv")).unwrap_or("");
    let inventory = parse_csv(csv_data).expect("Failed to parse CSV data");
    // yew::start_app_with_props::<App>(app::Props { inventory });
    yew::Renderer::<App>::with_props(app::Props {inventory});
}