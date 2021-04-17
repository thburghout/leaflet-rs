use yew::prelude::*;
use leaflet::yew::*;
use leaflet::TileLayer;

struct App {
    link: ComponentLink<Self>
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let layer = TileLayer::new_with_options(
            "https://api.mapbox.com/styles/v1/{id}/tiles/{z}/{x}/{y}?access_token={accessToken}".to_owned(),
            serde_json::json!({
                "attribution": "Map data &copy; <a href='https://www.openstreetmap.org/copyright'>OpenStreetMap</a> contributors, Imagery © <a href='https://www.mapbox.com/'>Mapbox</a>",
                "maxZoom": 18,
                "tileSize": 512,
                "zoomOffset": -1,
                "id": "mapbox/streets-v11",
                "accessToken": env!("MAPBOX_TOKEN"),
            }),
        );
        html!(
            <Map tile_layer=layer center=LatLng::new(51.505, -0.09) zoom=13.0 >
                <Marker lat_lng=LatLng::new(51.5, -0.09)/>
            </Map>
        )
    }
}



fn main() {
    yew::start_app::<App>();
}