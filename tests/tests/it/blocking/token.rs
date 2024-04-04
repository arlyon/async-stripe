use stripe_core::token::{CreateToken, CreateTokenAccount, CreateTokenAccountBusinessType};

use crate::mock::get_client;

// https://github.com/arlyon/async-stripe/issues/423
#[test]
fn create_account_token_smoke_test() {
    let client = get_client();

    let mut acct = CreateTokenAccount::new();
    acct.tos_shown_and_accepted = Some(true);
    acct.business_type = Some(CreateTokenAccountBusinessType::Individual);
    let mut create = CreateToken::new();
    create.account = Some(acct);

    let stripe_acct_token = create.send(&client).unwrap();
    assert!(!stripe_acct_token.used);
    assert!(!stripe_acct_token.livemode);
}
