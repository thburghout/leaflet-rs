use yew::prelude::*;
use leaflet::yew::*;

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
        html!(
            <Map center=LatLng::new(51.505, -0.09) zoom=13.0 >
                <Marker lat_lng=LatLng::new(51.5, -0.09)/>
            </Map>
        )
    }
}



fn main() {
    yew::start_app::<App>();
}