use data::inventory::TileInventory;
use leaflet::{
    Icon, IconOptions, LatLng, Map, MapOptions, Marker, Point, Popup, PopupOptions, TileLayer,
};
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{Element, HtmlElement};
use yew::prelude::*;

pub struct MapView {
    map_ref: NodeRef,
    map: Option<Map>,
    markers: Vec<Marker>,
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
        Self {
            map_ref: NodeRef::default(),
            map: None,
            markers: Vec::new(),
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

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div ref={self.map_ref.clone()} style="width:100%;height:400px;"></div>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        ctx.link().send_message(Msg::UpdateMarkers);
        false
    }
}

impl MapView {
    fn init_map(&mut self, ctx: &Context<Self>, element: Element) {
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
    }

    fn create_markers(&mut self, ctx: &Context<Self>, map: &Map) {
        let default = 0.0;
        for item in &ctx.props().inventory {
            let (lat, long) = (
                item.latitude.unwrap_or(default),
                item.longitude.unwrap_or(default),
            );
            let marker = Marker::new(&LatLng::new(lat, long));
            let popup_content = format!(
                "{}: {:?} damaged tiles",
                item.street_sign, item.number_of_tiles_damaged
            );
            let popup_options = PopupOptions::new();
            let popup = Popup::new(&popup_options, None);
            popup.set_content(&JsValue::from_str(&popup_content));
            marker.bind_popup(&popup);

            let item_clone = item.clone();
            let on_item_select = ctx.props().on_item_select.clone();
            let closure = Closure::wrap(Box::new(move || {
                on_item_select.emit(Some(item_clone.clone()));
            }) as Box<dyn Fn()>);

            marker.on("click", closure.as_ref().unchecked_ref());
            closure.forget();

            marker.add_to(map);
            self.markers.push(marker);
        }
    }

    fn update_markers(&self, ctx: &Context<Self>) {
        let selected_item = ctx.props().selected_item.as_ref();
        for (item, marker) in ctx.props().inventory.iter().zip(self.markers.iter()) {
            let is_selected = selected_item.map_or(false, |selected| selected.id == item.id);
            let icon_options = IconOptions::new();
            icon_options.set_icon_url(if is_selected {
                "/static/markers/marker-icon-2x-red.png".to_string()
            } else {
                "/static/markers/marker-icon-2x-blue.png".to_string()
            });
            icon_options.set_icon_size(Point::new(25.0, 41.0));
            let icon = Icon::new(&icon_options);
            marker.set_icon(&icon);
        }
    }
}
