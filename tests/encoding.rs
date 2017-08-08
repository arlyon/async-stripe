extern crate serde_json as json;
extern crate serde_qs as qs;
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

#[test]
fn serialize_range_query() {
    use stripe::{CustomerListParams, RangeQuery, RangeBounds};

    let query = RangeQuery::Bounds(RangeBounds {
        gt: None,
        gte: Some(1501598702),
        lt: Some(1504233902),
        lte: None,
    });
    assert_eq!(urldecode(qs::to_string(&query).unwrap()), "gte=1501598702&lt=1504233902");

    let mut params = CustomerListParams::default();
    params.created = Some(RangeQuery::eq(1501598702));
    params.limit = Some(3);
    assert_eq!(urldecode(qs::to_string(&params).unwrap()), "created=1501598702&limit=3");

    let mut params = CustomerListParams::default();
    params.created = Some(RangeQuery::gte(1501598702));
    params.limit = Some(3);
    assert_eq!(urldecode(qs::to_string(&params).unwrap()), "created[gte]=1501598702&limit=3");

    let mut params = CustomerListParams::default();
    params.created = Some(query);
    params.limit = Some(3);
    assert_eq!(urldecode(qs::to_string(&params).unwrap()), "created[gte]=1501598702&created[lt]=1504233902&limit=3");
}

fn urldecode(input: String) -> String {
    input.replace("%5B", "[").replace("%5D", "]")
}
