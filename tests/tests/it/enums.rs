use std::str::FromStr;

use stripe_billing::plan::{CreatePlanInlineProductParams, CreatePlanProduct};
use stripe_connect::AccountType;
use stripe_core::EventType;
use stripe_product::price::CreatePriceTiersUpTo;

#[test]
fn enums_basic() {
    assert_eq!(AccountType::Express, AccountType::from_str("express").unwrap());
    assert_eq!(AccountType::Express.as_str(), "express");
    assert_eq!(serde_json::to_string(&AccountType::Express).unwrap(), r#""express""#);
    assert_eq!(
        miniserde::json::from_str::<AccountType>(r#""express""#).unwrap(),
        AccountType::Express
    );
    assert_eq!(serde_json::from_str::<AccountType>(r#""express""#).unwrap(), AccountType::Express);
    assert_eq!(AccountType::Express.to_string(), "express");
    assert_eq!(AccountType::from_str("unknown"), Ok(AccountType::Unknown(String::from("unknown"))));
}

#[test]
fn enums_requests() {
    assert_eq!(serde_json::to_string(&CreatePriceTiersUpTo::Inf).unwrap(), r#""inf""#);
    assert_eq!(serde_json::to_string(&CreatePriceTiersUpTo::I64(2)).unwrap(), r#"2"#);

    assert_eq!(serde_json::to_string(&CreatePlanProduct::Id("id".into())).unwrap(), r#""id""#);
    assert_eq!(
        serde_json::to_string(&CreatePlanProduct::InlineProductParams(
            CreatePlanInlineProductParams::new("my name")
        ))
        .unwrap(),
        r#"{"name":"my name"}"#
    );
}

#[test]
fn three_d_secure_card_union_serializes_both_payload_shapes() {
    use serde_json::json;
    use stripe_payment::three_d_secure_authentication::{
        CreateThreeDSecureAuthenticationPaymentMethodData,
        CreateThreeDSecureAuthenticationPaymentMethodDataCard,
        CreateThreeDSecureAuthenticationPaymentMethodDataCardCard,
        CreateThreeDSecureAuthenticationPaymentMethodDataCardToken,
        CreateThreeDSecureAuthenticationPaymentMethodDataType,
    };

    for (card, expected) in [
        (
            CreateThreeDSecureAuthenticationPaymentMethodDataCard::Card(
                CreateThreeDSecureAuthenticationPaymentMethodDataCardCard::new(
                    12,
                    2030,
                    "4242424242424242",
                ),
            ),
            json!({"exp_month": 12, "exp_year": 2030, "number": "4242424242424242"}),
        ),
        (
            CreateThreeDSecureAuthenticationPaymentMethodDataCard::CardToken(
                CreateThreeDSecureAuthenticationPaymentMethodDataCardToken::new("tok_test"),
            ),
            json!({"token": "tok_test"}),
        ),
    ] {
        let data = CreateThreeDSecureAuthenticationPaymentMethodData::new(
            card,
            CreateThreeDSecureAuthenticationPaymentMethodDataType::Card,
        );
        assert_eq!(serde_json::to_value(data).unwrap(), json!({"card": expected, "type": "card"}));
    }
}

#[test]
fn from_str_and_deser_behavior_match_on_unknown_variant() {
    let acct_authorized = "account.application.authorized";
    assert_eq!(
        EventType::AccountApplicationAuthorized,
        EventType::from_str(acct_authorized).unwrap()
    );
    assert_eq!(EventType::AccountApplicationAuthorized.as_str(), acct_authorized);

    assert_eq!(EventType::Unknown(String::from("acct")), EventType::from_str("acct").unwrap());
    assert_eq!(
        miniserde::json::from_str::<EventType>(r#""acct""#).unwrap(),
        EventType::Unknown(String::from("acct"))
    );
    assert_eq!(
        serde_json::from_str::<EventType>(r#""acct""#).unwrap(),
        EventType::Unknown(String::from("acct"))
    );
}
