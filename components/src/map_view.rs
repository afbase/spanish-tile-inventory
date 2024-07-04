use data::inventory::TileInventory;
use leaflet::{
    Icon, IconOptions, LatLng, Map, MapOptions, Marker, Point, Popup, PopupOptions,
    TileLayer,
};
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{Element, HtmlElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub inventory: Vec<TileInventory>,
    pub selected_item: Option<TileInventory>,
    pub on_item_select: Callback<Option<TileInventory>>,
}

pub struct MapView {
    map_ref: NodeRef,
    map: Option<Map>,
    markers: Vec<Marker>,
}

impl Component for MapView {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            map_ref: NodeRef::default(),
            map: None,
            markers: vec![],
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div ref={self.map_ref.clone()} style="height: 400px;"></div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let map_element = self.map_ref.cast::<Element>().unwrap();
            self.init_map(map_element);
            self.add_markers(ctx);
        }
        self.update_markers(ctx);
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

        self.map = Some(map);
    }

    fn add_markers(&mut self, ctx: &Context<Self>) {
        if let Some(map) = &self.map {
            for item in &ctx.props().inventory {
                let lat_lng = LatLng::new(item.latitude, item.longitude);
                let marker = Marker::new(&lat_lng);

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
        for (index, item) in ctx.props().inventory.iter().enumerate() {
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

            if let Some(marker) = self.markers.get(index) {
                marker.set_icon(&icon);
            }
        }
    }
}
