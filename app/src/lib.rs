use components::InventoryView;
use data::inventory::TileInventory;
use gloo_console as console_logger;
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

                // Select the first item from the inventory
                self.selected_item = self.inventory.first().cloned();

                if let Some(selected_item) = &self.selected_item {
                    console_logger::log!("Initially selected item ID:", selected_item.id);
                    console_logger::log!(
                        "Initially selected item Street Sign:",
                        &selected_item.street_sign
                    );
                }

                true
            }
            Msg::ItemSelected(item) => {
                self.selected_item = item;
                if let Some(selected_item) = &self.selected_item {
                    console_logger::log!("Selected item ID:", selected_item.id);
                    console_logger::log!("Selected item Street Sign:", &selected_item.street_sign);
                } else {
                    console_logger::log!("No item selected");
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        console_logger::log!(
            "App's inventory count before rendering InventoryView: ",
            self.inventory.len()
        );

        html! {
            <div class="container mt-4">
                <h1 class="text-center mb-4">{"Spanish Tile Inventory Analysis"}</h1>
                <InventoryView
                    inventory={self.inventory.clone()}
                    selected_item={self.selected_item.clone()}
                    on_item_select={ctx.link().callback(Msg::ItemSelected)}
                />
            </div>
        }
    }
}
