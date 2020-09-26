use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::console;

use crate::record_parser::*;

use std::rc::Rc;

#[derive(Clone, Properties)]
pub struct RecordComponentProps {
    pub recordspec: Rc<RecordSpec>,
    pub records: Rc<Vec<RecordHierarchy>>,
}

pub struct RecordComponent {
    link: ComponentLink<Self>,
    props: RecordComponentProps,
}

pub enum Msg {

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

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let renderField = |field: &ParsedField| {
            // TODO(knielsen): Have a checkbox to determine if unknown fields should be shown or not
            if field.spec().is_some() {
                html! {
                    <div class="field">
                        <p class="name">
                            { field.spec().as_ref().map(|spec| spec.name().clone()).unwrap_or("Unknown".to_string()) }
                        </p>
                        <p class="value">
                            { field.spec().as_ref()
                                .map(|spec| spec.formatter().format(field.raw().clone()).unwrap_or(html! { field.raw() }))
                                .unwrap_or(html! { field.raw() }) }
                        </p>
                    </div>
                }
            } else {
                html! {}
            }
        };

        let renderRecord = |record: &RecordHierarchy| { html! {
            <div class="record">
                { if let Some(spec) = record.record().spec() {
                    html! { <div class="title"> { spec.name() } </div> }
                } else {
                    html! { }
                } }
               
                <div class="fields">
                    { for record.record().fields().iter().map(renderField) }
                </div>
                
                <RecordComponent recordspec={self.props.recordspec.clone()}
                                             records={record.children()} />
            </div>
        } };

        html! { for self.props.records.iter().map(renderRecord) }
    }
}
