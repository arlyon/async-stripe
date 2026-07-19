use stripe_shared::Customer;

fn main() {
    let s = std::env::args().nth(1).unwrap_or_else(|| "{}".into());
    let v: Option<Customer> = stripe_miniserde::json::from_str(&s).ok();
    std::hint::black_box(v);
}
