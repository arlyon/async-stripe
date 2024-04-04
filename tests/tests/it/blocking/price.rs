use std::collections::HashMap;

use stripe_product::price::{UpdatePrice, UpdatePriceCurrencyOptions};
use stripe_product::PriceTaxBehavior;
use stripe_types::Currency;

use crate::mock::get_client;

#[test]
// https://github.com/arlyon/async-stripe/issues/417
fn update_price() {
    let client = get_client();

    let mut update = UpdatePrice::new();
    let mut currency_opts = HashMap::new();
    let mut opt = UpdatePriceCurrencyOptions::new();
    opt.unit_amount = Some(4);
    currency_opts.insert(Currency::USD, opt);
    update.currency_options = Some(&currency_opts);

    let price_id = "price_123".parse().unwrap();

    let price = update.send(&client, &price_id).unwrap();
    assert_eq!(price.id, price_id);
    assert_eq!(price.tax_behavior, Some(PriceTaxBehavior::Unspecified));
}
