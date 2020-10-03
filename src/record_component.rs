use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::console;

use crate::record_parser::*;

use std::rc::Rc;
use std::collections::HashMap;

#[derive(Clone, Properties)]
pub struct RecordComponentProps {
    pub recordspec: Rc<RecordSpec>,
    pub records: Rc<Vec<RecordHierarchy>>,
    pub errors: Option<Rc<HashMap<usize, Error>>>,
}

pub struct RecordComponent {
    link: ComponentLink<Self>,
    props: RecordComponentProps,
}

pub enum Msg {

}

fn css_class_from_error(error: &Error) -> &str {
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
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }


    fn view(&self) -> Html {
        let render_field = |field: &ParsedField, error: Option<&Error>| {
            // TODO(knielsen): Have a checkbox to determine if unknown fields should be shown or not
            if field.spec().is_some() {
                let spec = field.spec().as_ref();

                let mut classes = vec!["field"];
                if let Some(spec) = spec {
                    if let Some(err) = error {
                        if spec.field_number() == err.field {
                            classes.push(css_class_from_error(err));
                        }
                    }
                }

                html! {
                    <div class=classes>
                        <p class="name">
                            { spec.map(|spec| spec.name().clone()).unwrap_or("Unknown".to_string()) }
                        </p>
                        <p class="value">
                            { spec
                                .map(|spec| spec.formatter().format(field.raw().clone()).unwrap_or(html! { field.raw() }))
                                .unwrap_or(html! { field.raw() }) }
                        </p>
                    </div>
                }
            } else {
                html! {}
            }
        };

        let render_record = |record: &RecordHierarchy| {
            if !record.contains_error(&self.props.errors) {
                return html! {};
            }

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
                        html! { }
                    } }

                    { error_html }
                
                    <div class="fields">
                        { for record.record().fields().iter().map(|rec| render_field(rec, error)) }
                    </div>
                    
                    <RecordComponent recordspec={self.props.recordspec.clone()}
                                     records={record.children()}
                                     errors={self.props.errors.clone()} />
                </div>
            }
        };

        html! { for self.props.records.iter().map(render_record) }
    }
}
