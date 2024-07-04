use js_sys::Promise;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    pub fn initGeoSearch(map: &JsValue);

    #[wasm_bindgen(js_namespace = window)]
    pub fn geocodeAddress(address: &str) -> Promise;
}
