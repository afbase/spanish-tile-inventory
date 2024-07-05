use std::{collections::HashMap};
use data::inventory::TileInventory;
use gloo_console as console_logger;
use leaflet::{
    Icon, IconOptions, LatLng, Map, MapOptions, Marker, MarkerOptions, Point, Popup, PopupOptions, TileLayer,
};
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{window, Element, HtmlElement, HtmlImageElement};
use yew::prelude::*;

pub struct MapView {
    map_ref: NodeRef,
    map: Option<Map>,
    markers: HashMap<TileInventory, Marker>,
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
        console_logger::log!("MapView created with inventory count: ", ctx.props().inventory.len());
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
        console_logger::log!("MapView view() called with inventory count: ", ctx.props().inventory.len());
        html! {
            <div ref={self.map_ref.clone()} style="width:100%;height:400px;"></div>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        console_logger::log!("MapView changed, updating markers. Inventory count: ", ctx.props().inventory.len());
        ctx.link().send_message(Msg::UpdateMarkers);
        true
    }
}

impl MapView {
    fn init_map(&mut self, ctx: &Context<Self>, element: Element) {
        console_logger::log!("Initializing map with inventory count: ", ctx.props().inventory.len());
        let map_options = MapOptions::new();
        let html_element: HtmlElement = element.dyn_into().unwrap();
        let map = Map::new_with_element(&html_element, &map_options);

        // New Orleans French Quarter coordinates
        let center = LatLng::new(29.9584, -90.0644);
        map.set_view(&center, 15.0);

        let tile_layer_url = "https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png";
        let tile_layer = TileLayer::new(tile_layer_url);
        map.add_layer(&tile_layer);

        self.create_markers(ctx, &map);

        self.map = Some(map);
        console_logger::log!("Map initialized");
    }

    fn create_markers(&mut self, ctx: &Context<Self>, map: &Map) {
        let inventory_count = ctx.props().inventory.len();
        console_logger::log!("Creating markers. Inventory count: ", inventory_count);
        for item in &ctx.props().inventory {
            if let (Some(lat), Some(long)) = (item.latitude, item.longitude) {
                let lat_long = LatLng::new(lat, long);
                let icon_options = IconOptions::new();
                icon_options.set_icon_url("static/markers/marker-icon-2x-blue.png".to_string());
                icon_options.set_icon_size(Point::new(50.0, 82.0));
                icon_options.set_icon_anchor(Point::new(25.0, 82.0));
                icon_options.set_popup_anchor(Point::new(0.0, -82.0));
                let icon = Icon::new(&icon_options);
                let marker_options = MarkerOptions::new();
                marker_options.set_icon(icon);
                let marker = Marker::new_with_options(&lat_long, &marker_options);
                let popup_content = format!(
                    "{}: {:?} damaged tiles",
                    item.street_sign, item.number_of_tiles_damaged
                );
                let popup_options = PopupOptions::new();
                let popup = Popup::new_with_lat_lng(&lat_long, &popup_options);
                popup.set_content(&JsValue::from_str(&popup_content));
                marker.bind_popup(&popup);

                let item_clone = item.clone();
                let on_item_select = ctx.props().on_item_select.clone();
                let closure = Closure::wrap(Box::new(move || {
                    console_logger::log!("Marker clicked: ", item_clone.street_sign.clone());
                    on_item_select.emit(Some(item_clone.clone()));
                }) as Box<dyn Fn()>);

                marker.on("click", closure.as_ref().unchecked_ref());
                closure.forget();

                marker.add_to(map);
                console_logger::log!("Marker added: ", item.street_sign.clone(), " at ", lat, ", ", long);
                self.markers.insert(item.clone(), marker);
            } else {
                console_logger::log!("Skipping marker for ", item.street_sign.clone(), " due to missing coordinates");
            }
        }
        console_logger::log!("Total markers created: ", self.markers.len());
        self.update_markers(ctx);
    }

