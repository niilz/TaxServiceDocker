use crate::button::Button;
use crate::input::Input;
use crate::web::fetch_tax;
use crate::Tax;
use anyhow::Error;
use yew::services::ConsoleService;
use yew::web_sys::FocusEvent;

pub struct Form {
    link: yew::ComponentLink<Self>,
    task: Option<yew::services::fetch::FetchTask>,
    good: Option<String>,
    country: Option<String>,
    res: Option<Tax>,
    no_result: bool,
}

#[derive(Debug)]
pub enum Action {
    Fetch,
    Submit,
    Success(anyhow::Result<Tax, Error>),
    Failed(Error),
    UpdateGood(String),
    UpdateCountry(String),
}

impl yew::Component for Form {
    type Message = Action;
    type Properties = ();
    fn create(_: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        ConsoleService::log("new Form");
        Form {
            link,
            task: None,
            good: None,
            country: None,
            res: None,
            no_result: false,
        }
    }
    fn update(&mut self, msg: Self::Message) -> bool {
        self.no_result = false;
        match msg {
            Action::Fetch => {
                ConsoleService::log("fetch: update=true");
                self.res = None;
                let good = self.good.as_ref().unwrap();
                let country = self.country.as_ref().unwrap();
                let tax = Tax::new(&good, &country);
                self.task = Some(fetch_tax(tax, self.link.clone()));
                // println!("After fetch_tax: {:?}", self.task);
            }
            Action::Submit => { /* Just do nothing on submit (preventDefault)*/ }
            Action::Success(Ok(tax_json)) => {
                ConsoleService::log(&format!("received tax-json: {:?}", tax_json));
                self.res = Some(tax_json);
            }
            Action::Success(Err(e)) => {
                ConsoleService::log(&format!("No entity found: {}", e));
                self.no_result = true;
                self.res = None;
            }
            Action::Failed(e) => {
                ConsoleService::log(&format!("fetch totally failed with {}", e));
                self.res = None;
            }
            Action::UpdateGood(good) => self.good = Some(good),
            Action::UpdateCountry(country) => self.country = Some(country),
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        ConsoleService::log("change Form");
        false
    }

    fn view(&self) -> yew::Html {
        ConsoleService::log("render view Form");
        let submit_cb = self.link.callback(|e: FocusEvent| {
            e.prevent_default();
            Action::Submit
        });
        let res = if self.res.is_some() {
            yew::html!(<div>{self.res.as_ref().unwrap()}</div>)
        } else if self.no_result {
            yew::html!(<div>{"No result found"}</div>)
        } else {
            yew::html!()
        };
        yew::html! {
            <>
                <form onsubmit=submit_cb>
                    <Input placeholder="Please enter good" handle_input=self.link.callback(|g| Action::UpdateGood(g))/>
                    <Input placeholder="Please enter country" handle_input=self.link.callback(|c| Action::UpdateCountry(c))/>
                    <Button prevent_def=true text="Get Tax" handle_click=self.link.callback(|_| Action::Fetch)/>
                </form>
                {res}
            </>
        }
    }
}
