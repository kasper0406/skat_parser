use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::console;

use crate::record_parser::*;
use crate::record_component::css_class_from_error;

use std::rc::Rc;
use std::collections::HashMap;

#[derive(Clone, Properties)]
pub struct FieldComponentProps {
    pub field: ParsedField,
    pub error: Option<Error>,
}

pub struct FieldComponent {
    link: ComponentLink<Self>,
    props: FieldComponentProps,

    show_raw: bool,
}

pub enum Msg {
    ToggleRaw,
}

impl Component for FieldComponent {
    type Message = Msg;
    type Properties = FieldComponentProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        FieldComponent {
            link,
            props,
            show_raw: false,
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            ToggleRaw => self.show_raw = !self.show_raw,
        }
        true
    }


    fn view(&self) -> Html {
        // TODO(knielsen): Have a checkbox to determine if unknown fields should be shown or not
        let field = &self.props.field;
        if field.spec().is_some() {
            let spec = field.spec().as_ref();

            let mut classes = vec!["field"];
            if let Some(spec) = spec {
                if let Some(err) = &self.props.error {
                    if spec.field_number() == err.field {
                        classes.push(css_class_from_error(&err));
                    }
                }
            }

            html! {
                <div class=classes>
                    <p class="name">
                        { spec.map(|spec| spec.name().clone()).unwrap_or("Unknown".to_string()) }
                    </p>
                    <p class="value" ondblclick=self.link.callback(|event| Msg::ToggleRaw)>
                        { if !self.show_raw {
                            spec
                                .map(|spec| spec.formatter().format(field.raw().clone()).unwrap_or(html! { field.raw() }))
                                .unwrap_or(html! { <p class="raw"> { field.raw() } </p> })
                          } else {
                              html! { <p class="raw"> { field.raw() } </p> }
                          }
                        }
                    </p>
                </div>
            }
        } else {
            html! {}
        }
    }
}
