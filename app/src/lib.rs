use yew::prelude::*;
use components::{analysis_display::AnalysisDisplay, map_view::MapView};
use data::inventory::TileInventory;

pub struct App {
    selected_item: Option<TileInventory>,
}

pub enum Msg {
    ItemSelected(Option<TileInventory>),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub inventory: Vec<TileInventory>,
}

impl Component for App {
    type Message = Msg;
    type Properties = Props;

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
        html! {
            <div class="container mt-4">
                <h1 class="text-center mb-4">{"Spanish Tile Inventory Analysis"}</h1>
                <AnalysisDisplay 
                    inventory={ctx.props().inventory.clone()} 
                    on_selection={ctx.link().callback(Msg::ItemSelected)}
                />
                <MapView 
                    inventory={ctx.props().inventory.clone()} 
                    selected_item={self.selected_item.clone()}
                />
            </div>
        }
    }
}