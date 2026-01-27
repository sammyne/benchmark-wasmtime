mod bindgen {
    wit_bindgen::generate!();

    use crate::App;

    export!(App);
}

use bindgen::exports::sammyne::sevenz7z::api::Guest;

pub struct App;

impl Guest for App {
    fn unzip(req: Vec<u8>) -> Result<Vec<u8>, String> {
        crate::unzip(&req).map_err(|err| err.to_string())
    }

    fn zip(req: Vec<u8>) -> Result<Vec<u8>, String> {
        crate::zip(&req).map_err(|err| err.to_string())
    }
}
