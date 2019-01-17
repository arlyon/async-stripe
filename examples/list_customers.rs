use stripe::{Customer, CustomerListParams, RangeBounds, RangeQuery};

fn main() {
    // Create a new client
    let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");
    let client = stripe::Client::new(secret_key);

    // List customers
    let customers = Customer::list(
        &client,
        CustomerListParams {
            created: Some(RangeQuery::gte(1501598702)),
            ending_before: None,
            limit: Some(3),
            starting_after: None,
        },
    )
    .unwrap();

    // Print the first three customers
    println!("{:?}", customers);

    // List the next three customers (using default)
    let mut params = CustomerListParams::default();
    params.limit = Some(3);
    params.starting_after = customers.data.last().map(|cust| cust.id.as_str());
    let customers2 = Customer::list(&client, params).unwrap();

    // Print the following three customers
    println!("{:?}", customers2);

    // List all customers within a given time range
    let range = RangeQuery::Bounds(RangeBounds {
        gt: None,
        gte: Some(customers.data[0].created as i64),
        lt: None,
        lte: customers2.data.last().map(|cust| cust.created as i64),
    });
    let mut params = CustomerListParams::default();
    params.created = Some(range);
    let customers3 = Customer::list(&client, params).unwrap();

    // Print all customers create in the time range
    println!("{:?}", customers3);
}
