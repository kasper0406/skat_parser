#![recursion_limit="512"]

extern crate yew;
extern crate wasm_bindgen;
extern crate console_error_panic_hook;
extern crate web_sys as ws;

use wasm_bindgen::prelude::*;
use console_error_panic_hook::set_once as set_panic_hook;

use web_sys::console;

use yew::prelude::*;
use yew_router::prelude::*;

mod file_drop_component;
mod welcome;
mod record_parser;
mod record_component;
mod record_list_component;
mod field_component;
mod field_list_component;

use record_parser::RecordSpec;

use std::rc::Rc;

#[derive(Clone, Properties)]
pub struct RouterProps {
    recordspec: RecordSpec,
    errorspec: RecordSpec,
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
        html!{ <welcome::Welcome
                    recordspec={Rc::new(self.props.recordspec.clone())}
                    errorspec={Rc::new(self.props.errorspec.clone())} /> }
    }
}

#[wasm_bindgen]
pub fn run(record_spec_yml: &str, error_spec_yml: &str) {
    set_panic_hook();

    let recordspec: RecordSpec = serde_yaml::from_str(record_spec_yml).unwrap();
    let errorspec: RecordSpec = serde_yaml::from_str(error_spec_yml).unwrap();
    yew::start_app_with_props::<EindkomstRouter>(RouterProps { recordspec, errorspec });
}
