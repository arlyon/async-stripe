//! Setup and teardown for the stripe mock service.

use lazy_static::lazy_static;
use std::process::{Child, Command};
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref STRIPE_MOCK_PROCESS: Arc<Mutex<Option<Child>>> = Arc::new(Mutex::new(None));
}

pub fn with_client<T>(test: T) -> ()
where
    T: FnOnce(&stripe::Client) -> () + std::panic::UnwindSafe,
{
    let keep_alive = setup();

    let result = std::panic::catch_unwind(|| {
        let client = stripe::Client::from_url("http://localhost:12112", "sk_test_123");
        test(&client)
    });

    drop(keep_alive);
    teardown();
    assert!(result.is_ok())
}

fn setup() -> Arc<Mutex<Option<Child>>> {
    const STRIPE_MOCK_PATH: &str = "./stripe-mock/stripe-mock_0.79.0/stripe-mock";
    const STRIPE_MOCK_PORT: &str = "12112";
    const PATH_SPEC: &str = "./testing/openapi/spec3.json";
    const PATH_FIXTURES: &str = "./testing/openapi/fixtures3.json";

    // If we've already got a stripe mock process, then we've got nothing to do!
    let mut init = STRIPE_MOCK_PROCESS.lock().unwrap(/* poison */);
    if init.is_some() {
        return Arc::clone(&STRIPE_MOCK_PROCESS);
    }

    // Check our environment is configured correctly to run tests
    let env_port = std::env::var("STRIPE_MOCK_PORT");
    let port = env_port.as_ref().map(|v| v.as_str()).unwrap_or(STRIPE_MOCK_PORT);

    // Start the service
    let try_start = if std::fs::metadata(PATH_SPEC).is_ok() {
        Command::new(STRIPE_MOCK_PATH)
            .arg("-http-port")
            .arg(port)
            .arg("-spec")
            .arg(PATH_SPEC)
            .arg("-fixtures")
            .arg(PATH_FIXTURES)
            // Defaults to `inherit`
            // .stdout(Stdio::piped())
            // .stderr(Stdio::piped())
            .spawn()
    } else {
        Command::new(STRIPE_MOCK_PATH).arg("-http-port").arg(port).spawn()
    };
    let mut process = match try_start {
        Err(err) => panic!("failed to start `stripe-mock`: {}", err),
        Ok(child) => child,
    };

    // TODO: Could we poll `stdout` instead of sleeping here?
    std::thread::sleep(std::time::Duration::from_millis(500));

    // Check to make sure the process didn't exit
    match process.try_wait() {
        Ok(Some(status)) => panic!("process `stripe-mock` exited early: {}", status),
        Ok(None) => (/* ok */),
        Err(err) => panic!("failed to poll `stripe-mock`: {}", err),
    }

    // Initialize global
    if init.is_none() {
        init.replace(process);
    } else {
        // This should be unreachable, but lets kill the process
        // we started if this happens; just in case.
        process.kill().ok();
        unreachable!();
    }

    Arc::clone(&STRIPE_MOCK_PROCESS)
}

fn teardown() {
    // Lock the global to prevent `setup` from attempting to initialize it
    let mut deinit = STRIPE_MOCK_PROCESS.lock().unwrap(/* poison */);

    // Stop the mock process if the static pointer is the last pointer
    if Arc::strong_count(&STRIPE_MOCK_PROCESS) <= 1 {
        match deinit.take() {
            Some(mut child) => {
                child.kill().ok();
            }
            None => (),
        }
    }
}
