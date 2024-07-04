use data::inventory::TileInventory;
use gloo_net::http::Request;
use js_sys::{Array, Object, Promise, Reflect, JSON};
use leaflet::{
    Icon, IconOptions, LatLng, Map, MapOptions, Marker, Point, Popup, PopupOptions, TileLayer,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use web_sys::{Element, HtmlElement, HtmlInputElement};
use yew::prelude::*;

pub struct MapView {
    map_ref: NodeRef,
    lat_input_ref: NodeRef,
    lon_input_ref: NodeRef,
    addr_input_ref: NodeRef,
    results_ref: NodeRef,
    map: Option<Map>,
    marker: Option<Marker>,
}

pub enum Msg {
    InitMap,
    UpdateCoordinates(f64, f64),
    SearchAddress,
    SearchResults(Vec<NominatimResult>),
    ChooseAddress(f64, f64),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub inventory: Vec<TileInventory>,
    pub on_item_select: Callback<Option<TileInventory>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct NominatimResult {
    lat: String,
    lon: String,
    display_name: String,
}

impl Component for MapView {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::InitMap);

        Self {
            map_ref: NodeRef::default(),
            lat_input_ref: NodeRef::default(),
            lon_input_ref: NodeRef::default(),
            addr_input_ref: NodeRef::default(),
            results_ref: NodeRef::default(),
            map: None,
            marker: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InitMap => {
                if let Some(map_element) = self.map_ref.cast::<Element>() {
                    self.init_map(map_element);
                }
                true
            }
            Msg::UpdateCoordinates(lat, lon) => {
                if let Some(lat_input) = self.lat_input_ref.cast::<HtmlInputElement>() {
                    lat_input.set_value(&lat.to_string());
                }
                if let Some(lon_input) = self.lon_input_ref.cast::<HtmlInputElement>() {
                    lon_input.set_value(&lon.to_string());
                }
                if let Some(marker) = &self.marker {
                    let new_latlng = LatLng::new(lat, lon);
                    marker.set_lat_lng(&new_latlng);
                    if let Some(map) = &self.map {
                        map.set_view(&new_latlng, 18.0);
                    }
                    let popup_content = format!("Lat {:.8}<br />Lon {:.8}", lat, lon);
                    marker.bind_popup(&popup_content);
                    marker.open_popup();
                }
                true
            }
            Msg::SearchAddress => {
                if let Some(addr_input) = self.addr_input_ref.cast::<HtmlInputElement>() {
                    let address = addr_input.value();
                    let link = ctx.link().clone();
                    spawn_local(async move {
                        let url = format!(
                            "https://nominatim.openstreetmap.org/search?format=json&limit=3&q={}",
                            address
                        );
                        let response = Request::get(&url).send().await.unwrap();
                        let results: Vec<NominatimResult> = response.json().await.unwrap();
                        link.send_message(Msg::SearchResults(results));
                    });
                }
                false
            }
            Msg::SearchResults(results) => {
                if let Some(results_div) = self.results_ref.cast::<HtmlElement>() {
                    let mut html = String::new();
                    for result in results {
                        html.push_str(&format!(
                            "<div class='address' onclick='window.choose_addr({}, {});'>{}</div>",
                            result.lat, result.lon, result.display_name
                        ));
                    }
                    results_div.set_inner_html(&html);
                }
                true
            }
            Msg::ChooseAddress(lat, lon) => {
                ctx.link().send_message(Msg::UpdateCoordinates(lat, lon));
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="container">
                <b>{"Coordinates"}</b>
                <form>
                    <input type="text" ref={self.lat_input_ref.clone()} id="lat" size="12" />
                    <input type="text" ref={self.lon_input_ref.clone()} id="lon" size="12" />
                </form>

                <b>{"Address Lookup"}</b>
                <div id="search">
                    <input type="text" ref={self.addr_input_ref.clone()} id="addr" size="58" />
                    <button type="button" onclick={ctx.link().callback(|_| Msg::SearchAddress)}>{"Search"}</button>
                    <div ref={self.results_ref.clone()} id="results"></div>
                </div>

                <br />

                <div ref={self.map_ref.clone()} id="map" style="width:100%;height:400px;"></div>
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let window = web_sys::window().unwrap();
            let closure = ctx
                .link()
                .callback(move |coords: (f64, f64)| Msg::ChooseAddress(coords.0, coords.1));
            let closure = Closure::wrap(Box::new(move |lat: f64, lon: f64| {
                closure.emit((lat, lon));
            }) as Box<dyn Fn(f64, f64)>);
            window.set_onclick(Some(closure.as_ref().unchecked_ref()));
            closure.forget();
        }
    }
}

impl MapView {
    fn init_map(&mut self, element: Element) {
        let map_options = MapOptions::new();
        let html_element: HtmlElement = element.dyn_into().unwrap();
        let map = Map::new_with_element(&html_element, &map_options);

        let start_lat = 40.75637123;
        let start_lon = -73.98545321;
        let center = LatLng::new(start_lat, start_lon);
        map.set_view(&center, 9.0);

        let tile_layer_url = "http://{s}.tile.osm.org/{z}/{x}/{y}.png";
        let tile_layer = TileLayer::new(tile_layer_url);
        map.add_layer(&tile_layer);

        let marker = Marker::new(&center);
        marker.add_to(&map);

        let marker_clone = marker.clone();
        let map_clone = map.clone();
        let link = ctx.link().clone();
        let closure = Closure::wrap(Box::new(move |_| {
            let lat = marker_clone.get_lat_lng().lat();
            let lon = marker_clone.get_lat_lng().lng();
            let zoom = map_clone.get_zoom();
            let new_zoom = if zoom < 18.0 { zoom + 2.0 } else { 18.0 };
            map_clone.set_view(&LatLng::new(lat, lon), new_zoom);
            link.send_message(Msg::UpdateCoordinates(lat, lon));
        }) as Box<dyn Fn()>);
        marker.on("dragend", &closure);
        closure.forget();

        self.map = Some(map);
        self.marker = Some(marker);

        ctx.link()
            .send_message(Msg::UpdateCoordinates(start_lat, start_lon));
    }
}
