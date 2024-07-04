mod analysis_display;
mod js_bindings;
mod map_view;

pub use analysis_display::AnalysisDisplay;
pub use js_bindings::{geocodeAddress, initGeoSearch};
pub use map_view::MapView;

use yew::prelude::*;
use data::inventory::TileInventory;

#[derive(Properties, PartialEq)]
pub struct InventoryViewProps {
    pub inventory: Vec<TileInventory>,
}

pub enum Msg {
    ItemSelected(Option<TileInventory>),
}

pub struct InventoryView {
    selected_item: Option<TileInventory>,
}

impl Component for InventoryView {
    type Message = Msg;
    type Properties = InventoryViewProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            selected_item: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ItemSelected(item) => {
                self.selected_item = item;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_item_select = ctx.link().callback(Msg::ItemSelected);
        
        html! {
            <div>
                <MapView 
                    inventory={ctx.props().inventory.clone()} 
                    selected_item={self.selected_item.clone()}
                    on_item_select={on_item_select.clone()}
                />
                <AnalysisDisplay 
                    inventory={ctx.props().inventory.clone()}
                    selected_item={self.selected_item.clone()}
                    on_item_select={on_item_select}
                />
            </div>
        }
    }
}