use yew::events::{ChangeData, InputData};
use yew::services::ConsoleService;

pub struct Input {
    link: yew::ComponentLink<Self>,
    props: InputProps,
}

#[derive(Debug, Clone, yew::Properties)]
pub struct InputProps {
    pub placeholder: String,
    pub handle_input: yew::Callback<String>,
}

#[derive(Debug)]
pub enum InputMsg {
    Change(String),
    Input(String),
    UnsupportedChangeDataVariant,
}

impl yew::Component for Input {
    type Message = InputMsg;
    type Properties = InputProps;
    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Input { link, props }
    }
    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            InputMsg::Change(value) | InputMsg::Input(value) => {
                self.props.handle_input.emit(value);
                true
            }
            InputMsg::UnsupportedChangeDataVariant => {
                ConsoleService::log("This ChangeDataVariant is not supported");
                false
            }
        }
    }
    fn change(&mut self, _: Self::Properties) -> bool {
        ConsoleService::log("input change got called");
        false
    }
    fn view(&self) -> yew::Html {
        let change_cb = self.link.callback(|e| {
            if let ChangeData::Value(v) = e {
                InputMsg::Change(v)
            } else {
                InputMsg::UnsupportedChangeDataVariant
            }
        });
        let input_cb = self.link.callback(|e: InputData| InputMsg::Change(e.value));
        yew::html! {
            <input onchange=change_cb oninput=input_cb type="text" placeholder={&self.props.placeholder} />
        }
    }
}
