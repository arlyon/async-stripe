fn main() {
    // Dynamic inputs keep signature verification and typed event dispatch reachable.
    // This is a compilation fixture; the collector never executes it.
    let mut args = std::env::args().skip(1);
    let payload = args.next().unwrap_or_else(|| "{}".into());
    let signature = args.next().unwrap_or_default();
    let secret = args.next().unwrap_or_default();
    std::hint::black_box(stripe_webhook::Webhook::construct_event(&payload, &signature, &secret))
        .ok();
}
