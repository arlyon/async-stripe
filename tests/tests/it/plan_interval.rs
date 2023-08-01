//! Basic tests to ensure that the plan interval types
//! are exported properly. Mainly just needs to compile.

use stripe_billing::plan;
use stripe_billing::plan::requests::RetrievePlan;
use stripe_billing::subscription_item::requests::CreateSubscriptionItem;
use stripe_types::plan::PlanInterval;

use crate::mock;

#[test]
fn can_create_plan() {
    let id = "price_123".parse().unwrap();
    mock::with_client(|client| {
        let mut plan = plan::requests::retrieve(client, &id, RetrievePlan::new()).unwrap();
        plan.interval = PlanInterval::Month;
    });
}

#[test]
fn can_create_subscription_interval() {
    let _recurring = stripe::SubscriptionPriceDataRecurring {
        interval: stripe::SubscriptionInterval::Month,
        interval_count: Some(100),
    };
}

#[test]
fn can_create_subscription_plan_interval() {
    mock::with_client(|client| {
        let id = "sub_123";
        let mut create = CreateSubscriptionItem::new(id);
        create.price_data = Some(SubscriptionItem {
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
