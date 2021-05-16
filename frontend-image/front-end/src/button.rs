use crate::form::Action;
use yew::services::ConsoleService;

#[derive(Debug)]
pub struct Button {
    link: yew::ComponentLink<Self>,
    props: ButtonProps,
    is_loggedin: bool,
}

#[derive(Debug, Clone, yew::Properties)]
pub struct ButtonProps {
    pub handle_click: yew::Callback<Action>,
    pub text: String,
    pub prevent_def: bool,
}

pub enum Interaction {
    Click,
}

impl yew::Component for Button {
    type Message = Interaction;
    type Properties = ButtonProps;
    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        ConsoleService::log("new Button");
        Button {
            link,
            props,
            is_loggedin: false,
        }
    }
    fn update(&mut self, msg: Self::Message) -> bool {
        ConsoleService::log("updated Button");
        match msg {
            Interaction::Click => {
                self.props.handle_click.emit(Action::Fetch);
                true
            }
        }
    }
    fn change(&mut self, _: Self::Properties) -> bool {
        ConsoleService::log("Change Button got called");
        false
    }
    fn view(&self) -> yew::Html {
        ConsoleService::log("called view Button");
        let click_cb = self.link.callback(|_| Interaction::Click);
        yew::html! {
            <button onclick=click_cb >{&self.props.text}</button>
        }
    }
}
