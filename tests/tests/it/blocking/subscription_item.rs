use chrono::Utc;
use stripe_billing::usage_record::{
    CreateSubscriptionItemUsageRecord, CreateSubscriptionItemUsageRecordAction,
    CreateSubscriptionItemUsageRecordTimestamp,
};

use super::get_client;

#[test]
fn can_create_usage_record() {
    let client = get_client();

    let subscription_item_id = "si_JVbsG8wiy20ycs".parse().unwrap();
    let usage_record = CreateSubscriptionItemUsageRecord::new(&subscription_item_id, 42)
        .action(CreateSubscriptionItemUsageRecordAction::Increment)
        .timestamp(CreateSubscriptionItemUsageRecordTimestamp::Timestamp(Utc::now().timestamp()))
        .send_blocking(&client)
        .unwrap();
    assert_eq!(usage_record.quantity, 42);
    assert_eq!(usage_record.subscription_item, subscription_item_id.as_str());
}
