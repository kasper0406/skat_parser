#![recursion_limit="256"]

extern crate yew;
extern crate wasm_bindgen;
extern crate console_error_panic_hook;
extern crate web_sys as ws;

use wasm_bindgen::prelude::*;
use console_error_panic_hook::set_once as set_panic_hook;

use web_sys::console;

use yew::prelude::*;
use yew_router::prelude::*;

mod routes;
mod file_drop_component;
mod welcome;
mod record_parser;
mod record_component;

use routes::EindkomstRoutes;

use record_parser::RecordSpec;

use std::rc::Rc;

#[derive(Clone, Properties)]
pub struct RouterProps {
    recordspec: RecordSpec,
}

pub struct EindkomstRouter {
    props: RouterProps,
}

impl Component for EindkomstRouter {
    type Message = ();
    type Properties = RouterProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        EindkomstRouter {
            props
        }
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html!{ <welcome::Welcome recordspec={Rc::new(self.props.recordspec.clone())} /> }
    }
}

#[wasm_bindgen]
pub fn run(record_spec_yml: &str) {
    set_panic_hook();

    let recordspec: RecordSpec = serde_yaml::from_str(record_spec_yml).unwrap();
    yew::start_app_with_props::<EindkomstRouter>(RouterProps { recordspec });
}
