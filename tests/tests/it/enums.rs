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
    assert!(AccountType::from_str("unknown").is_err());
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
fn from_str_and_deser_behavior_match_on_unknown_variant() {
    let acct_authorized = "account.application.authorized";
    assert_eq!(
        EventType::AccountApplicationAuthorized,
        EventType::from_str(acct_authorized).unwrap()
    );
    assert_eq!(EventType::AccountApplicationAuthorized.as_str(), acct_authorized);

    assert_eq!(EventType::Unknown, EventType::from_str("acct").unwrap());
    assert_eq!(miniserde::json::from_str::<EventType>(r#""acct""#).unwrap(), EventType::Unknown);
    assert_eq!(serde_json::from_str::<EventType>(r#""acct""#).unwrap(), EventType::Unknown);
}
