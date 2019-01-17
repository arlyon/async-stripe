#[test]
fn sync() {
    let client = stripe::Client::new("sk_key");
    let client = std::sync::Arc::new(client);
    let clone1 = client.clone();
    let clone2 = client.clone();
    std::thread::spawn(move || {
        assert!(stripe::Customer::retrieve(&clone1, "").is_err());
    });
    std::thread::spawn(move || {
        assert!(stripe::Customer::retrieve(&clone2, "").is_err());
    });
}
