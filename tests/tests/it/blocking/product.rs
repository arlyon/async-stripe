use stripe_product::product::{CreateProduct, Features};

use crate::mock::get_client;

#[test]
// FIXME: stripe-mock is missing required `type` field
#[ignore]
// https://github.com/arlyon/async-stripe/issues/437
fn create_product() {
    let client = get_client();

    let mut create = CreateProduct::new("my product");
    let features = vec![Features::new("great feature")];
    create.features = Some(&features);

    let product = create.send(&client).unwrap();
    assert_eq!(product.features.first().unwrap().name, Some("great feature".into()));
}
