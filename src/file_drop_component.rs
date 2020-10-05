use wasm_bindgen::prelude::*;
use yew::prelude::*;
use web_sys::console;

use futures::future;
use futures::stream::{self, StreamExt};

use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;

#[derive(Clone, Properties)]
pub struct FileDropComponentProps {
    pub onfileloaded: Callback<Vec<u8>>,
    pub text: String,
}

pub struct FileDropComponent {
    link: ComponentLink<Self>,
    props: FileDropComponentProps,

    error: Option<String>,
    loading: bool,
}

pub enum Msg {
    DragOver(DragEvent),
    FileDropped(DragEvent),
}

async fn load_bytes_from_stream(callback: Callback<Vec<u8>>, js_stream: ws::ReadableStream) {
    let stream = wasm_streams::ReadableStream::from_raw(js_stream.dyn_into().unwrap()).into_stream();

    let mut buffer = vec![];
    stream.for_each(|item| {
        let array = js_sys::Uint8Array::new(&item.unwrap());
        buffer.append(&mut array.to_vec());
        
        future::ready(())
    }).await;

    callback.emit(buffer);
}

impl Component for FileDropComponent {
    type Message = Msg;
    type Properties = FileDropComponentProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        FileDropComponent {
            link,
            props,
            error: None,
            loading: false,
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
                self.loading = true;

                console::log_1(&"File dropped".into());
                if let Some(files) = event.data_transfer().unwrap().files() {
                    console::log_1(&format!("{} files dropped", files.length()).into());
                    if files.length() != 1 {
                        self.error = Some("Exactly one file should be processed!".into());
                    }

                    let file = files.get(0).unwrap();
                    console::log_1(&format!("Received file {}", file.name()).into());

                    let js_stream = file.stream();
                    
                    spawn_local(load_bytes_from_stream(self.props.onfileloaded.clone(), js_stream));
                }

                event.prevent_default();
            },
        }

        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                {
                    if let Some(error) = &self.error {
                        html! { <p> { error } </p> }
                    } else {
                        html! { }
                    }
                }
                
                <div ondragover=self.link.callback(|event| Msg::DragOver(event))
                    ondrop=self.link.callback(|event| Msg::FileDropped(event))
                    class="dropzone">
                    { if self.loading {
                        "Loading...".to_string()
                      } else {
                        self.props.text.clone()
                      }
                    }
                </div>
            </>
        }
    }
}
