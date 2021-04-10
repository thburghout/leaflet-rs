use super::{LatLng, Map};
use serde::Serialize;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;
use leaflet_sys as sys;

pub struct Marker {
    marker: sys::Marker,
    on_drag: Option<Closure<dyn FnMut(sys::Event)>>,
}

#[derive(Default, Serialize)]
pub struct MarkerOptions {
    pub draggable: Option<bool>,
}

impl Marker {
    pub fn new(latlng: LatLng) -> Marker {
        Marker::new_with_options(latlng, &MarkerOptions::default())
    }

    pub fn new_with_options(latlng: LatLng, options: &MarkerOptions) -> Marker {
        Marker {
            marker: sys::Marker::new(
                &latlng.into(),
                &JsValue::from_serde(options).expect("error serializing marker options"),
            ),
            on_drag: None,
        }
    }

    pub fn on_drag(&mut self, listener: Box<dyn FnMut(sys::Event)>) {
        self.on_drag = Some(Closure::wrap(listener));
        self.marker.on("drag", self.on_drag.as_ref().unwrap())
    }

    pub fn add_to(&self, map: &Map) {
        self.marker.addTo(map.inner());
    }
}

impl Drop for Marker {
    fn drop(&mut self) {
        if self.on_drag.is_some() {
            self.marker.off("drag");
            self.marker.remove();
        }
    }
}

impl MarkerOptions {
    pub fn draggable(&mut self, draggable: bool) -> &mut Self {
        self.draggable = Some(draggable);
        self
    }
}
