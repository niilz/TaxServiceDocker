// Compile project with:
// wasm-pack build --target web --out-name wasm --out-dir ./static/

use crate::{
    form::{Action, Form},
    Tax,
};
use anyhow::{Error, Result};
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::ConsoleService;

const TAX_SERVICE_URL: &'static str = "http://localhost:8080/app/service/get";

pub type JsonResp = Response<Json<Result<Tax, Error>>>;

pub fn fetch_tax(tax: Tax, link: yew::ComponentLink<Form>) -> FetchTask {
    let url = &format!(
        "{}?good={}&country={}",
        TAX_SERVICE_URL, tax.good, tax.destination.name
    );
    ConsoleService::log(&format!("making fetch to url: {}", url));

    let get_req = Request::get(url)
        .body(Nothing)
        .expect("Error during get_request");
    let cb = link.callback(move |res: JsonResp| match res.into_parts() {
        (meta, Json(res)) if meta.status.is_success() => Action::Success(res),
        (_, Json(Err(e))) => Action::Failed(e),
        _ => unreachable!(),
    });
    FetchService::fetch(get_req, cb).expect("Error during fetch")
}
