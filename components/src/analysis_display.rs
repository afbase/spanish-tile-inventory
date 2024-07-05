use data::inventory::TileInventory;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

pub struct AnalysisDisplay {
    selected_street: Option<String>,
    selected_address: Option<String>,
    current_photo_index: usize,
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
    NextPhoto,
    PreviousPhoto,
}

impl Component for AnalysisDisplay {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            selected_street: None,
            selected_address: None,
            current_photo_index: 0,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::StreetSelected(street) => {
                self.selected_street = Some(street);
                self.selected_address = None;
                self.current_photo_index = 0;
                self.notify_selection(ctx);
                true
            }
            Msg::AddressSelected(address) => {
                self.selected_address = Some(address);
                self.current_photo_index = 0;
                self.notify_selection(ctx);
                true
            }
            Msg::NextPhoto => {
                if let Some(item) = &ctx.props().selected_item {
                    let photos = self.get_photos(item);
                    if !photos.is_empty() {
                        self.current_photo_index = (self.current_photo_index + 1) % photos.len();
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            Msg::PreviousPhoto => {
                if let Some(item) = &ctx.props().selected_item {
                    let photos = self.get_photos(item);
                    if !photos.is_empty() {
                        self.current_photo_index =
                            (self.current_photo_index + photos.len() - 1) % photos.len();
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
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
                    <select value={self.selected_street.clone().unwrap_or_default()} onchange={ctx.link().callback(|e: Event| Msg::StreetSelected(e.target_unchecked_into::<HtmlSelectElement>().value()))}>
                        <option disabled=true selected={self.selected_street.is_none()}>{"Select Street Sign"}</option>
                        { for street_signs.iter().map(|street| html! { <option value={street.clone()} selected={Some(street) == self.selected_street.as_ref()}>{street}</option> }) }
                    </select>
                </div>
                <div>
                    <select value={self.selected_address.clone().unwrap_or_default()} disabled={self.selected_street.is_none()} onchange={ctx.link().callback(|e: Event| Msg::AddressSelected(e.target_unchecked_into::<HtmlSelectElement>().value()))}>
                        <option disabled=true selected={self.selected_address.is_none()}>{"Select Address"}</option>
                        { for addresses.iter().map(|address| html! { <option value={address.clone()} selected={Some(address) == self.selected_address.as_ref()}>{address}</option> }) }
                    </select>
                </div>
                { self.render_selected_item_info(ctx) }
                { self.render_photo_viewer(ctx) }
            </div>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        if let Some(item) = &ctx.props().selected_item {
            self.selected_street = Some(item.street_sign.clone());
            self.selected_address = Some(item.street_address.clone());
            self.current_photo_index = 0;
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
                    <p>{format!("Approximate Latitude: {:?}", item.latitude)}</p>
                    <p>{format!("Approximate Longitude: {:?}", item.longitude)}</p>
                </div>
            }
        } else {
            html! { <p>{"No item selected"}</p> }
        }
    }

    fn render_photo_viewer(&self, ctx: &Context<Self>) -> Html {
        if let Some(item) = &ctx.props().selected_item {
            let photos = self.get_photos(item);
            if !photos.is_empty() {
                html! {
                    <div class="photo-viewer">
                        <img src={photos[self.current_photo_index].clone()} alt="Tile inventory" />
                        <div>
                            <button onclick={ctx.link().callback(|_| Msg::PreviousPhoto)}>{"Previous"}</button>
                            <button onclick={ctx.link().callback(|_| Msg::NextPhoto)}>{"Next"}</button>
                        </div>
                        <p>{format!("Photo {} of {}", self.current_photo_index + 1, photos.len())}</p>
                    </div>
                }
            } else {
                html! { <p>{"No photos available for this sign"}</p> }
            }
        } else {
            html! {}
        }
    }

    fn get_photos(&self, item: &TileInventory) -> Vec<String> {
        vec![
            item.photo_1.clone(),
            item.photo_2.clone(),
            item.photo_3.clone(),
            item.photo_4.clone(),
            item.photo_5.clone(),
        ]
        .into_iter()
        .flatten()
        .flat_map(|p| p.into_os_string().into_string())
        .collect()
    }
}
