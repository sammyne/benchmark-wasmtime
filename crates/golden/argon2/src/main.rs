use std::hint;

mod api;

fn main() {
    let (password, salt) = {
        let mut args = std::env::args().skip(1);

        let p = args.next().expect("miss password");
        let s = args.next().expect("miss salt");

        (p, s)
    };

    let _h = hint::black_box(api::must_hash(&password, &salt));
}
