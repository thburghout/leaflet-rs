use yew::prelude::*;
use yew::virtual_dom::{VChild, VNode};
use crate::LatLng;

pub type ElemId = isize; // maybe String?

#[derive(Clone, PartialEq, derive_more::From)]
pub enum MapElements {
    Marker(VChild<Marker>),
    Polyline(VChild<Polyline>),
}

pub struct Marker {
    pub(crate) props: MarkerProps,
    link: ComponentLink<Self>,
}

pub struct Polyline {
    props: PolylineProps,
    link: ComponentLink<Self>,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct MarkerProps {
    #[prop_or_default]
    pub id: Option<ElemId>,

    pub lat_lng: LatLng,

    #[prop_or_default]
    pub draggable: Option<bool>,

    #[prop_or_default]
    pub on_drag: Option<Callback<LatLng>>,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct PolylineProps {
    #[prop_or_default]
    pub id: Option<ElemId>,

    pub path: Vec<LatLng>,
    pub color: String,
}

#[allow(clippy::from_over_into)]
impl Into<crate::Marker> for MarkerProps {
    fn into(self) -> crate::Marker {
        let options = crate::MarkerOptions {
            draggable: self.draggable,
        };
        let mut m = crate::Marker::new_with_options(self.lat_lng, &options);
        if let Some(callback) = self.on_drag {
            m.on_drag(Box::new(move |e| callback.emit(e.latlng().into())));
        }
        m
    }
}

#[allow(clippy::from_over_into)]
impl Into<crate::Polyline> for PolylineProps {
    fn into(self) -> crate::Polyline {
        let options = crate::PolylineOptions {
            color: Some(self.color),
        };
        crate::Polyline::new_with_options(self.path, &options)
    }
}

#[allow(clippy::from_over_into)]
impl Into<VNode> for MapElements {
    fn into(self) -> VNode {
        match self {
            MapElements::Marker(c) => c.into(),
            MapElements::Polyline(c) => c.into(),
        }
    }
}

impl Component for Marker {
    type Message = ();
    type Properties = MarkerProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html!()
    }
}

impl Component for Polyline {
    type Message = ();
    type Properties = PolylineProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, _: Self::Message) -> bool {
        true
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html!()
    }
}
