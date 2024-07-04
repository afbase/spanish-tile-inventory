use data::inventory::TileInventory;
use js_sys::{Array, Object, Reflect};
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
    map: Option<JsValue>,
    markers: Vec<JsValue>,
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
        let window = web_sys::window().unwrap();
        let leaflet = Reflect::get(&window, &JsValue::from_str("L")).unwrap();

        let map = Reflect::get(&leaflet, &JsValue::from_str("map")).unwrap();
        let map = js_sys::Function::new_with_args("element", "return new this(element)")
            .call2(&map, &JsValue::from(element), &Object::new())
            .unwrap();

        let set_view = Reflect::get(&map, &JsValue::from_str("setView")).unwrap();
        let lat_lng = Array::of2(&JsValue::from_f64(29.9511), &JsValue::from_f64(-90.0715));
        js_sys::Function::new_with_args("latlng, zoom", "this.setView(latlng, zoom)")
            .call3(&set_view, &map, &lat_lng, &JsValue::from_f64(13.0))
            .unwrap();

        let tile_layer = Reflect::get(&leaflet, &JsValue::from_str("tileLayer")).unwrap();
        let tile_layer =
            js_sys::Function::new_with_args("url, options", "return new this(url, options)")
                .call2(
                    &tile_layer,
                    &JsValue::from_str("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"),
                    &Object::new(),
                )
                .unwrap();

        let add_to = Reflect::get(&tile_layer, &JsValue::from_str("addTo")).unwrap();
        js_sys::Function::new_with_args("map", "this.addTo(map)")
            .call1(&add_to, &map)
            .unwrap();

        self.map = Some(map);
    }

    fn add_markers(&mut self, ctx: &Context<Self>) {
        let window = web_sys::window().unwrap();
        let leaflet = Reflect::get(&window, &JsValue::from_str("L")).unwrap();
        let marker_fn = Reflect::get(&leaflet, &JsValue::from_str("marker")).unwrap();

        for item in &ctx.props().inventory {
            let lat_lng = Array::of2(
                &JsValue::from_f64(item.latitude),
                &JsValue::from_f64(item.longitude),
            );
            let marker = js_sys::Function::new_with_args("latlng", "return new this(latlng)")
                .call1(&marker_fn, &lat_lng)
                .unwrap();

            let add_to = Reflect::get(&marker, &JsValue::from_str("addTo")).unwrap();
            js_sys::Function::new_with_args("map", "this.addTo(map)")
                .call1(&add_to, &self.map.as_ref().unwrap())
                .unwrap();

            // Add click event listener
            let on_item_select = ctx.props().on_item_select.clone();
            let item_clone = item.clone();
            let click_closure = Closure::wrap(Box::new(move || {
                on_item_select.emit(Some(item_clone.clone()));
            }) as Box<dyn Fn()>);

            let on = Reflect::get(&marker, &JsValue::from_str("on")).unwrap();
            js_sys::Function::new_with_args("event, handler", "this.on(event, handler)")
                .call2(&on, &JsValue::from_str("click"), click_closure.as_ref())
                .unwrap();

            click_closure.forget(); // Prevent the closure from being dropped

            self.markers.push(marker);
        }
    }

    fn update_markers(&self, ctx: &Context<Self>) {
        let window = web_sys::window().unwrap();
        let leaflet = Reflect::get(&window, &JsValue::from_str("L")).unwrap();
        let icon_fn = Reflect::get(&leaflet, &JsValue::from_str("icon")).unwrap();

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

            let icon_options = Object::new();
            Reflect::set(
                &icon_options,
                &JsValue::from_str("iconUrl"),
                &JsValue::from_str(icon_url),
            )
            .unwrap();
            Reflect::set(
                &icon_options,
                &JsValue::from_str("iconSize"),
                &Array::of2(&JsValue::from_f64(25.0), &JsValue::from_f64(41.0)),
            )
            .unwrap();

            let icon = js_sys::Function::new_with_args("options", "return new this(options)")
                .call1(&icon_fn, &icon_options)
                .unwrap();

            let set_icon =
                Reflect::get(&self.markers[index], &JsValue::from_str("setIcon")).unwrap();
            js_sys::Function::new_with_args("icon", "this.setIcon(icon)")
                .call1(&set_icon, &icon)
                .unwrap();
        }
    }
}
