mod bindgen {
    wit_bindgen::generate!();

    use crate::App;

    export!(App);
}

use bindgen::exports::sammyne::argon2::api::Guest;

pub struct App;

impl Guest for App {
    fn hash(password: Vec<u8>, salt: String) -> Result<Vec<u8>, String> {
        crate::api::hash(&password, &salt).map_err(|err| err.to_string())
    }

    fn must_hash(password: String, salt: String) -> u8 {
        crate::api::must_hash(&password, &salt)
    }
}
