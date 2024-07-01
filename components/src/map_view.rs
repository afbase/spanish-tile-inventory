use yew::prelude::*;
use web_sys::{Element, HtmlElement};
use wasm_bindgen::{JsCast, JsValue};
use js_sys::{Object, Reflect, Array};
use data::inventory::TileInventory;

/// Represents the state and logic for the Map View component
pub struct MapView {
    /// Reference to the DOM element where the map will be rendered
    map_ref: NodeRef,
    /// JavaScript value representing the Leaflet map object
    map: Option<JsValue>,
    /// Vector of JavaScript values representing Leaflet marker objects
    markers: Vec<JsValue>,
}

/// Properties for the MapView component
#[derive(Properties, PartialEq)]
pub struct Props {
    /// The complete inventory of tile items
    pub inventory: Vec<TileInventory>,
    /// The currently selected inventory item, if any
    pub selected_item: Option<TileInventory>,
}

impl Component for MapView {
    type Message = ();
    type Properties = Props;

    /// Creates a new instance of the MapView component
    /// Input: 
    ///   - _ctx: &Context<Self> - The component's context (unused in this method)
    /// Output: Self - A new instance of MapView
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            map_ref: NodeRef::default(),
            map: None,
            markers: vec![],
        }
    }

    /// Renders the component's view
    /// Input: 
    ///   - _ctx: &Context<Self> - The component's context (unused in this method)
    /// Output: Html - The rendered view
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div ref={self.map_ref.clone()} style="height: 400px;"></div>
        }
    }

    /// Handles post-render logic, including map initialization and marker updates
    /// Inputs:
    ///   - ctx: &Context<Self> - The component's context
    ///   - first_render: bool - Whether this is the first render of the component
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            // Initialize the map on first render
            let map_element = self.map_ref.cast::<Element>().unwrap();
            self.init_map(map_element);
            self.add_markers(ctx);
        }
        // Update markers on every render
        self.update_markers(ctx);
    }
}

impl MapView {
    /// Initializes the Leaflet map
    /// Input:
    ///   - element: Element - The DOM element where the map will be rendered
    fn init_map(&mut self, element: Element) {
        let window = web_sys::window().unwrap();
        let leaflet = Reflect::get(&window, &JsValue::from_str("L")).unwrap();
        
        // Create the map object
        let map = Reflect::get(&leaflet, &JsValue::from_str("map")).unwrap();
        let map = js_sys::Function::new_with_args("element", "return new this(element)")
            .call2(&map, &JsValue::from(element), &Object::new())
            .unwrap();

        // Set the map view
        let set_view = Reflect::get(&map, &JsValue::from_str("setView")).unwrap();
        let lat_lng = Array::of2(&JsValue::from_f64(29.9511), &JsValue::from_f64(-90.0715));
        js_sys::Function::new_with_args("latlng, zoom", "this.setView(latlng, zoom)")
            .call3(&set_view, &map, &lat_lng, &JsValue::from_f64(13.0))
            .unwrap();

        // Add the tile layer
        let tile_layer = Reflect::get(&leaflet, &JsValue::from_str("tileLayer")).unwrap();
        let tile_layer = js_sys::Function::new_with_args(
            "url, options",
            "return new this(url, options)",
        )
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

    /// Adds markers to the map for each inventory item
    /// Input:
    ///   - ctx: &Context<Self> - The component's context
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

            self.markers.push(marker);
        }
    }

    /// Updates the markers on the map, highlighting the selected item
    /// Input:
    ///   - ctx: &Context<Self> - The component's context
    fn update_markers(&self, ctx: &Context<Self>) {
        let window = web_sys::window().unwrap();
        let leaflet = Reflect::get(&window, &JsValue::from_str("L")).unwrap();
        let icon_fn = Reflect::get(&leaflet, &JsValue::from_str("icon")).unwrap();

        for (index, item) in ctx.props().inventory.iter().enumerate() {
            let is_selected = ctx.props().selected_item.as_ref().map_or(false, |selected| selected.id == item.id);
            let icon_url = if is_selected {
                "/static/markers/marker-icon-2x-red.png"
            } else {
                "/static/markers/marker-icon-2x-blue.png"
            };

            let icon_options = Object::new();
            Reflect::set(&icon_options, &JsValue::from_str("iconUrl"), &JsValue::from_str(icon_url)).unwrap();
            Reflect::set(&icon_options, &JsValue::from_str("iconSize"), &Array::of2(&JsValue::from_f64(25.0), &JsValue::from_f64(41.0))).unwrap();

            let icon = js_sys::Function::new_with_args("options", "return new this(options)")
                .call1(&icon_fn, &icon_options)
                .unwrap();

            let set_icon = Reflect::get(&self.markers[index], &JsValue::from_str("setIcon")).unwrap();
            js_sys::Function::new_with_args("icon", "this.setIcon(icon)")
                .call1(&set_icon, &icon)
                .unwrap();
        }
    }
}