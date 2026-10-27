use std::process::Command;
use std::thread;

use stripe_webhook::{EventObject, Webhook};

const PAYLOAD: &str = r#"{
    "id": "evt_stack_test",
    "object": "event",
    "api_version": "2017-05-25",
    "created": 1492774577,
    "livemode": false,
    "pending_webhooks": 1,
    "type": "account.external_account.created",
    "data": {
        "object": {
            "object": "bank_account",
            "country": "us",
            "currency": "usd",
            "id": "ba_stack_test",
            "last4": "6789",
            "status": "verified"
        }
    }
}"#;

// Stack overflow aborts the process rather than unwinding. Isolate each check so
// regressions fail a normal test assertion instead of aborting the test suite.
fn on_small_stack(test_name: &str, check: impl FnOnce() + Send + 'static) {
    const CHILD_ENV: &str = "STRIPE_WEBHOOK_STACK_TEST_CHILD";
    if std::env::var(CHILD_ENV).as_deref() == Ok(test_name) {
        thread::Builder::new()
            .name(test_name.into())
            .stack_size(256 * 1024)
            .spawn(check)
            .unwrap()
            .join()
            .unwrap();
        return;
    }

    let output = Command::new(std::env::current_exe().unwrap())
        .args(["--exact", test_name, "--nocapture"])
        .env(CHILD_ENV, test_name)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{test_name} failed on a 256 KiB stack: {}\n{}\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
}

#[test]
fn signed_webhook_on_small_stack() {
    on_small_stack("signed_webhook_on_small_stack", || {
        let secret = "whsec_stack_test";
        let timestamp = 1492774577;
        let signature = Webhook::generate_test_header(PAYLOAD, secret, Some(timestamp));
        let event = Webhook::construct_event_with_timestamp(PAYLOAD, &signature, secret, timestamp)
            .unwrap();
        assert_eq!(event.id.as_str(), "evt_stack_test");
        assert!(matches!(event.data.object, EventObject::AccountExternalAccountCreated(_)));

        let invalid = PAYLOAD.replace("\"bank_account\"", "\"invalid_object\"");
        let signature = Webhook::generate_test_header(&invalid, secret, Some(timestamp));
        assert!(
            Webhook::construct_event_with_timestamp(&invalid, &signature, secret, timestamp)
                .is_err()
        );
    });
}

#[cfg(feature = "deserialize")]
#[test]
fn serde_webhook_on_small_stack() {
    on_small_stack("serde_webhook_on_small_stack", || {
        let event: stripe_webhook::Event = serde_json::from_str(PAYLOAD).unwrap();
        assert_eq!(event.id.as_str(), "evt_stack_test");
        assert!(matches!(event.data.object, EventObject::AccountExternalAccountCreated(_)));

        let invalid = PAYLOAD.replace("\"bank_account\"", "\"invalid_object\"");
        assert!(serde_json::from_str::<stripe_webhook::Event>(&invalid).is_err());
    });
}
