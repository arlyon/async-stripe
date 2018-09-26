#[macro_use] extern crate serde_json as json;
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
    let encoded = urldecode(qs::to_string(&params).unwrap());
    assert_eq!(encoded, "created[gte]=1501598702&created[lt]=1504233902&limit=3");
}

fn urldecode(input: String) -> String {
    input.replace("%5B", "[").replace("%5D", "]")
}


#[test]
fn deserialize_customer_source_params() {
    use stripe::{CardParams, CustomerSourceParams, SourceId, TokenId};

    let examples = [
        (json!("tok_189g322eZvKYlo2CeoPw2sdy"),
         Some(CustomerSourceParams::Token("tok_189g322eZvKYlo2CeoPw2sdy".parse::<TokenId>().unwrap()))),
        (json!("src_xyzABC123"),
         Some(CustomerSourceParams::Source("src_xyzABC123".parse::<SourceId>().unwrap()))),
        (json!({"object": "card", "exp_month": "12", "exp_year": "2017", "number": "1111222233334444"}),
         Some(CustomerSourceParams::Card(CardParams {
             exp_month: "12",
             exp_year: "2017",
             number: "1111222233334444",
             name: None,
             cvc: None,
         })),

         // Error: Missing `{"object": "card"}`
        (json!({"exp_month": "12", "exp_year": "2017", "number": "1111222233334444"}), None),
    ];

    for (value, expected) in &examples {
        let input = json::to_string(value).unwrap();
        assert_eq!(json::from_str(&input).ok(), expected);
    }
}
