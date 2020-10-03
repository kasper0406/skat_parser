use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::console;

use futures::future;
use futures::stream::{self, StreamExt};
use futures::stream::Stream;

use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_futures::spawn_local;

use std::str;
use std::rc::Rc;

use std::collections::HashMap;

use encoding::{Encoding, DecoderTrap};
use encoding::all::ISO_8859_1;

use crate::record_parser::*;
use crate::record_component::*;
use crate::file_drop_component::*;

#[derive(Clone, Properties)]
pub struct WelcomeProps {
    pub recordspec: Rc<RecordSpec>,
    pub errorspec: Rc<RecordSpec>,
}

pub struct Welcome {
    link: ComponentLink<Self>,
    props: WelcomeProps,
    
    records: Option<Rc<Vec<RecordHierarchy>>>,
    errors: Option<Rc<HashMap<usize, Error>>>,
}

pub enum Msg {
    RecordFileLoaded(Vec<u8>),
    RecordsParsed(Rc<Vec<RecordHierarchy>>),

    ErrorFileLoaded(Vec<u8>),
    UpdateErrors(Rc<HashMap<usize, Error>>),
}

async fn async_parse_records(bytes: Vec<u8>, spec: Rc<RecordSpec>, link: ComponentLink<Welcome>) {
    let content = ISO_8859_1.decode(&bytes, DecoderTrap::Strict).unwrap();

    let records = parse_records(&content, spec.clone());
    let records_with_hierarchy = build_hierarchy(&records, &spec);

    link.send_message(Msg::RecordsParsed(Rc::new(records_with_hierarchy)))
}

async fn async_process_errors(bytes: Vec<u8>, spec: Rc<RecordSpec>, link: ComponentLink<Welcome>) {
    let content = ISO_8859_1.decode(&bytes, DecoderTrap::Strict).unwrap();
    let records = parse_records(&content, spec.clone());

    link.send_message(Msg::UpdateErrors(Rc::new(extract_errors(&records))))
}

impl Component for Welcome {
    type Message = Msg;
    type Properties = WelcomeProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Welcome {
            link,
            props,
            records: None,
            errors: None,
        }
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RecordFileLoaded(bytes) => {
                spawn_local(async_parse_records(bytes, self.props.recordspec.clone(), self.link.clone()));
            },
            Msg::RecordsParsed(records) => {
                self.records = Some(records);
            },
            Msg::ErrorFileLoaded(bytes) => {
                spawn_local(async_process_errors(bytes, self.props.errorspec.clone(), self.link.clone()));
            },
            Msg::UpdateErrors(errors) => {
                self.errors = Some(errors);
            },
        }

        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                <h1>{ "eIndkomst viser"}</h1>
                
                { if self.records.is_none() {
                    html! {
                        <FileDropComponent
                                onfileloaded={self.link.callback(|content| Msg::RecordFileLoaded(content))}
                                text="Drop eIndkomst filen her" />
                    }
                } else {
                    html! {}
                } }
                
                { if self.errors.is_none() {
                    html! {
                        <FileDropComponent
                                onfileloaded={self.link.callback(|content| Msg::ErrorFileLoaded(content))}
                                text="Drop fejl filen her" />
                    }
                } else {
                    html! {}
                } }

                { if let Some(records) = &self.records {
                    html! {
                    <div>
                        <RecordComponent recordspec={self.props.recordspec.clone()}
                                         records={records.clone()}
                                         errors={self.errors.clone()} />
                    </div>
                    }
                } else {
                    html! {}
                } }
            </>
        }
    }
}
