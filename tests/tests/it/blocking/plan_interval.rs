//! Basic tests to ensure that the plan interval types
//! are exported properly. Mainly just needs to compile.

use stripe_billing::plan::RetrievePlan;
use stripe_billing::subscription_item::{
    CreateSubscriptionItem, CreateSubscriptionItemPriceData,
    CreateSubscriptionItemPriceDataRecurring, CreateSubscriptionItemPriceDataRecurringInterval,
};
use stripe_billing::{PlanId, PlanInterval};
use stripe_types::Currency;

use super::get_client;

#[test]
fn can_create_plan() {
    let client = get_client();

    let id = PlanId::from("price_123");
    let plan = RetrievePlan::new(id).send_blocking(&client).unwrap();
    assert_eq!(plan.interval, PlanInterval::Month);
    assert_eq!(plan.amount, Some(2000));
}

#[test]
// See ignore reason in subscription testing
#[ignore]
fn can_create_subscription_plan_interval() {
    let client = get_client();

    let price_data = CreateSubscriptionItemPriceData::new(
        Currency::USD,
        "My Product",
        CreateSubscriptionItemPriceDataRecurring::new(
            CreateSubscriptionItemPriceDataRecurringInterval::Day,
        ),
    );

    let id = "sub_123";
    let _result =
        CreateSubscriptionItem::new(id).price_data(price_data).send_blocking(&client).unwrap();
}
