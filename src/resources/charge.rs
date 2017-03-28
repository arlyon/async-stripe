use client::Client;
use error::{Error, ErrorCode};
use params::{List, Metadata, Timestamp};
use resources::{Address, Currency, CustomerSource, Refund, Source};

#[derive(Debug, Deserialize)]
pub struct ChargeOutcome {
    #[serde(rename = "type")]
    pub outcome_type: String, // (authorized, manual_review, issuer_declined, blocked, invalid)
    pub network_status: String, // (approved_by_network, declined_by_network, not_sent_to_network, reversed_after_approval)
    #[serde(default)] pub reason: Option<String>,
    #[serde(default)] pub risk_level: String, // (normal, elevated, highest, not_assessed, unknown)
    #[serde(default)] pub seller_message: String,
    #[serde(default)] pub rule: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct FraudDetails {
    #[serde(skip_serializing_if = "Option::is_none")] pub user_report: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub stripe_report: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ShippingDetails {
    pub name: String,
    pub address: Address,
    #[serde(skip_serializing_if = "Option::is_none")] pub carrier: Option<String>, // eg. Fedex, UPS, USPS
    #[serde(skip_serializing_if = "Option::is_none")] pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub tracking_number: Option<String>,
}

#[derive(Default, Serialize)]
pub struct CaptureParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")] pub amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")] pub application_fee: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")] pub receipt_email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")] pub statement_descriptor: Option<&'a str>,
}

#[derive(Serialize)]
pub struct DestinationParams<'a> {
    pub account: &'a str,
    pub amount: u64,
}

#[derive(Default, Serialize)]
pub struct ChargeParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")] pub amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")] pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")] pub application_fee: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")] pub capture: Option<bool>, // NOTE: if None, Stripe assumes true
    #[serde(skip_serializing_if = "Option::is_none")] pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")] pub destination: Option<DestinationParams<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")] pub fraud_details: Option<FraudDetails>,
    #[serde(skip_serializing_if = "Option::is_none")] pub transfer_group: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")] pub on_behalf_of: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")] pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")] pub receipt_email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")] pub shipping: Option<ShippingDetails>,
    #[serde(skip_serializing_if = "Option::is_none")] pub source: Option<CustomerSource<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")] pub statement_descriptor: Option<&'a str>,
}

#[derive(Debug, Deserialize)]
pub struct Charge {
    pub id: String,
    pub amount: u64,
    pub amount_refunded: u64,
    pub application: Option<String>,
    pub application_fee: Option<String>,
    pub balance_transaction: String,
    pub captured: bool,
    pub created: Timestamp,
    pub currency: Currency,
    pub customer: Option<String>,
    pub description: Option<String>,
    pub destination: Option<String>,
    pub dispute: Option<String>,
    pub failure_code: Option<ErrorCode>,
    pub failure_message: Option<String>,
    pub fraud_details: FraudDetails,
    pub invoice: Option<String>,
    pub livemode: bool,
    pub metadata: Metadata,
    pub on_behalf_of: Option<String>,
    pub order: Option<String>,
    pub outcome: ChargeOutcome,
    pub paid: bool,
    pub receipt_email: Option<String>,
    pub receipt_number: Option<String>,
    pub refunded: bool,
    pub refunds: List<Refund>,
    pub shipping: Option<ShippingDetails>,
    pub source: Source,
    pub source_transfer: Option<String>,
    pub statement_descriptor: Option<String>,
    pub status: String, // (succeeded, pending, failed)
    pub transfer_group: Option<String>,
}

impl Charge {
    pub fn create(c: &Client, params: ChargeParams) -> Result<Charge, Error> {
        return c.post("/charges", params);
    }

    pub fn get(c: &Client, charge_id: &str) -> Result<Charge, Error> {
        return c.get(&format!("/charges/{}", charge_id));
    }

    pub fn update(c: &Client, charge_id: &str, params: ChargeParams) -> Result<Charge, Error> {
        return c.post(&format!("/charges/{}", charge_id), params);
    }

    pub fn capture(c: &Client, charge_id: &str, params: CaptureParams) -> Result<Charge, Error> {
        return c.post(&format!("/charges/{}/capture", charge_id), params);
    }
}
