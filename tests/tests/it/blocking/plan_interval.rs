//! Basic tests to ensure that the plan interval types
//! are exported properly. Mainly just needs to compile.

use stripe_billing::plan::RetrievePlan;
use stripe_billing::subscription_item::{
    CreateSubscriptionItem, CreateSubscriptionItemPriceData,
    CreateSubscriptionItemPriceDataRecurring, CreateSubscriptionItemPriceDataRecurringInterval,
};
use stripe_billing::PlanInterval;

use crate::mock::get_client;

#[test]
fn can_create_plan() {
    let client = get_client();

    let id = "price_123".parse().unwrap();
    let plan = RetrievePlan::new().send(&client, &id).unwrap();
    assert_eq!(plan.interval, PlanInterval::Month);
    assert_eq!(plan.amount, Some(2000));
}

#[test]
// See ignore reason in subscription testing
#[ignore]
fn can_create_subscription_plan_interval() {
    let client = get_client();

    let id = "sub_123";
    let mut create = CreateSubscriptionItem::new(id);
    create.price_data = Some(CreateSubscriptionItemPriceData::new(
        stripe_types::Currency::USD,
        "My Product",
        CreateSubscriptionItemPriceDataRecurring::new(
            CreateSubscriptionItemPriceDataRecurringInterval::Day,
        ),
    ));
    let _result = create.send(&client).unwrap();
}
