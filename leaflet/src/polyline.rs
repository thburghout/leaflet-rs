use crate::{LatLng, Map};
use serde::Serialize;
use wasm_bindgen::JsValue;
use leaflet_sys as sys;

#[derive(Default, Serialize)]
pub struct PolylineOptions {
    pub color: Option<String>,
}

pub struct Polyline {
    polyline: sys::Polyline,
}

impl Polyline {
    pub fn new(latlngs: Vec<LatLng>) -> Polyline {
        Polyline::new_with_options(latlngs, &PolylineOptions::default())
    }

    pub fn new_with_options(latlngs: Vec<LatLng>, options: &PolylineOptions) -> Polyline {
        Polyline {
            polyline: sys::Polyline::new_with_options(
                latlngs
                    .into_iter()
                    .map(|x| -> sys::LatLng { x.into() })
                    .map(JsValue::from)
                    .collect(),
                &JsValue::from_serde(options).expect("error serializing polyline options"),
            ),
        }
    }

    pub fn add_to(&self, map: &Map) {
        self.polyline.addTo(map.inner());
    }
}
