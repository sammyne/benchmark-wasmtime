mod api;

fn main() {
    let markdown = std::env::args().skip(1).next().expect("miss arg");

    let _parsed = api::validate(&markdown).expect("parse");
}
