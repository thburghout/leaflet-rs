use super::LatLng;
use wasm_bindgen::JsValue;
use leaflet_sys as sys;

pub struct Map {
    map: sys::Map,
}

impl Map {
    pub fn new(id: &str) -> Map {
        Map {
            map: sys::Map::new(id, &JsValue::null()),
        }
    }

    pub fn set_view(&mut self, center: LatLng, zoom: f64) {
        self.map.setView(&center.into(), zoom);
    }

    // todo make pub(crate) again
    pub fn inner(&self) -> &sys::Map {
        &self.map
    }
}
