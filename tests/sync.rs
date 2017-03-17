extern crate stripe;

use std::sync::Arc;
use std::thread;

#[test]
fn sync() {
    let client = Arc::new(stripe::Client::new("".to_string()));
    let clone1 = client.clone();
    let clone2 = client.clone();
    thread::spawn(move || {
        assert!(stripe::Customer::get(&clone1, "").is_err());
    });
    thread::spawn(move || {
        assert!(stripe::Customer::get(&clone2, "").is_err());
    });
}
