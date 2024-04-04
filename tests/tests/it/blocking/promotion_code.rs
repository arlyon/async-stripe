use stripe_product::promotion_code::CreatePromotionCode;

use crate::mock::get_client;

#[test]
// https://github.com/arlyon/async-stripe/issues/389
fn create_promotion_code() {
    let client = get_client();

    let mut create = CreatePromotionCode::new("code");
    create.active = Some(true);

    let result = create.send(&client).unwrap();
    assert!(result.active);
}
