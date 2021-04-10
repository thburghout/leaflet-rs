use leaflet::{Map, LatLng, Marker};
use leaflet::sys::{TileLayer, Circle};
use wasm_bindgen::JsValue;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

fn main() {
    log!("Hello World!");

    let mut map = Map::new("mymap");
    map.set_view((51.505, -0.09).into(), 13.0);

    let tiles = TileLayer::new(
        "https://api.mapbox.com/styles/v1/{id}/tiles/{z}/{x}/{y}?access_token={accessToken}",
        &JsValue::from_serde(&serde_json::json!({
            "attribution": "Map data &copy; <a href='https://www.openstreetmap.org/copyright'>OpenStreetMap</a> contributors, Imagery © <a href='https://www.mapbox.com/'>Mapbox</a>",
            "maxZoom": 18,
            "id": "mapbox/streets-v11",
            "tileSize": 512,
            "zoomOffset": -1,
            "accessToken": env!("MAPBOX_TOKEN"),
        })).unwrap(),
    );
    tiles.addTo(map.inner());


    let marker = Marker::new((51.5, -0.09).into());
    marker.add_to(&map);

    let circle = Circle::new_with_options(
        &LatLng::new(51.508, -0.11).into(),
        &JsValue::from_serde(&serde_json::json!({
            "color": "red",
            "fillColor": "#f03",
            "fillOpacity": 0.5,
            "radius": 500
        })).unwrap(),
    );
    circle.addTo(map.inner());

}