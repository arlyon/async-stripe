use miniserde::json::from_str;
use serde_json::{Value, json};
use stripe_connect::Account;
use stripe_core::customer::RetrieveCustomerReturned;
use stripe_core::{
    Charge, ChargeStatus, Customer, CustomerTaxExempt, File, FileLink, FilePurpose, PaymentSource,
};
use stripe_types::{Currency, Timestamp};

fn mock_customer_with_card() -> Value {
    json!({
      "id": "cus_1234",
      "object": "customer",
      "balance": 0,
      "created": 1542631579,
      "currency": null,
      "default_source": "card_ABCD",
      "delinquent": false,
      "description": "Customer for green.tea@example.com",
      "discount": null,
      "email": null,
      "invoice_prefix": "99999AA",
      "livemode": false,
      "metadata": {},
      "shipping": null,
      "sources": {
        "object": "list",
        "data": [
          {
            "id": "card_ABCD",
            "object": "card",
            "address_city": null,
            "address_country": null,
            "address_line1": null,
            "address_line1_check": null,
            "address_line2": null,
            "address_state": null,
            "address_zip": null,
            "address_zip_check": null,
            "brand": "American Express",
            "country": "US",
            "customer": "cus_1234",
            "cvc_check": null,
            "dynamic_last4": null,
            "exp_month": 11,
            "exp_year": 2019,
            "fingerprint": "ffff9999ffff9999",
            "funding": "credit",
            "last4": "4242",
            "metadata": {},
            "name": null,
            "tokenization_method": null
          }
        ],
        "has_more": false,
        "total_count": 1,
        "url": "/v1/customers/cus_1234/sources"
      },
      "subscriptions": {
        "object": "list",
        "data": [],
        "has_more": false,
        "total_count": 0,
        "url": "/v1/customers/cus_1234/subscriptions"
      },
      "tax_info": null,
      "tax_info_verification": null
    })
}

#[test]
fn deserialize_customer_with_card() {
    let example = mock_customer_with_card().to_string();
    let result: Customer = from_str(&example).expect("deserialization failed");
    assert_eq!(result.balance, Some(0));
    assert!(!result.livemode);
}

#[test]
fn deserialize_customer_with_source() {
    let example = json!({
      "id": "cus_5678",
      "object": "customer",
      "account_balance": 0,
      "created": 1538150891,
      "currency": null,
      "default_source": "src_EFGH",
      "delinquent": false,
      "description": null,
      "discount": null,
      "email": null,
      "invoice_prefix": "00AA00AA",
      "livemode": false,
      "metadata": {},
      "shipping": null,
      "sources": {
        "object": "list",
        "data": [
          {
            "id": "src_EFGH",
            "object": "source",
            "amount": null,
            "card": {
              "exp_month": 9,
              "exp_year": 2019,
              "brand": "Visa",
              "country": "US",
              "fingerprint": "AAAA1111aaaa2222",
              "funding": "credit",
              "last4": "4242",
              "three_d_secure": "optional",
              "name": null,
              "address_line1_check": null,
              "address_zip_check": null,
              "cvc_check": null,
              "tokenization_method": null,
              "dynamic_last4": null
            },
            "client_secret": "src_client_secret_SUPERSECRETTECH",
            "created": 1538150891,
            "currency": null,
            "customer": "cus_5678",
            "flow": "none",
            "livemode": false,
            "metadata": {},
            "owner": {
              "address": null,
              "email": null,
              "name": null,
              "phone": null,
              "verified_address": null,
              "verified_email": null,
              "verified_name": null,
              "verified_phone": null
            },
            "statement_descriptor": "My Company",
            "status": "chargeable",
            "type": "card",
            "usage": "reusable"
          }
        ],
        "has_more": false,
        "total_count": 1,
        "url": "/v1/customers/cus_5678/sources"
      },
      "subscriptions": {
        "object": "list",
        "data": [],
        "has_more": false,
        "total_count": 0,
        "url": "/v1/customers/cus_5678/subscriptions"
      },
      "tax_info": null,
      "tax_info_verification": null
    })
    .to_string();

    let result: Customer = from_str(&example).unwrap();
    assert_eq!(result.description, None);
}

