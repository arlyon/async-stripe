extern crate serde_json as json;
extern crate stripe;

#[test]
fn debug() {
    assert_eq!(format!("{:?}", stripe::Currency::AED), "AED");
    assert_eq!(format!("{:?}", stripe::Currency::USD), "USD");
    assert_eq!(format!("{:?}", stripe::Currency::ZMW), "ZMW");
}

#[test]
fn display() {
    assert_eq!(format!("{}", stripe::Currency::AED), "aed");
    assert_eq!(format!("{}", stripe::Currency::USD), "usd");
    assert_eq!(format!("{}", stripe::Currency::ZMW), "zmw");
}

#[test]
fn serialize() {
    assert_eq!(json::to_string(&stripe::Currency::AED).unwrap(), "\"aed\"");
    assert_eq!(json::to_string(&stripe::Currency::USD).unwrap(), "\"usd\"");
    assert_eq!(json::to_string(&stripe::Currency::ZMW).unwrap(), "\"zmw\"");
}

#[test]
fn deserialize() {
    assert_eq!(json::from_str::<stripe::Currency>("\"aed\"").unwrap(), stripe::Currency::AED);
    assert_eq!(json::from_str::<stripe::Currency>("\"usd\"").unwrap(), stripe::Currency::USD);
    assert_eq!(json::from_str::<stripe::Currency>("\"zmw\"").unwrap(), stripe::Currency::ZMW);
}
