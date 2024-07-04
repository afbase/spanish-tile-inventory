use data::inventory::TileInventory;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

pub struct AnalysisDisplay {
    selected_street: Option<String>,
    selected_address: Option<String>,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub inventory: Vec<TileInventory>,
    pub selected_item: Option<TileInventory>,
    pub on_item_select: Callback<Option<TileInventory>>,
}

pub enum Msg {
    StreetSelected(String),
    AddressSelected(String),
}

impl Component for AnalysisDisplay {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            selected_street: None,
            selected_address: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::StreetSelected(street) => {
                self.selected_street = Some(street);
                self.selected_address = None;
                self.notify_selection(ctx);
                true
            }
            Msg::AddressSelected(address) => {
                self.selected_address = Some(address);
                self.notify_selection(ctx);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let street_signs: Vec<String> = ctx
            .props()
            .inventory
            .iter()
            .map(|item| item.street_sign.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        let addresses: Vec<String> = if let Some(street) = &self.selected_street {
            ctx.props()
                .inventory
                .iter()
                .filter(|item| &item.street_sign == street)
                .map(|item| item.street_address.clone())
                .collect()
        } else {
            vec![]
        };

        html! {
            <div>
                <h2>{"Inventory Analysis"}</h2>
                <div>
                    <select onchange={ctx.link().callback(|e: Event| Msg::StreetSelected(e.target_unchecked_into::<HtmlSelectElement>().value()))}>
                        <option selected=true disabled=true>{"Select Street Sign"}</option>
                        { for street_signs.iter().map(|street| html! { <option value={street.clone()}>{street}</option> }) }
                    </select>
                </div>
                <div>
                    <select disabled={self.selected_street.is_none()} onchange={ctx.link().callback(|e: Event| Msg::AddressSelected(e.target_unchecked_into::<HtmlSelectElement>().value()))}>
                        <option selected=true disabled=true>{"Select Address"}</option>
                        { for addresses.iter().map(|address| html! { <option value={address.clone()}>{address}</option> }) }
                    </select>
                </div>
                { self.render_selected_item_info(ctx) }
            </div>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        if let Some(item) = &ctx.props().selected_item {
            self.selected_street = Some(item.street_sign.clone());
            self.selected_address = Some(item.street_address.clone());
            true
        } else {
            false
        }
    }
}

impl AnalysisDisplay {
    fn notify_selection(&self, ctx: &Context<Self>) {
        let selected_item = ctx
            .props()
            .inventory
            .iter()
            .find(|item| {
                Some(&item.street_sign) == self.selected_street.as_ref()
                    && Some(&item.street_address) == self.selected_address.as_ref()
            })
            .cloned();
        ctx.props().on_item_select.emit(selected_item);
    }

    fn render_selected_item_info(&self, ctx: &Context<Self>) -> Html {
        if let Some(item) = &ctx.props().selected_item {
            html! {
                <div>
                    <h3>{"Selected Item"}</h3>
                    <p>{format!("Street Sign: {}", item.street_sign)}</p>
                    <p>{format!("Address: {}", item.street_address)}</p>
                    <p>{format!("Damaged Tiles: {}", item.number_of_tiles_damaged)}</p>
                    <p>{format!("Latitude: {}", item.latitude)}</p>
                    <p>{format!("Longitude: {}", item.longitude)}</p>
                </div>
            }
        } else {
            html! { <p>{"No item selected"}</p> }
        }
    }
}
