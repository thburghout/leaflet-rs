use core::fmt;
use serde::{Deserialize, Serialize};
use leaflet_sys as sys;

#[derive(Copy, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LatLng {
    pub lat: f64,
    pub lng: f64,
}

impl LatLng {
    pub fn new(lat: f64, lng: f64) -> LatLng {
        LatLng { lat, lng }
    }

    pub fn new_from_pair(latlng: (f64, f64)) -> LatLng {
        let (lat, lng) = latlng;
        LatLng::new(lat, lng)
    }
}

#[allow(clippy::from_over_into)]
impl Into<sys::LatLng> for LatLng {
    fn into(self) -> sys::LatLng {
        sys::LatLng::new(self.lat, self.lng)
    }
}

impl From<sys::LatLng> for LatLng {
    fn from(val: sys::LatLng) -> Self {
        LatLng::new(val.lat(), val.lng())
    }
}

impl From<(f64, f64)> for LatLng {
    fn from(latlng: (f64, f64)) -> Self {
        LatLng::new_from_pair(latlng)
    }
}

impl fmt::Display for LatLng {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        if let Some(precision) = formatter.precision() {
            write!(
                formatter,
                "{:.*}, {:.*}",
                precision, self.lat, precision, self.lng
            )
        } else {
            write!(formatter, "{}, {}", self.lat, self.lng)
        }
    }
}
