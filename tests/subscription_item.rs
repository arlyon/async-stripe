use chrono::Utc;

mod mock;

#[test]
#[cfg(feature = "blocking")]
fn can_create_usage_record() {
    let subscription_item_id = "si_JVbsG8wiy20ycs".parse().unwrap();
    mock::with_client(|client| {
        let usage_record = stripe::UsageRecord::create(
            client,
            &subscription_item_id,
            stripe::CreateUsageRecord {
                quantity: 42,
                action: Some(stripe::UsageRecordAction::Increment),
                timestamp: Some(Utc::now().timestamp()),
            },
        );
    });
}
