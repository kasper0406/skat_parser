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

use crate::record_parser::*;

use crate::routes::EindkomstRoutes;

#[derive(Clone, Properties)]
pub struct WelcomeProps {
    pub recordspec: RecordSpec,
}

pub struct Welcome {
    link: ComponentLink<Self>,
    props: WelcomeProps,
    
    records: Option<Vec<ParsedRecord>>,
    // Error with file load to be displayed to the user
    error: Option<String>,
}

pub enum Msg {
    DragOver(DragEvent),
    FileDropped(DragEvent),
    FileLoaded(Vec<u8>),

    RecordsParsed(Vec<ParsedRecord>),
}

async fn load_bytes_from_stream(link: ComponentLink<Welcome>, js_stream: ws::ReadableStream) {
    let mut stream = wasm_streams::ReadableStream::from_raw(js_stream.dyn_into().unwrap()).into_stream();

    let mut buffer = vec![];
    stream.for_each(|item| {
        let array = js_sys::Uint8Array::new(&item.unwrap());
        buffer.append(&mut array.to_vec());
        
        future::ready(())
    }).await;

    link.send_message(Msg::FileLoaded(buffer));
}

async fn async_parse_records(bytes: Vec<u8>, spec: RecordSpec, link: ComponentLink<Welcome>) {
    let content = std::str::from_utf8(&bytes).unwrap();

    let records = parse_records(&content, spec);
    link.send_message(Msg::RecordsParsed(records))
}

impl Component for Welcome {
    type Message = Msg;
    type Properties = WelcomeProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Welcome {
            link,
            props,
            error: None,
            records: None,
        }
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DragOver(event) => {
                event.data_transfer().unwrap().set_drop_effect("copy");
                event.prevent_default();
            },
            Msg::FileDropped(event) => {
                console::log_1(&"File dropped".into());
                if let Some(files) = event.data_transfer().unwrap().files() {
                    console::log_1(&format!("{} files dropped", files.length()).into());
                    if files.length() != 1 {
                        self.error = Some("Exactly one file should be processed!".into());
                    }

                    let file = files.get(0).unwrap();
                    console::log_1(&format!("Received file {}", file.name()).into());

                    let js_stream = file.stream();
                    
                    spawn_local(load_bytes_from_stream(self.link.clone(), js_stream));
                }

                event.prevent_default();
            },
            Msg::FileLoaded(bytes) => {
                spawn_local(async_parse_records(bytes, self.props.recordspec.clone(), self.link.clone()));
            },
            Msg::RecordsParsed(records) => {
                self.records = Some(records);
            },
        }

        true
    }

    fn view(&self) -> Html {
        let renderField = |field: &ParsedField| { html! {
            <div class="field">
                <p> { field.spec().as_ref().map(|spec| spec.name().clone()).unwrap_or("Unknown".to_string()) } </p>
                { field.spec().as_ref().map(|spec|
                    match spec.formatter().format(field.raw().clone()) {
                        Ok(parsed) => parsed,
                        _ => "PARSE_ERROR".to_string()
                    })
                    .unwrap_or(field.raw().clone()) }
            </div>
        } };

        let renderRecord = |record: &ParsedRecord| { html! {
            <div class="record">
                { for record.fields().iter().map(renderField) }
            </div>
        } };

        let recordsView = if let Some(records) = &self.records {
            html! { for records.iter().map(renderRecord) }
        } else {
            html!{ { "Upload a file to show records" } }
        };

        html! {
            <>
                <h1>{ "eIndkomst viewer"}</h1>
                {
                    if let Some(error) = &self.error {
                        html! { <p> { error } </p> }
                    } else {
                        html! { }
                    }
                }
                <p ondragover=self.link.callback(|event| Msg::DragOver(event))
                   ondrop=self.link.callback(|event| Msg::FileDropped(event))>
                   { "Drop your main file here" }
                </p>

                <div>
                    { recordsView }
                </div>
            </>
        }
    }
}
