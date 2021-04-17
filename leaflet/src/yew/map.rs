use super::elements::{ElemId, MapElements, MarkerProps, PolylineProps};
use wasm_bindgen::JsValue;
use yew::{html::ChildrenRenderer, prelude::*};
use leaflet_sys as sys;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use crate::LatLng;
use crate::tile_layer::TileLayer;

pub struct Map {
    props: MapProps,
    link: ComponentLink<Self>,
    id: String,
    map: Option<crate::Map>,
    markers: HashMap<ElemId, crate::Marker>,
    update_markers: Vec<(ElemId, MarkerProps)>,
    polylines: HashMap<ElemId, crate::Polyline>,
    update_polylines: Vec<(ElemId, PolylineProps)>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct MapProps {
    pub children: ChildrenRenderer<MapElements>,

    pub tile_layer: TileLayer,

    pub center: LatLng,

    pub zoom: f32,
}

impl Component for Map {
    type Message = ();
    type Properties = MapProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let id = thread_rng()
            .sample_iter(Alphanumeric)
            .take(10)
            .map(char::from)
            .collect();
        Self {
            props,
            link,
            id,
            map: None,
            markers: HashMap::new(),
            update_markers: vec![],
            polylines: HashMap::new(),
            update_polylines: vec![],
        }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        let polylines = self.props.polylines();
        let changed_polylines: Vec<(ElemId, PolylineProps)> = props
            .polylines()
            .into_iter()
            .filter(changed_elements(&polylines))
            .collect();
        let do_render = !changed_polylines.is_empty();
        for (id, polyline) in changed_polylines {
            self.update_polylines.push((id, polyline.clone()));
        }
        self.props = props;
        do_render
    }

    fn view(&self) -> Html {
        html! {
            <div id={&self.id} style="width: 100%; height: 400px;">
            </div>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.init();
        }
        let map = self.map.as_ref().unwrap();
        for (id, marker) in self.update_markers.drain(..) {
            let m: crate::Marker = marker.into();
            m.add_to(map);
            self.markers.insert(id, m);
        }
        for (id, polyline) in self.update_polylines.drain(..) {
            let p: crate::Polyline = polyline.into();
            p.add_to(map);
            self.polylines.insert(id, p);
        }
    }
}

impl Map {
    fn init(&mut self) {
        let mut map = crate::Map::new(&self.id);
        map.set_view(self.props.center.into(), self.props.zoom.into());
        self.props.tile_layer.add_to(&map);
        self.map = Some(map);
        for (id, marker) in self.props.markers() {
            self.update_markers.push((id, marker))
        }
        for (id, polyline) in self.props.polylines() {
            self.update_polylines.push((id, polyline))
        }
    }
}

impl MapProps {
    pub fn markers(&self) -> Vec<(ElemId, MarkerProps)> {
        let mut id: ElemId = -1;
        self.children
            .iter()
            .filter_map(|x| match x {
                MapElements::Marker(m) => {
                    id += 1;
                    Some((m.props.id.unwrap_or(id), m.props))
                }
                _ => None,
            })
            .collect()
    }

    pub fn polylines(&self) -> Vec<(ElemId, PolylineProps)> {
        let mut id: ElemId = -1;
        self.children
            .iter()
            .filter_map(|x| match x {
                MapElements::Polyline(m) => {
                    id += 1;
                    Some((m.props.id.unwrap_or(id), m.props))
                }
                _ => None,
            })
            .collect()
    }
}

fn changed_elements<'a, Id, Elem>(
    current: &'a [(Id, Elem)],
) -> Box<dyn Fn(&(Id, Elem)) -> bool + 'a>
where
    Elem: PartialEq,
    Id: PartialEq,
{
    Box::new(move |(id, element)| {
        let candidate = current.iter().find(|(other_id, _)| other_id == id);
        if let Some((_, candidate)) = candidate {
            candidate != element
        } else {
            false
        }
    })
}
