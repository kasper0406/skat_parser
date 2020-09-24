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

use crate::routes::EindkomstRoutes;

#[derive(Clone, Debug)]
enum FieldFormatter {
    Raw,
    Integer,
    Date,
    CprNumber,
}

impl FieldFormatter {
    fn format(&self, raw_value: String) -> Result<String, ()> {
        match self {
            FieldFormatter::Integer => {
                let parsed = raw_value.parse::<i64>();
                parsed.map(|value| format!("{}", value)).map_err(|_| ())
            },
            _ => Ok(raw_value),
        }
    }
}

#[derive(Clone, Debug)]
struct RecordField {
    field_number: usize,
    start: usize,
    length: usize,
    name: String,
    formatter: FieldFormatter,
}

impl RecordField {
    fn key(start: usize, length: usize) -> Self {
        RecordField {
            start, length,
            field_number: 999,
            name: "".to_string(),
            formatter: FieldFormatter::Raw,
        }
    }
}

impl RecordField {
    fn parse(&self, line: &str) -> ParsedField {
        let raw_value = line[(self.start - 1)..(self.start + self.length - 1)].to_string();
        ParsedField::of(raw_value, self)
    }
}

struct RecordType {
    key: String,
    fields: Vec<RecordField>,
}

struct RecordSpec {
    key_field: RecordField,
    records: Vec<RecordType>,
}

impl RecordSpec {
    fn get_record_type(&self, line: &str) -> Option<&RecordType> {
        let key = self.key_field.parse(line).raw_value;
        self.records.iter().find(|record| record.key == key)
    }
}

#[derive(Debug)]
struct ParsedRecord {
//    title: String,
    fields: Vec<ParsedField>,
}

impl ParsedRecord {
    fn of(fields: Vec<ParsedField>) -> Self {
        ParsedRecord { fields }
    }

    fn unknown(line: String) -> Self {
        ParsedRecord { fields: vec![
            ParsedField::unknown(&line),
        ] }
    }
}

#[derive(Debug)]
struct ParsedField {
    raw_value: String,
    field_spec: Option<RecordField>,
}

impl ParsedField {
    fn of(raw_value: String, field_spec: &RecordField) -> Self {
        ParsedField {
            raw_value: raw_value.to_string(),
            field_spec: Some(field_spec.clone()),
        }
    }

    fn unknown(raw_value: &str) -> Self {
        ParsedField {
            raw_value: raw_value.to_string(),
            field_spec: None,
        }
    }
}

async fn parse_records(bytes: Vec<u8>, spec: RecordSpec, link: ComponentLink<Welcome>) {
    let lines = str::from_utf8(&bytes).unwrap().lines();

    let mut records = vec![];

    for line in lines {
        records.push(if let Some(record_type) = spec.get_record_type(line) {
            let mut fields = vec![];

            let mut idx = 1;

            for field in &record_type.fields {
                assert![ field.start <= idx, "Start must be smaller than index" ];
                if field.start != idx {
                    fields.push(ParsedField::unknown(&line[(idx - 1)..field.start]));
                }
                fields.push(field.parse(line));
                idx = field.start + field.length;
            }
            if line.len() != idx {
                fields.push(ParsedField::unknown(&line[(idx - 1)..]));
            }

            ParsedRecord::of(fields)
        } else {
            ParsedRecord::unknown(line.to_string())
        });
    }

    link.send_message(Msg::RecordsParsed(records))
}

pub struct Welcome {
    link: ComponentLink<Self>,
    
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

impl Component for Welcome {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Welcome {
            link,
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
                let spec = RecordSpec {
                    key_field: RecordField::key(8, 4),
                    records: vec![
                        RecordType {
                            key: "1000".to_string(),
                            fields: vec![
                                RecordField {
                                    field_number: 0,
                                    start: 1,
                                    length: 7,
                                    name: "Line nr.".to_string(),
                                    formatter: FieldFormatter::Integer,
                                },
                                RecordField {
                                    field_number: 0,
                                    start: 8,
                                    length: 4,
                                    name: "Record nr.".to_string(),
                                    formatter: FieldFormatter::Raw,
                                },
                            ]
                        }
                    ],
                };

                spawn_local(parse_records(bytes, spec, self.link.clone()));
            },
            Msg::RecordsParsed(records) => {
                self.records = Some(records);
            },
        }

        true
    }

    fn view(&self) -> Html {
        let renderField = |field: &ParsedField| { html! {
            <div>
                <p> { field.field_spec.as_ref().map(|spec| spec.name.clone()).unwrap_or("Unknown".to_string()) } </p>
                { field.field_spec.as_ref().map(|spec|
                    match spec.formatter.format(field.raw_value.clone()) {
                        Ok(parsed) => parsed,
                        _ => "PARSE_ERROR".to_string()
                    })
                    .unwrap_or(field.raw_value.clone()) }
            </div>
        } };

        let renderRecord = |record: &ParsedRecord| { html! {
            <div>
                { for record.fields.iter().map(renderField) }
            </div>
        } };

        let recordsView = if let Some(records) = &self.records {
            html! { for records.iter().map(renderRecord) }
        } else {
            html!{ { "Upload a file to show records" } }
        };

        html! {
            <>
                <h1>{ "eIndkomst loader"}</h1>
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
