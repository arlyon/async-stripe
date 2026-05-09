use futures_util::TryStreamExt;
use stripe::Client;
use stripe_billing::{invoice, subscription};
use stripe_core::{charge, customer, payment_intent, refund, setup_intent};
use stripe_payment::payment_method;
use stripe_product::{price, product};

#[tokio::main]
async fn main() -> Result<(), stripe::StripeError> {
    // Compile-time benchmark: we want the full codegen of request send paths
    // and pagination streams, but no network activity at runtime. Missing key
    // short-circuits immediately without invoking any of the async machinery.
    let Ok(key) = std::env::var("STRIPE_SECRET_KEY") else {
        return Ok(());
    };
    let client = Client::new(key);

    let _ = customer::RetrieveCustomer::new("cus_1").send(&client).await;
    let _ = charge::RetrieveCharge::new("ch_1").send(&client).await;
    let _ = payment_intent::RetrievePaymentIntent::new("pi_1").send(&client).await;
    let _ = setup_intent::RetrieveSetupIntent::new("seti_1").send(&client).await;
    let _ = refund::RetrieveRefund::new("re_1").send(&client).await;
    let _ = subscription::RetrieveSubscription::new("sub_1").send(&client).await;
    let _ = invoice::RetrieveInvoice::new("in_1").send(&client).await;
    let _ = price::RetrievePrice::new("price_1").send(&client).await;
    let _ = product::RetrieveProduct::new("prod_1").send(&client).await;
    let _ = payment_method::RetrievePaymentMethod::new("pm_1").send(&client).await;

    let _ = customer::CreateCustomer::new().send(&client).await;
    let _ = payment_intent::CreatePaymentIntent::new(2000, stripe_types::Currency::USD)
        .send(&client)
        .await;

    let customers_paginator = customer::ListCustomer::new().paginate();
    let mut customers = customers_paginator.stream(&client);
    while let Some(_) = customers.try_next().await? {}

    let charges_paginator = charge::ListCharge::new().paginate();
    let _all: Vec<_> = charges_paginator.stream(&client).try_collect().await?;

    let subs_paginator = subscription::ListSubscription::new().paginate();
    let _subs: Vec<_> = subs_paginator.stream(&client).try_collect().await?;

    Ok(())
}
