use yew::prelude::*;
use data::inventory::TileInventory;
use components::{MapView, InventoryView};
use utils::csv_parser::parse_csv;
use gloo_console as console;

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
            let csv_data = include_bytes!("../../inventory.csv");
            let inventory = parse_csv(csv_data).expect("Failed to parse CSV data");
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