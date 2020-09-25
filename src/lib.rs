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
mod app;
mod welcome;
mod record_parser;

use routes::EindkomstRoutes;

use record_parser::RecordSpec;

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
        /*
        html! {
            <div style="-webkit-user-select: none; cursor: default;">
                <Router<EindkomstRoutes>
                    render = Router::render(move |switch: EindkomstRoutes| {
                        match switch {
                            EindkomstRoutes::Welcome => html!{ <welcome::Welcome recordspec={recordspec} /> },
                            EindkomstRoutes::Eindkomst => html!{ <app::App /> }
                        }
                    })
                    redirect = Router::redirect(|route: Route| {
                        EindkomstRoutes::Welcome
                    })
                />
            </div>
        }
        */

        html!{ <welcome::Welcome recordspec={self.props.recordspec.clone()} /> }
    }
}

#[wasm_bindgen]
pub fn run(record_spec_yml: &str) {
    set_panic_hook();

    let recordspec: RecordSpec = serde_yaml::from_str(record_spec_yml).unwrap();
    yew::start_app_with_props::<EindkomstRouter>(RouterProps { recordspec });
}
