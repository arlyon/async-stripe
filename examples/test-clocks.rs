use std::time::{Duration, SystemTime, UNIX_EPOCH};

use futures_util::TryStreamExt;
use stripe::{
    AdvanceTestClock, Client, CreateTestClock, ListTestClocks, TestHelpersTestClockStatus,
};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");
    let client = Client::new(secret_key);

    let now = SystemTime::now();
    let timestamp = now.duration_since(UNIX_EPOCH).unwrap().as_secs();

    let test_clock_1 = stripe::TestHelpersTestClock::create(
        &client,
        &CreateTestClock { frozen_time: timestamp as i64, name: "Example test clock 1" },
    )
    .await
    .unwrap();

    assert_eq!(test_clock_1.status, Some(TestHelpersTestClockStatus::Ready));
    println!("created a test clock at https://dashboard.stripe.com/test/billing/subscriptions/test-clocks/{}", test_clock_1.id);

    let test_clock_2 = stripe::TestHelpersTestClock::create(
        &client,
        &CreateTestClock { frozen_time: timestamp as i64, name: "Example test clock 2" },
    )
    .await
    .unwrap();

    assert_eq!(test_clock_2.status, Some(TestHelpersTestClockStatus::Ready));
    println!("created a test clock at https://dashboard.stripe.com/test/billing/subscriptions/test-clocks/{}", test_clock_2.id);

    let mut all_test_clocks_params = ListTestClocks::default();
    all_test_clocks_params.limit = Some(1); // Force pagination to happen
    let all_test_clocks = stripe::TestHelpersTestClock::list(&client, &ListTestClocks::default())
        .await
        .unwrap()
        .paginate(ListTestClocks::default())
        .stream(&client)
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    assert_eq!(all_test_clocks.len(), 2);
    println!(
        "all test clocks: {:?}",
        all_test_clocks.into_iter().map(|test_clock| test_clock.id).collect::<Vec<_>>()
    );

    let new_timestamp = timestamp + (60 * 60 * 60);
    let mut test_clock_1 = stripe::TestHelpersTestClock::advance(
        &client,
        &test_clock_1.id,
        &AdvanceTestClock { frozen_time: new_timestamp as i64 },
    )
    .await
    .unwrap();
    assert_eq!(test_clock_1.status, Some(TestHelpersTestClockStatus::Advancing));
    println!("advancing test clock {} to {}", test_clock_1.id, new_timestamp);

    while test_clock_1.status == Some(TestHelpersTestClockStatus::Advancing) {
        println!("test clock {} is still advancing...", test_clock_1.id);
        sleep(Duration::from_secs(1)).await;

        test_clock_1 =
            stripe::TestHelpersTestClock::retrieve(&client, &test_clock_1.id).await.unwrap();
    }
    println!("test clock {} is now on status {}", test_clock_1.id, test_clock_1.status.unwrap());

    let deleted_test_clock_1 =
        stripe::TestHelpersTestClock::delete(&client, &test_clock_1.id).await.unwrap();
    assert!(deleted_test_clock_1.deleted);
    println!("delete test clock {}", deleted_test_clock_1.id);

    let deleted_test_clock_2 =
        stripe::TestHelpersTestClock::delete(&client, &test_clock_2.id).await.unwrap();
    assert!(deleted_test_clock_2.deleted);
    println!("delete test clock {}", deleted_test_clock_2.id);
}
