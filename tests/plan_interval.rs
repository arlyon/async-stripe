//! Basic tests to ensure that the plan interval types
//! are exported properly. Mainly just needs to compile.

mod mock;

#[test]
#[cfg(feature = "blocking")]
fn can_create_plan() {
    let id = "price_123".parse().unwrap();
    mock::with_client(|client| {
        let mut plan = stripe::Plan::retrieve(client, &id, &[]).unwrap();
        plan.interval = Some(stripe::PlanInterval::Month);
    });
}

#[test]
#[cfg(feature = "blocking")]
fn can_create_subscription_interval() {
    let _recurring = stripe::SubscriptionPriceDataRecurring {
        interval: stripe::SubscriptionInterval::Month,
        interval_count: Some(100),
    };
}

#[test]
#[cfg(feature = "blocking")]
fn can_create_subscription_plan_interval() {
    mock::with_client(|client| {
        let id = "sub_123".parse().unwrap();
        let mut create = stripe::CreateSubscriptionItem::new(id);
        create.price_data = Some(stripe::SubscriptionItemPriceData {
            currency: stripe::Currency::USD,
            product: "My Product".to_string(),
            recurring: stripe::SubscriptionItemPriceDataRecurring {
                interval: stripe::SubscriptionItemInterval::Day,
                interval_count: Some(6),
            },
            tax_behavior: None,
            unit_amount: None,
            unit_amount_decimal: None,
        });
        let _result = stripe::SubscriptionItem::create(client, create).unwrap();
    });
}
