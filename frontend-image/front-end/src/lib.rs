#![recursion_limit = "256"]
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

mod button;
mod form;
mod input;
mod web;
use form::Form;

#[derive(Debug, Serialize, Deserialize)]
pub struct Tax {
    id: Option<u32>,
    good: String,
    amount: Option<u32>,
    destination: Country,
}
impl Tax {
    pub fn new(good: &str, country: &str) -> Self {
        Tax {
            id: None,
            good: good.to_string(),
            amount: None,
            destination: Country::new(country),
        }
    }
}
impl std::fmt::Display for Tax {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            fmt,
            "id: {:?}, good: {}, amount: {:?}, dest: {}",
            self.id, self.good, self.amount, self.destination
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Country {
    name: String,
    id: Option<u32>,
}
impl Country {
    fn new(name: &str) -> Self {
        Country {
            id: None,
            name: name.to_string(),
        }
    }
}

impl std::fmt::Display for Country {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "id: {:?}, name: {}", self.id, self.name)
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::App::<Form>::new().mount_to_body();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
