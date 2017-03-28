extern crate serde_json as json;
extern crate stripe;

#[test]
fn debug_currency() {
    use stripe::Currency;
    assert_eq!(format!("{:?}", Currency::AED), "AED");
    assert_eq!(format!("{:?}", Currency::USD), "USD");
    assert_eq!(format!("{:?}", Currency::ZMW), "ZMW");
}

#[test]
fn display_currency() {
    use stripe::Currency;
    assert_eq!(format!("{}", Currency::AED), "aed");
    assert_eq!(format!("{}", Currency::USD), "usd");
    assert_eq!(format!("{}", Currency::ZMW), "zmw");
}

#[test]
fn serialize_currency() {
    use stripe::Currency;
    assert_eq!(json::to_string(&Currency::AED).unwrap(), "\"aed\"");
    assert_eq!(json::to_string(&Currency::USD).unwrap(), "\"usd\"");
    assert_eq!(json::to_string(&Currency::ZMW).unwrap(), "\"zmw\"");
}

#[test]
fn deserialize_currency() {
    use stripe::Currency;
    assert_eq!(json::from_str::<Currency>("\"aed\"").unwrap(), Currency::AED);
    assert_eq!(json::from_str::<Currency>("\"usd\"").unwrap(), Currency::USD);
    assert_eq!(json::from_str::<Currency>("\"zmw\"").unwrap(), Currency::ZMW);
}
