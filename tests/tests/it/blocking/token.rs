use stripe_core::token::{CreateToken, CreateTokenAccount, CreateTokenAccountBusinessType};

use super::get_client;

// https://github.com/arlyon/async-stripe/issues/423
#[test]
fn create_account_token_smoke_test() {
    let client = get_client();

    let mut acct = CreateTokenAccount::new();
    acct.tos_shown_and_accepted = Some(true);
    acct.business_type = Some(CreateTokenAccountBusinessType::Individual);

    let stripe_acct_token = CreateToken::new().account(acct).send_blocking(&client).unwrap();
    assert!(!stripe_acct_token.used);
    assert!(!stripe_acct_token.livemode);
}
