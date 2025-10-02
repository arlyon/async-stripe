use stripe_product::promotion_code::{
    CreatePromotionCode, CreatePromotionCodePromotion, CreatePromotionCodePromotionType,
};

use super::get_client;

#[test]
// https://github.com/arlyon/async-stripe/issues/389
fn create_promotion_code() {
    let client = get_client();

    let result = CreatePromotionCode::new(CreatePromotionCodePromotion {
        coupon: Some("test".to_string()),
        type_: CreatePromotionCodePromotionType::Coupon,
    })
    .active(true)
    .send_blocking(&client)
    .unwrap();
    assert!(result.active);
}
