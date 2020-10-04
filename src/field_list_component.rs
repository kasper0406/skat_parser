use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::console;

use crate::field_component::*;
use crate::record_parser::*;

use std::rc::Rc;

pub struct FieldListComponent {
    link: ComponentLink<Self>,
    props: FieldListComponentProps,
}

#[derive(Clone, Properties)]
pub struct FieldListComponentProps {
    pub fields: Vec<ParsedField>,
    pub error: Option<Error>,
}

impl Component for FieldListComponent {
    type Message = ();
    type Properties = FieldListComponentProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        FieldListComponent {
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
        let render_field = |field: &ParsedField| {
            html! {
                <FieldComponent field=field.clone()
                                error=self.props.error.clone() />
            }
        };

        html! {
            <div class="fields">
                { for self.props.fields.iter().map(render_field) }
            </div>
        }
    }
}
