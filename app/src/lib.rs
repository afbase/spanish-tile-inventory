use components::{AnalysisDisplay, MapView};
use data::inventory::TileInventory;
use utils::csv_parser::parse_csv;
use yew::prelude::*;

pub struct App {
    inventory: Vec<TileInventory>,
    selected_item: Option<TileInventory>,
}

pub enum Msg {
    ItemSelected(Option<TileInventory>),
    InventoryLoaded(Vec<TileInventory>),
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
            Msg::ItemSelected(item) => {
                self.selected_item = item;
                true
            }
            Msg::InventoryLoaded(inventory) => {
                self.inventory = inventory;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_item_select = ctx.link().callback(Msg::ItemSelected);

        html! {
            <div class="container mt-4">
                <h1 class="text-center mb-4">{"Spanish Tile Inventory Analysis"}</h1>
                <div class="row">
                    <div class="col-md-6">
                        <AnalysisDisplay
                            inventory={self.inventory.clone()}
                            selected_item={self.selected_item.clone()}
                            on_item_select={on_item_select.clone()}
                        />
                    </div>
                    <div class="col-md-6">
                        <MapView
                            inventory={self.inventory.clone()}
                            selected_item={self.selected_item.clone()}
                            on_item_select={on_item_select}
                        />
                    </div>
                </div>
            </div>
        }
    }
}
