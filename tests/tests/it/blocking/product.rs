use stripe_product::product::{CreateProduct, Features};

use super::get_client;

#[test]
// FIXME: stripe-mock is missing required `type` field
#[ignore]
// https://github.com/arlyon/async-stripe/issues/437
fn create_product() {
    let client = get_client();

    let features = vec![Features::new("great feature")];
    let product = CreateProduct::new("my product")
        .marketing_features(&features)
        .send_blocking(&client)
        .unwrap();
    assert_eq!(product.marketing_features.first().unwrap().name, Some("great feature".into()));
}
