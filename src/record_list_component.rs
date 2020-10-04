use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::console;

use crate::record_component::*;
use crate::record_parser::*;

use std::rc::Rc;
use std::collections::HashMap;

pub struct RecordListComponent {
    link: ComponentLink<Self>,
    props: RecordListComponentProps,
}

#[derive(Clone, Properties)]
pub struct RecordListComponentProps {
    pub recordspec: Rc<RecordSpec>,
    pub records: Vec<Rc<RecordHierarchy>>,
    pub errors: Option<Rc<HashMap<usize, Error>>>,
}

impl Component for RecordListComponent {
    type Message = ();
    type Properties = RecordListComponentProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        RecordListComponent {
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
        let render_record = |record: &Rc<RecordHierarchy>| {
            html! {
                <RecordComponent record=record
                                 errors=self.props.errors.clone()
                                 recordspec=self.props.recordspec.clone() />
            }
        };

        html! { for self.props.records.iter().map(|record| render_record(record)) }
    }
}
