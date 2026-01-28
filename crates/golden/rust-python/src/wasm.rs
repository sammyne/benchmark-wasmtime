mod bindgen {
    wit_bindgen::generate!();

    use crate::App;

    export!(App);
}

use bindgen::exports::sammyne::rust_python::api::Guest;

pub struct App;

impl Guest for App {
    fn run(code: String) -> Result<String, String> {
        crate::run(&code).map_err(|err| err.to_string())
    }
}
