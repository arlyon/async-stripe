use chrono::Utc;
use stripe_billing::usage_record::{
    CreateUsageRecord, CreateUsageRecordAction, CreateUsageRecordTimestamp,
};

use crate::mock::get_client;

#[test]
fn can_create_usage_record() {
    let client = get_client();

    let subscription_item_id = "si_JVbsG8wiy20ycs".parse().unwrap();
    let creator = CreateUsageRecord {
        quantity: 42,
        action: Some(CreateUsageRecordAction::Increment),
        timestamp: Some(CreateUsageRecordTimestamp::Timestamp(Utc::now().timestamp())),
        expand: None,
    };
    let usage_record = creator.send(&client, &subscription_item_id).unwrap();
    assert_eq!(usage_record.quantity, 42);
    assert_eq!(usage_record.subscription_item, subscription_item_id.as_str());
}
