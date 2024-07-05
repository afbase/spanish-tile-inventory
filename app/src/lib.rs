use components::InventoryView;
use data::inventory::{self, TileInventory};
use gloo_console as console;
use utils::csv_parser::parse_csv_str;
use yew::prelude::*;
pub static INVENTORY_CSV_BYTES: &[u8] = include_bytes!("../../inventory_latlong.csv");
pub struct App {
    inventory: Vec<TileInventory>,
    selected_item: Option<TileInventory>,
}

pub enum Msg {
    InventoryLoaded(Vec<TileInventory>),
    ItemSelected(Option<TileInventory>),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        // Load inventory data
        ctx.link().send_future(async {
            let inventory = parse_csv_str(INVENTORY_CSV_BYTES).expect("Failed to parse CSV data");
            Msg::InventoryLoaded(inventory)
        });

        Self {
            inventory: vec![],
            selected_item: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InventoryLoaded(inventory) => {
                self.inventory = inventory;
                true
            }
            Msg::ItemSelected(item) => {
                self.selected_item = item;
                if let Some(selected_item) = &self.selected_item {
                    console::log!("Selected item ID:", selected_item.id);
                    console::log!("Selected item Street Sign:", &selected_item.street_sign);
                    // Add more fields as needed
                } else {
                    console::log!("No item selected");
                }
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="container mt-4">
                <h1 class="text-center mb-4">{"Spanish Tile Inventory Analysis"}</h1>
                <InventoryView inventory={self.inventory.clone()} />
            </div>
        }
    }
}
