use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::console;

use crate::record_parser::*;
use crate::record_list_component::*;
use crate::field_list_component::*;

use std::rc::Rc;
use std::collections::HashMap;

#[derive(Clone, Properties)]
pub struct RecordComponentProps {
    pub recordspec: Rc<RecordSpec>,
    pub record: Rc<RecordHierarchy>,
    pub errors: Option<Rc<HashMap<usize, Error>>>,
}

pub struct RecordComponent {
    link: ComponentLink<Self>,
    props: RecordComponentProps,

    expand: bool,
}

pub enum Msg {
    ToggleExpand,
}

pub fn css_class_from_error(error: &Error) -> &str {
    match error.severity {
        ErrorType::Error => "error",
        ErrorType::Warning => "warning",
    }
}

impl Component for RecordComponent {
    type Message = Msg;
    type Properties = RecordComponentProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        RecordComponent {
            link,
            props,
            expand: false,
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            ToggleExpand => {
                self.expand = !self.expand;
            }
        }
        true
    }


    fn view(&self) -> Html {
        let record = &self.props.record;
        let expand_children = self.expand || record.contains_error(&self.props.errors);
        let expandable = !record.children().is_empty();

        // Find a potential error for the record
        // TODO(knielsen): Currently the linenumber record field number hardcoded
        let maybe_raw_linenr = record.record().field(1);
        let error = maybe_raw_linenr.map(|raw_linenr| {
            self.props.errors.as_ref().map(|errors| {
                let linenr = raw_linenr.parse::<usize>().unwrap();
                errors.get(&linenr)
            }).flatten()
        }).flatten();

        let mut classes = vec!["record"];
        let mut error_html = html!{};
        if let Some(err) = error {
            classes.push(css_class_from_error(&err));
            error_html = html! {
                <div class="msg">
                    <b>{ err.error_id.clone() }</b>{ ":" } { err.error_text.clone() }
                </div>
            };
        }

        html! {
            <div class=classes>
                { if let Some(spec) = record.record().spec() {
                    html! { <div class="title"> { spec.name() } </div> }
                } else {
                    html! { <div class="title unknown"> { "Unknown record" } </div> }
                } }

                { error_html }

                <FieldListComponent fields=record.record().fields().to_vec()
                                    error=error.map(|err| err.clone()) />
                
                { if expand_children {
                    html! {
                        <RecordListComponent recordspec={self.props.recordspec.clone()}
                                             records={record.children()}
                                             errors={self.props.errors.clone()} />
                    }
                } else if expandable {
                    html! {
                        <div class="expandable" onclick=self.link.callback(|evnet| Msg::ToggleExpand)>
                            { "Click to expand" }
                        </div> }
                } else {
                    html! {}
                } }
            </div>
        }
    }
}