#[test]
fn deserialize_checkout_event() {
    use stripe_core::Event;

    let example = json!({
      "created": 1326853478,
      "livemode": false,
      "id": "evt_00000000000000",
      "type": "checkout.session.completed",
      "object": "event",
      "request": null,
      "pending_webhooks": 1,
      "api_version": "2019-05-16",
      "data": {
        "object": {
          "id": "cs_00000000000000",
          "object": "checkout.session",
          "billing_address_collection": null,
          "cancel_url": "https://example.com/cancel",
          "client_reference_id": null,
          "customer": null,
          "customer_email": null,
          "display_items": [
            {
              "amount": 1500,
              "currency": "usd",
              "custom": {
                "description": "Comfortable cotton t-shirt",
                "images": null,
                "name": "T-shirt"
              },
              "quantity": 2,
              "type": "custom"
            }
          ],
          "livemode": false,
          "locale": null,
          "mode": "payment",
          "payment_intent": "pi_00000000000000",
          "payment_method_types": [
            "card"
          ],
          "payment_status": "paid",
          "submit_type": null,
          "subscription": null,
          "success_url": "https://example.com/success"
        }
      }
    })
    .to_string();
    let result: Event = from_str(&example).unwrap();
    assert_eq!(result.pending_webhooks, 1);
}

#[test]
// https://github.com/arlyon/async-stripe/issues/456
// NB: deserialization test because `stripe_mock` always includes `refunds`
fn deserialize_charge_with_no_refunds() {
    let example = json!({
      "amount": 0,
      "billing_details": {},
      "amount_captured": 0,
      "amount_refunded": 0,
      "captured": true,
      "currency": "cad",
      "created": 1703349829,
      "disputed": false,
      "object": "charge",
      "id": "ch_123",
      "livemode": false,
      "metadata": {},
      "paid": false,
      "status": "pending",
      "refunded": false,
    })
    .to_string();
    let charge: Charge = from_str(&example).unwrap();
    assert_eq!(charge.id.as_str(), "ch_123");
    assert_eq!(charge.currency, Currency::CAD);
    assert_eq!(charge.status, ChargeStatus::Pending);
    assert_eq!(charge.created, 1703349829);
}

const FILE_CREATED: Timestamp = 1704511150;
fn mock_file() -> Value {
    json!({
      "created": FILE_CREATED,
      "id": "file_123",
      "size": 0,
      "object": "file",
      "purpose": "account_requirement"
    })
}

#[test]
fn deserialize_file() {
    let file = mock_file();
    let file: File = from_str(&file.to_string()).unwrap();
    assert_eq!(file.purpose, FilePurpose::AccountRequirement);
    assert_eq!(file.created, FILE_CREATED);
}

#[test]
fn deserialize_id() {
    use std::str::FromStr;

    use stripe_core::FileId;

    #[derive(miniserde::Deserialize)]
    struct Id {
        id: FileId,
    }
    let data = json!({"id": "file_123"});
    let id: Id = from_str(&data.to_string()).unwrap();
    assert_eq!(id.id, FileId::from_str("file_123").unwrap());
}

#[test]
fn deserialize_account_with_external_accounts() {
    let acct_url = "/v1/accounts/acct_123/external_accounts";
    let account = json!({
      "id": "acct_123",
      "object": "account",
      "external_accounts": {
        "data": [],
        "has_more": false,
        "object": "list",
        "url": acct_url
      },
    });
    let result: Account = from_str(&account.to_string()).unwrap();
    assert_eq!(result.id.as_str(), "acct_123");

    let external_accts = result.external_accounts.as_ref().unwrap();
    assert_eq!(external_accts.url, acct_url);
    assert!(external_accts.data.is_empty());
}

