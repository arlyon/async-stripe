use stripe::{Customer, ListCustomers, RangeBounds, RangeQuery};

fn main() {
    // Create a new client
    let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");
    let client = stripe::Client::new(secret_key);

    // List customers
    let customers = Customer::list(
        &client,
        ListCustomers {
            limit: Some(3),
            created: Some(RangeQuery::gte(1501598702)),
            starting_after: None,
            ending_before: None,
            email: None,
            expand: &[],
        },
    )
    .unwrap();

    // Print the first three customers
    println!("{:?}", customers);

    // List the next three customers (using `new`)
    let mut params = ListCustomers::new();
    params.limit = Some(3);
    params.starting_after = customers.data.last().map(|cust| cust.id.clone());
    let customers2 = Customer::list(&client, params).unwrap();

    // Print the following three customers
    println!("{:?}", customers2);

    // List all customers within a given time range
    let mut params = ListCustomers::new();
    params.created = Some(RangeQuery::Bounds(RangeBounds {
        gt: None,
        gte: customers.data[0].created.map(|x| x as i64),
        lt: None,
        lte: customers2.data.last().and_then(|cust| cust.created.map(|x| x as i64)),
    }));
    let customers3 = Customer::list(&client, params).unwrap();

    // Print all customers create in the time range
    println!("{:?}", customers3);
}
