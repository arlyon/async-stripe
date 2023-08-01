use chrono::Utc;
use stripe_billing::usage_record::requests::{
    CreateUsageRecord, CreateUsageRecordAction, CreateUsageRecordTimestamp,
};

use crate::mock;

#[test]
fn can_create_usage_record() {
    let subscription_item_id = "si_JVbsG8wiy20ycs".parse().unwrap();
    mock::with_client(|client| {
        let usage_record = stripe_billing::usage_record::requests::create(
            client,
            &subscription_item_id,
            CreateUsageRecord {
                quantity: 42,
                action: Some(CreateUsageRecordAction::Increment),
                timestamp: Some(CreateUsageRecordTimestamp::I64(Utc::now().timestamp())),
                expand: None,
            },
        )
        .unwrap();
        assert_eq!(usage_record.quantity, 42);
        assert_eq!(usage_record.subscription_item, subscription_item_id.as_str());
    });
}
