mod analysis_display;
mod js_bindings;
mod map_view;

pub use analysis_display::AnalysisDisplay;
pub use map_view::MapView;

use data::inventory::TileInventory;
use gloo_console as console_logger;
use yew::prelude::*;

pub enum Msg {
    ItemSelected(Option<TileInventory>),
}

// pub struct InventoryView {
//     selected_item: Option<TileInventory>,
// }

#[derive(Properties, PartialEq)]
pub struct InventoryViewProps {
    pub inventory: Vec<TileInventory>,
    pub selected_item: Option<TileInventory>,
    pub on_item_select: Callback<Option<TileInventory>>,
}

pub struct InventoryView;

impl Component for InventoryView {
    type Message = ();
    type Properties = InventoryViewProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_item_select = ctx.props().on_item_select.clone();
        console_logger::log!(
            "InventoryView's inventory count before rendering MapView: ",
            ctx.props().inventory.len()
        );

        html! {
            <div>
                <MapView
                    inventory={ctx.props().inventory.clone()}
                    selected_item={ctx.props().selected_item.clone()}
                    on_item_select={on_item_select.clone()}
                />
                <AnalysisDisplay
                    inventory={ctx.props().inventory.clone()}
                    selected_item={ctx.props().selected_item.clone()}
                    on_item_select={on_item_select}
                />
            </div>
        }
    }
}
