use crate::js_bindings::{geocodeAddress, initGeoSearch};
use data::inventory::TileInventory;
use js_sys::{Reflect};
use leaflet::{
    Icon, IconOptions, LatLng, Map, MapOptions, Marker, Point, Popup, PopupOptions, TileLayer,
};
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use web_sys::{Element, HtmlElement};
use yew::prelude::*;

pub struct MapView {
    map_ref: NodeRef,
    map: Option<Map>,
    markers: Vec<Marker>,
    geocoded_inventory: Vec<(TileInventory, LatLng)>,
}

pub enum Msg {
    InitMap,
    DataGeocoded(Vec<(TileInventory, LatLng)>),
    UpdateMarkers,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub inventory: Vec<TileInventory>,
    pub selected_item: Option<TileInventory>,
    pub on_item_select: Callback<Option<TileInventory>>,
}

impl Component for MapView {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::InitMap);

        Self {
            map_ref: NodeRef::default(),
            map: None,
            markers: vec![],
            geocoded_inventory: vec![],
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InitMap => {
                let inventory = ctx.props().inventory.clone();
                let link = ctx.link().clone();
                spawn_local(async move {
                    let geocoded = geocode_inventory(&inventory).await;
                    link.send_message(Msg::DataGeocoded(geocoded));
                });
                false
            }
            Msg::DataGeocoded(geocoded) => {
                self.geocoded_inventory = geocoded;
                if self.map.is_some() {
                    self.add_markers(ctx);
                }
                true
            }
            Msg::UpdateMarkers => {
                self.update_markers(ctx);
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div ref={self.map_ref.clone()} style="height: 400px;"></div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            if let Some(map_element) = self.map_ref.cast::<Element>() {
                self.init_map(map_element);
                if !self.geocoded_inventory.is_empty() {
                    self.add_markers(ctx);
                }
            }
        }
        ctx.link().send_message(Msg::UpdateMarkers);
    }
}

impl MapView {
    fn init_map(&mut self, element: Element) {
        let map_options = MapOptions::new();
        let html_element: HtmlElement = element.dyn_into().unwrap();
        let map = Map::new_with_element(&html_element, &map_options);

        let center = LatLng::new(29.9511, -90.0715);
        map.set_view(&center, 13.0);

        let tile_layer_url = "https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png";
        let tile_layer = TileLayer::new(tile_layer_url);
        map.add_layer(&tile_layer);

        // Initialize GeoSearch
        initGeoSearch(map.as_ref());

        self.map = Some(map);
    }

    fn add_markers(&mut self, ctx: &Context<Self>) {
        if let Some(map) = &self.map {
            for (item, latlng) in &self.geocoded_inventory {
                let marker = Marker::new(latlng);

                let icon_options = IconOptions::new();
                icon_options.set_icon_url("/static/markers/marker-icon-2x-blue.png".to_string());
                icon_options.set_icon_size(Point::new(25.0, 41.0));
                let icon = Icon::new(&icon_options);
                marker.set_icon(&icon);

                let popup_content = format!(
                    "{}: {} damaged tiles",
                    item.street_sign, item.number_of_tiles_damaged
                );
                let popup_options = PopupOptions::new();
                let popup = Popup::new(&popup_options, None);
                popup.set_content(&JsValue::from_str(&popup_content));
                marker.bind_popup(&popup);

                marker.add_to(map);

                let on_item_select = ctx.props().on_item_select.clone();
                let item_clone = item.clone();
                let closure = Closure::wrap(Box::new(move || {
                    on_item_select.emit(Some(item_clone.clone()));
                }) as Box<dyn Fn()>);

                marker.on("click", closure.as_ref().unchecked_ref());
                closure.forget();

                self.markers.push(marker);
            }
        }
    }

    fn update_markers(&self, ctx: &Context<Self>) {
        for ((item, _), marker) in self.geocoded_inventory.iter().zip(self.markers.iter()) {
            let is_selected = ctx
                .props()
                .selected_item
                .as_ref()
                .map_or(false, |selected| selected.id == item.id);
            let icon_url = if is_selected {
                "/static/markers/marker-icon-2x-red.png"
            } else {
                "/static/markers/marker-icon-2x-blue.png"
            };

            let icon_options = IconOptions::new();
            icon_options.set_icon_url(icon_url.to_string());
            icon_options.set_icon_size(Point::new(25.0, 41.0));
            let icon = Icon::new(&icon_options);

            marker.set_icon(&icon);
        }
    }
}

async fn geocode_inventory(inventory: &[TileInventory]) -> Vec<(TileInventory, LatLng)> {
    let mut geocoded = Vec::new();
    for item in inventory {
        let address = format!("{}, New Orleans, LA", item.street_address);
        if let Ok(result) = wasm_bindgen_futures::JsFuture::from(geocodeAddress(&address)).await {
            if let Some(coords) = result.dyn_ref::<js_sys::Object>() {
                let lat = Reflect::get(coords, &JsValue::from_str("lat"))
                    .unwrap()
                    .as_f64()
                    .unwrap();
                let lng = Reflect::get(coords, &JsValue::from_str("lng"))
                    .unwrap()
                    .as_f64()
                    .unwrap();
                geocoded.push((item.clone(), LatLng::new(lat, lng)));
            }
        }
    }
    geocoded
}
