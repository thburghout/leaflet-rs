use leaflet_sys as sys;
use wasm_bindgen::JsValue;
use crate::Map;

#[derive(Clone, PartialEq)]
pub struct TileLayer {
    url_template: String,
    options: serde_json::Value,
}


impl TileLayer {
    pub fn new_with_options(url_template: String, options: serde_json::Value) -> TileLayer {
        TileLayer { url_template, options }
    }

    pub fn add_to(&self, map: &Map) {
        sys::TileLayer::new(&self.url_template, &JsValue::from_serde(&self.options).unwrap())
            .addTo(map.inner());
    }
}
