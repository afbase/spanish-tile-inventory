use data::inventory::TileInventory;
use gloo_console as console_logger;
use leaflet::{
    Icon, IconOptions, LatLng, Map, MapOptions, Marker, MarkerOptions, Point, TileLayer,
};
use std::collections::HashMap;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{Element, HtmlElement};
use yew::prelude::*;

pub struct MapView {
    map_ref: NodeRef,
    map: Option<Map>,
    markers: HashMap<u32, Marker>,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub inventory: Vec<TileInventory>,
    pub selected_item: Option<TileInventory>,
    pub on_item_select: Callback<Option<TileInventory>>,
}

pub enum Msg {
    InitMap,
    UpdateMarkers,
}

impl Component for MapView {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::InitMap);
        console_logger::log!(
            "MapView created with inventory count: ",
            ctx.props().inventory.len()
        );
        Self {
            map_ref: NodeRef::default(),
            map: None,
            markers: HashMap::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InitMap => {
                if let Some(map_element) = self.map_ref.cast::<Element>() {
                    self.init_map(ctx, map_element);
                }
                true
            }
            Msg::UpdateMarkers => {
                self.update_markers(ctx);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        console_logger::log!(
            "MapView view() called with inventory count: ",
            ctx.props().inventory.len()
        );
        html! {
            <div ref={self.map_ref.clone()} style="width:100%;height:400px;"></div>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        console_logger::log!(
            "MapView changed, updating markers. Inventory count: ",
            ctx.props().inventory.len()
        );
        self.update_markers(ctx);
        true
    }
}

impl MapView {
    fn init_map(&mut self, ctx: &Context<Self>, element: Element) {
        console_logger::log!(
            "Initializing map with inventory count: ",
            ctx.props().inventory.len()
        );
        let map_options = MapOptions::new();
        let html_element: HtmlElement = element.dyn_into().unwrap();
        let map = Map::new_with_element(&html_element, &map_options);

        // New Orleans French Quarter coordinates
        let center = LatLng::new(29.9584, -90.0644);
        map.set_view(&center, 15.0);

        self.add_tile_layer(&map);
        self.map = Some(map);
        self.add_markers(ctx);
        console_logger::log!("Map initialized");
    }

    fn add_tile_layer(&self, map: &Map) {
        let tile_layer_url = "https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png";
        let tile_layer = TileLayer::new(tile_layer_url);
        tile_layer.add_to(map);
    }

    fn add_markers(&mut self, ctx: &Context<Self>) {
        if let Some(map) = &self.map {
            for item in &ctx.props().inventory {
                // the following is add_marker
                if let (Some(lat), Some(long)) = (item.latitude, item.longitude) {
                    let lat_long = LatLng::new(lat, long);
                    let icon_options = IconOptions::new();
                    icon_options.set_icon_url("static/markers/marker-icon-blue.png".to_string());
                    icon_options.set_icon_size(Point::new(50.0, 82.0));
                    icon_options.set_icon_anchor(Point::new(25.0, 82.0));
                    icon_options.set_popup_anchor(Point::new(0.0, -82.0));
                    let icon = Icon::new(&icon_options);
                    let marker_options = MarkerOptions::new();
                    marker_options.set_icon(icon);
                    let marker = Marker::new_with_options(&lat_long, &marker_options);

                    let item_clone = item.clone();
                    let on_item_select = ctx.props().on_item_select.clone();
                    let closure = Closure::wrap(Box::new(move || {
                        console_logger::log!("Marker clicked: ", item_clone.street_sign.clone());
                        on_item_select.emit(Some(item_clone.clone()));
                    }) as Box<dyn Fn()>);

                    marker.on("click", closure.as_ref().unchecked_ref());
                    closure.forget();

                    marker.add_to(map);
                    console_logger::log!(
                        "Marker added: ",
                        item.street_sign.clone(),
                        " at ",
                        lat,
                        ", ",
                        long
                    );
                    self.markers.insert(item.id, marker);
                } else {
                    console_logger::log!(
                        "Skipping marker for ",
                        item.street_sign.clone(),
                        " due to missing coordinates"
                    );
                }
                // the above is add_marker
            }
        }
    }
    
    // keeping the function signatures just in case
    // fn add_marker(&mut self, ctx: &Context<Self>, map: &Map, item: &TileInventory) 
    // fn update_marker(&mut self, ctx: &Context<Self>, map: &Map, item: &TileInventory)

    fn update_markers(&mut self, ctx: &Context<Self>) {
        if let Some(map) = &self.map {
            for item in &ctx.props().inventory {
                // the following is update_marker
                if let (Some(lat), Some(long)) = (item.latitude, item.longitude) {
                    let lat_long = LatLng::new(lat, long);

                    if let Some(marker) = self.markers.get(&item.id) {
                        marker.set_lat_lng(&lat_long);
                    } else {
                        // the following is add_marker
                        if let (Some(lat), Some(long)) = (item.latitude, item.longitude) {
                            let lat_long = LatLng::new(lat, long);
                            let icon_options = IconOptions::new();
                            icon_options
                                .set_icon_url("static/markers/marker-icon-blue.png".to_string());
                            icon_options.set_icon_size(Point::new(50.0, 82.0));
                            icon_options.set_icon_anchor(Point::new(25.0, 82.0));
                            icon_options.set_popup_anchor(Point::new(0.0, -82.0));
                            let icon = Icon::new(&icon_options);
                            let marker_options = MarkerOptions::new();
                            marker_options.set_icon(icon);
                            let marker = Marker::new_with_options(&lat_long, &marker_options);

                            let item_clone = item.clone();
                            let on_item_select = ctx.props().on_item_select.clone();
                            let closure = Closure::wrap(Box::new(move || {
                                console_logger::log!(
                                    "Marker clicked: ",
                                    item_clone.street_sign.clone()
                                );
                                on_item_select.emit(Some(item_clone.clone()));
                            })
                                as Box<dyn Fn()>);

                            marker.on("click", closure.as_ref().unchecked_ref());
                            closure.forget();

                            marker.add_to(map);
                            console_logger::log!(
                                "Marker added: ",
                                item.street_sign.clone(),
                                " at ",
                                lat,
                                ", ",
                                long
                            );
                            self.markers.insert(item.id, marker);
                        } else {
                            console_logger::log!(
                                "Skipping marker for ",
                                item.street_sign.clone(),
                                " due to missing coordinates"
                            );
                        }
                        // the above is add_marker
                    }

                    let is_selected = ctx
                        .props()
                        .selected_item
                        .as_ref()
                        .map_or(false, |selected| selected.id == item.id);
                    let icon_options = IconOptions::new();
                    let icon_url = if is_selected {
                        "static/markers/marker-icon-green.png"
                    } else {
                        "static/markers/marker-icon-blue.png"
                    };
                    icon_options.set_icon_url(icon_url.to_string());
                    icon_options.set_icon_size(Point::new(50.0, 82.0));
                    icon_options.set_icon_anchor(Point::new(25.0, 82.0));
                    icon_options.set_popup_anchor(Point::new(0.0, -82.0));
                    let icon = Icon::new(&icon_options);

                    if let Some(marker) = self.markers.get(&item.id) {
                        marker.set_icon(&icon);
                    }

                    if is_selected {
                        console_logger::log!(
                            "Selected marker: ",
                            item.street_sign.clone(),
                            " with icon: ",
                            icon_url
                        );
                    }
                } else {
                    if let Some(marker) = self.markers.remove(&item.id) {
                        marker.remove();
                    }
                    console_logger::log!(
                        "Removed marker for ",
                        item.street_sign.clone(),
                        " due to missing coordinates"
                    );
                }
                // the above is update_marker
            }
        }
    }
}
