mod mock;

// Using fixture for this test because the stripe-mock server does not (currently [2023-05-25]) support the `currency_options` field.
// See: https://github.com/stripe/stripe-mock/issues/420
#[test]
#[cfg(feature = "blocking")]
fn deserialize_currency_options() {
    use stripe::{Currency, CurrencyOptionTaxBehavior, Price};

    let fixture = r#"
    {
        "id": "price_1234",
        "object": "price",
        "active": true,
        "billing_scheme": "per_unit",
        "created": 1651739124,
        "currency": "usd",
        "currency_options": {
            "eur": {
                "custom_unit_amount": null,
                "tax_behavior": "exclusive",
                "unit_amount": 14388,
                "unit_amount_decimal": "14388"
            },
            "usd": {
                "custom_unit_amount": null,
                "tax_behavior": "exclusive",
                "unit_amount": 14388,
                "unit_amount_decimal": "14388"
            }
        },
        "custom_unit_amount": null,
        "livemode": false,
        "lookup_key": "lookup_key_1234",
        "metadata": {},
        "nickname": null,
        "product": "prod_1234",
        "recurring": {
            "aggregate_usage": null,
            "interval": "year",
            "interval_count": 1,
            "trial_period_days": null,
            "usage_type": "licensed"
        },
        "tax_behavior": "exclusive",
        "tiers_mode": null,
        "transform_quantity": null,
        "type": "recurring",
        "unit_amount": 14388,
        "unit_amount_decimal": "14388"
    }
    "#;

    let result: Result<Price, serde_json::Error> = serde_json::from_str(fixture);
    assert!(result.is_ok());

    let price = result.unwrap();
    assert!(&price.currency_options.is_some());

    let currency_options = price.currency_options.unwrap();
    assert!(currency_options.contains_key(&Currency::USD));
    assert!(currency_options.contains_key(&Currency::EUR));

    let usd_option = currency_options.get(&Currency::USD).unwrap();
    assert!(usd_option.custom_unit_amount.is_none());
    assert_eq!(Some(14388), usd_option.unit_amount);
    assert_eq!(Some(CurrencyOptionTaxBehavior::Exclusive), usd_option.tax_behavior);
    assert_eq!(Some("14388".to_string()), usd_option.unit_amount_decimal);
}