    fn derive_markers(&mut self, inventory: &Vec<TileInventory>) {
        self.markers = inventory
        .iter()
        .flat_map(|tile_inventory|{
            if let (Some(lat), Some(long)) = (tile_inventory.latitude, tile_inventory.longitude) {
                let lat_long = LatLng::new(lat, long);
                console_logger::log!("Marker {}: lat: {}, lng: {}", lat_long.lat(), lat_long.lng());
                let icon_options = IconOptions::new();
                let icon_url = "static/markers/marker-icon-2x-red.png";
                icon_options.set_icon_url(icon_url.to_string());
                icon_options.set_icon_size(Point::new(50.0, 82.0));
                icon_options.set_icon_anchor(Point::new(25.0, 82.0));
                icon_options.set_popup_anchor(Point::new(0.0, -82.0));
                let icon = Icon::new(&icon_options);
                let marker_options = MarkerOptions::new();
                marker_options.set_icon(icon);
                let marker = Marker::new_with_options(&lat_long, &marker_options);
                Some((tile_inventory.clone(), marker))
            } else {
                None
            }
        })
        .collect::<_>();
    }

    fn update_markers(&mut self, ctx: &Context<Self>) {
        let inventory = ctx.props().inventory.clone();
        let inventory_count = inventory.len() as u32;
        console_logger::log!("Updating markers. Inventory count: ", inventory_count);
        if inventory_count > 0 {
            self.derive_markers(&inventory);
        }
    
        // Check if marker images exist
        let window = window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        
        let red_marker: HtmlImageElement = document.create_element("img")
            .unwrap()
            .dyn_into::<HtmlImageElement>()
            .unwrap();
        red_marker.set_src("static/markers/marker-icon-2x-red.png");
        
        let blue_marker: HtmlImageElement = document.create_element("img")
            .unwrap()
            .dyn_into::<HtmlImageElement>()
            .unwrap();
        blue_marker.set_src("static/markers/marker-icon-2x-blue.png");
    
        let red_onload = Closure::wrap(Box::new(|| {
            console_logger::log!("Red marker image loaded successfully");
        }) as Box<dyn Fn()>);
        red_marker.set_onload(Some(red_onload.as_ref().unchecked_ref()));
        red_onload.forget();
    
        let red_onerror = Closure::wrap(Box::new(|| {
            console_logger::log!("Failed to load red marker image");
        }) as Box<dyn Fn()>);
        red_marker.set_onerror(Some(red_onerror.as_ref().unchecked_ref()));
        red_onerror.forget();
    
        let blue_onload = Closure::wrap(Box::new(|| {
            console_logger::log!("Blue marker image loaded successfully");
        }) as Box<dyn Fn()>);
        blue_marker.set_onload(Some(blue_onload.as_ref().unchecked_ref()));
        blue_onload.forget();
    
        let blue_onerror = Closure::wrap(Box::new(|| {
            console_logger::log!("Failed to load blue marker image");
        }) as Box<dyn Fn()>);
        blue_marker.set_onerror(Some(blue_onerror.as_ref().unchecked_ref()));
        blue_onerror.forget();
    
        let selected_item = ctx.props().selected_item.as_ref();
        console_logger::log!("Number of markers: {}", self.markers.len());
        for (i, (tile_inventory, marker)) in self.markers.iter().enumerate() {
            let pos = marker.get_lat_lng();
            console_logger::log!("Marker ", i, ", lat: ", pos.lat(),", lng: ", pos.lng());
            let is_selected = selected_item.map_or(false, |selected| selected.id == tile_inventory.id);
            let icon_options = IconOptions::new();
            let icon_url = if is_selected {
                "static/markers/marker-icon-2x-red.png"
            } else {
                "static/markers/marker-icon-2x-blue.png"
            };
            icon_options.set_icon_url(icon_url.to_string());
            icon_options.set_icon_size(Point::new(50.0, 82.0));
            icon_options.set_icon_anchor(Point::new(25.0, 82.0));
            icon_options.set_popup_anchor(Point::new(0.0, -82.0));
            let icon = Icon::new(&icon_options);
            marker.set_icon(&icon);
    
            if is_selected {
                console_logger::log!("Selected marker: ", tile_inventory.street_sign.clone(), " with icon: ", icon_url);
            }
        }
        console_logger::log!("Markers updated");
    }
}