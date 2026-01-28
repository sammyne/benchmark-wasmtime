mod bindgen {
    wit_bindgen::generate!();

    use crate::App;

    export!(App);
}

use bindgen::exports::sammyne::argon2::api::Guest;

pub struct App;

impl Guest for App {
    fn hash(password: Vec<u8>, salt: String) -> Result<Vec<u8>, String> {
        crate::hash(&password, &salt).map_err(|err| err.to_string())
    }
}
