mod bindgen {
    wit_bindgen::generate!({
        with: { "sammyne:host/api@1.0.0": generate},
    });

    use crate::App;

    export!(App);
}

use bindgen::exports::sammyne::host_caller::api;
use bindgen::sammyne::host;

pub struct App;

impl api::Guest for App {
    fn multi_sleep(times: u64, duration_ms: u64) {
        for _i in 0..times {
            host::api::sleep(duration_ms);
        }
    }
}
