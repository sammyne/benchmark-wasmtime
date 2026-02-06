mod api;

fn main() {
    let data = std::env::args().skip(1).next().expect("miss data");

    let _z = std::hint::black_box(api::zip(data.as_bytes())).expect("zip failed");
}
