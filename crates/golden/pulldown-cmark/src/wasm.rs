mod bindgen {
    wit_bindgen::generate!();

    use crate::App;

    export!(App);
}

use bindgen::exports::sammyne::pulldown_cmark::api::Guest;

pub struct App;

impl Guest for App {
    fn parse(markdown: String) -> Result<String, String> {
        crate::parse(&markdown).map_err(|err| err.to_string())
    }
}
