extern crate serde_json as json;
extern crate stripe;

#[test]
#[ignore] // TODO: Figure out how to run this test with an sk_key, etc
fn customer_delete() {
    let client = stripe::Client::new("sk_key");
    let result = stripe::Customer::delete(&client, "cus_example_id");
    match result {
        Ok(deleted) => assert!(deleted.deleted, "Customer wasn't deleted"),
        Err(err) => assert!(false, format!("{}", err)),
    }
}