fn mock_payment_source() -> Value {
    json!({
      "object": "bank_account",
      "currency": "usd",
      "country": "us",
      "id": "ba_123",
      "status": "new",
      "last4": "1234",
    })
}

#[track_caller]
fn assert_payment_source_matches(result: &PaymentSource) {
    let PaymentSource::BankAccount(bank) = result else {
        panic!("Expected bank account");
    };
    assert_eq!(bank.last4, "1234");
    assert_eq!(bank.status, "new");
}

#[test]
fn deserialize_polymorphic() {
    let payment_source = mock_payment_source();
    let result =
        from_str(&payment_source.to_string()).expect("could not deserialize payment source");
    assert_payment_source_matches(&result);
}

#[test]
fn deserialize_expandable() {
    let file_link_base = json!({
      "created": 1704511150,
      "expired": false,
      "livemode": false,
      "object": "file_link",
      "metadata": {},
      "url": "http://localhost:3000",
      "id": "link_123",
    });

    let mut file_link = file_link_base.clone();
    file_link.as_object_mut().unwrap().insert("file".into(), Value::String("file_123".into()));

    let result: FileLink = from_str(&file_link.to_string()).unwrap();
    assert!(result.metadata.is_empty());
    assert!(!result.livemode);
    assert!(!result.expired);

    assert_eq!(result.url.as_ref().unwrap(), "http://localhost:3000");
    assert_eq!(result.created, 1704511150);
    assert_eq!(result.file.id().as_str(), "file_123");

    let mut file_link = file_link_base.clone();
    file_link.as_object_mut().unwrap().insert("file".into(), mock_file());

    let result: FileLink = from_str(&file_link.to_string()).unwrap();
    let file = result.file.as_object().unwrap();

    assert_eq!(file.created, FILE_CREATED);
    assert_eq!(file.purpose, FilePurpose::AccountRequirement);
    assert_eq!(file.id.as_str(), "file_123");
}

#[test]
fn deserialize_expandable_polymorphic() {
    let base_cust = json!({
        "currency": "usd",
        "created": 1704511150,
        "id": "cus_123",
        "livemode": false,
        "object": "customer",
        "tax_exempt": "exempt",
    });

    let mut cust = base_cust.clone();
    cust.as_object_mut().unwrap().insert("default_source".into(), Value::String("ba_123".into()));

    let result: Customer = from_str(&cust.to_string()).unwrap();
    assert_eq!(result.created, 1704511150);
    assert_eq!(result.currency, Some(Currency::USD));
    assert_eq!(result.tax_exempt, Some(CustomerTaxExempt::Exempt));
    assert_eq!(result.id.as_str(), "cus_123");
    assert_eq!(result.default_source.as_ref().unwrap().id().as_str(), "ba_123");

    let mut cust = base_cust.clone();
    cust.as_object_mut().unwrap().insert("default_source".into(), mock_payment_source());

    let result: Customer = from_str(&cust.to_string()).unwrap();
    let payment_source = result.default_source.as_ref().unwrap().as_object().unwrap();
    assert_payment_source_matches(payment_source);
}

#[test]
fn deserialize_customer_deleted_or_not() {
    let deleted_cust = json!({
        "deleted": true,
        "id": "cus_123",
        "object": "customer"
    });
    let result: RetrieveCustomerReturned = from_str(&deleted_cust.to_string()).unwrap();
    let RetrieveCustomerReturned::DeletedCustomer(deleted) = result else {
        panic!("expected deleted variant");
    };
    assert_eq!(deleted.id.as_str(), "cus_123");

    let cust = mock_customer_with_card().to_string();
    let result: RetrieveCustomerReturned = from_str(&cust).unwrap();
    let RetrieveCustomerReturned::Customer(cust) = result else {
        panic!("did not expected deleted variant")
    };
    assert_eq!(cust.id.as_str(), "cus_1234");
    assert_eq!(cust.sources.unwrap().data.len(), 1);
}
