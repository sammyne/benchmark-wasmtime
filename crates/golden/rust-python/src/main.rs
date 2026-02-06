use std::hint;

mod api;

fn main() {
    let code = std::env::args().skip(1).next().expect("miss python code");

    let _h = hint::black_box(api::run(&code));
}
