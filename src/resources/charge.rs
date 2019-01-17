use crate::config::{Client, Response};
use crate::error::ErrorCode;
use crate::params::{Identifiable, List, Metadata, RangeQuery, Timestamp};
use crate::resources::{Address, Currency, PaymentSource, PaymentSourceParams, Refund};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe charge object outcome.
///
/// For more details see [https://stripe.com/docs/api#charge_object-outcome](https://stripe.com/docs/api#charge_object-outcome)
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChargeOutcome {
    #[serde(rename = "type")]
    pub outcome_type: OutcomeType,
    pub network_status: NetworkStatus,
    #[serde(default)]
    pub reason: Option<OutcomeReason>,
    #[serde(default)]
    pub risk_level: Option<RiskLevel>,
    #[serde(default)]
    pub seller_message: Option<String>,
    #[serde(default)]
    pub rule: Option<String>,
}

/// An enum representing the possible values of a `ChargeOutcome`'s `type` field.
///
/// For more details see [https://stripe.com/docs/api#charge_object-outcome-type](https://stripe.com/docs/api#charge_object-outcome-type)
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Copy, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OutcomeType {
    Authorized,
    ManualReview,
    IssuerDeclined,
    Blocked,
    Invalid,

    /// A variant not yet supported by the library.
    /// It is an error to send `Other` as part of a request.
    #[serde(other, skip_serializing)]
    Other,
}

/// An enum representing the possible values of a `ChargeOutcome`'s `network_status` field.
///
/// For more details see [https://stripe.com/docs/api#charge_object-outcome-network_status](https://stripe.com/docs/api#charge_object-outcome-network_status)
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Eq)]
#[serde(rename_all = "snake_case")]
pub enum NetworkStatus {
    ApprovedByNetwork,
    DeclinedByNetwork,
    NotSentToNetwork,

    /// This value indiciates the payment was [blocked by Stripe](https://stripe.com/docs/declines#blocked-payments)
    /// after bank authorization, and may temporarily appear as “pending” on a cardholder’s statement.
    ReversedAfterApproval,

    /// A variant not yet supported by the library.
    /// It is an error to send `Other` as part of a request.
    #[serde(other, skip_serializing)]
    Other,
}

/// An enum representing the possible values of a `ChargeOutcome`'s `risk_level` field.
///
/// For more details see [https://stripe.com/docs/api#charge_object-outcome-risk_level](https://stripe.com/docs/api#charge_object-outcome-risk_level)
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RiskLevel {
    Normal,
    Elevated,
    Highest,
    NotAssessed,

    /// An unknown risk level.
    ///
    /// May also be a variant not yet supported by the library.
    #[serde(other)]
    #[serde(rename = "unknown")]
    Unknown,
}

/// An enum representing the possible values of a `ChargeOutcome`'s `reason` field.
///
/// For more details see [https://stripe.com/docs/api#charge_object-outcome-reason](https://stripe.com/docs/api#charge_object-outcome-reason)
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OutcomeReason {
    ApprovedWithId,
    CallIssuer,
    CardNotSupported,
    CardVelocityExceeded,
    CurrencyNotSupported,
    DoNotHonor,
    DoNotTryAgain,
    DuplicateTransaction,
    ExpiredCard,
    Fraudulent,
    GenericDecline,
    IncorrectNumber,
    IncorrectCvc,
    IncorrectPin,
    IncorrectZip,
    InsufficientFunds,
    InvalidAccount,
    InvalidAmount,
    InvalidCvc,
    InvalidExpiryYear,
    InvalidNumber,
    InvalidPin,
    IssuerNotAvailable,
    LostCard,
    MerchantBlacklist,
    NewAccountInformationAvailable,
    NoActionTaken,
    NotPermitted,
    PickupCard,
    PinTryExceeded,
    ProcessingError,
    ReenterTransaction,
    RestrictedCard,
    RevocationOfAllAuthorizations,
    RevocationOfAuthorization,
    SecurityViolation,
    ServiceNotAllowed,
    StolenCard,
    StopPaymentOrder,
    TestmodeDecline,
    TransactionNotAllowed,
    TryAgainLater,
    WithdrawalCountLimitExceeded,

    /// A variant not yet supported by the library.
    /// It is an error to send `Other` as part of a request.
    #[serde(other, skip_serializing)]
    Other,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct FraudDetails {
    pub user_report: Option<String>,
    #[serde(skip_serializing)]
    pub stripe_report: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ShippingDetails {
    pub name: String,
    pub address: Address,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>, // eg. Fedex, UPS, USPS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<String>,
}

/// The set of parameters that can be used when capturing a charge object.
///
/// For more details see (https://stripe.com/docs/api#charge_capture](https://stripe.com/docs/api#charge_capture).
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CaptureParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DestinationParams<'a> {
    pub account: &'a str,
    pub amount: u64,
}

/// The set of parameters that can be used when creating or updating a charge object.
///
/// For more details see [https://stripe.com/docs/api#create_charge](https://stripe.com/docs/api#create_charge)
/// and [https://stripe.com/docs/api#update_charge](https://stripe.com/docs/api#update_charge).
#[derive(Clone, Debug, Default, Serialize)]
pub struct ChargeParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture: Option<bool>, // NOTE: if None, Stripe assumes true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<DestinationParams<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fraud_details: Option<FraudDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<ShippingDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<PaymentSourceParams<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum SourceFilterType {
    All,
    AlipayAccount,
    BankAccount,
    BitcoinReceiver,
    Card,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceFilter {
    pub object: SourceFilterType,
}

impl SourceFilter {
    pub fn all() -> SourceFilter {
        SourceFilter { object: SourceFilterType::All }
    }
    pub fn alipay() -> SourceFilter {
        SourceFilter { object: SourceFilterType::AlipayAccount }
    }
    pub fn bank() -> SourceFilter {
        SourceFilter { object: SourceFilterType::BankAccount }
    }
    pub fn bitcoin() -> SourceFilter {
        SourceFilter { object: SourceFilterType::BitcoinReceiver }
    }
    pub fn card() -> SourceFilter {
        SourceFilter { object: SourceFilterType::Card }
    }
}

/// The set of parameters that can be used when listing charges.
///
/// For more details see [https://stripe.com/docs/api#list_charges](https://stripe.com/docs/api#list_charges)
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ChargeListParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<SourceFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<&'a str>,
}

/// The resource representing a Stripe charge object.
///
/// For more details see [https://stripe.com/docs/api#charges](https://stripe.com/docs/api#charges).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Charge {
    pub id: String,
    pub amount: u64,
    pub amount_refunded: u64,
    pub application: Option<String>,
    pub application_fee: Option<String>,
    pub balance_transaction: Option<String>,
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
    pub outcome: Option<ChargeOutcome>,
    pub paid: bool,
    pub payment_intent: Option<String>,
    pub receipt_email: Option<String>,
    pub receipt_number: Option<String>,
    pub refunded: bool,
    pub refunds: List<Refund>,
    pub review: Option<String>,
    pub shipping: Option<ShippingDetails>,
    pub source: PaymentSource,
    pub source_transfer: Option<String>,
    pub statement_descriptor: Option<String>,
    pub status: ChargeStatus,
    pub transfer_group: Option<String>,
}

impl Identifiable for Charge {
    fn id(&self) -> &str {
        &self.id
    }
}

/// The resource representing a Stripe charge object status.
///
/// For more details see [https://stripe.com/docs/api#charge_object-status](https://stripe.com/docs/api#charge_object-status)
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Eq)]
pub enum ChargeStatus {
    #[serde(rename = "succeeded")]
    Succeeded,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "failed")]
    Failed,
}

impl Charge {
    /// Creates a new charge.
    ///
    /// For more details see [https://stripe.com/docs/api#create_charge](https://stripe.com/docs/api#create_charge).
    pub fn create(client: &Client, params: ChargeParams<'_>) -> Response<Charge> {
        client.post_form("/charges", params)
    }

    /// Retrieves the details of a charge.
    ///
    /// For more details see [https://stripe.com/docs/api#retrieve_charge](https://stripe.com/docs/api#retrieve_charge).
    pub fn retrieve(client: &Client, charge_id: &str) -> Response<Charge> {
        client.get(&format!("/charges/{}", charge_id))
    }

    /// Updates a charge's properties.
    ///
    /// For more details see [https://stripe.com/docs/api#update_charge](https://stripe.com/docs/api#update_charge).
    pub fn update(client: &Client, charge_id: &str, params: ChargeParams<'_>) -> Response<Charge> {
        client.post_form(&format!("/charges/{}", charge_id), params)
    }

    /// Capture captures a previously created charge with capture set to false.
    ///
    /// For more details see [https://stripe.com/docs/api#charge_capture](https://stripe.com/docs/api#charge_capture).
    pub fn capture(
        client: &Client,
        charge_id: &str,
        params: CaptureParams<'_>,
    ) -> Response<Charge> {
        client.post_form(&format!("/charges/{}/capture", charge_id), params)
    }

    /// List all charges.
    ///
    /// For more details see [https://stripe.com/docs/api#list_charges](https://stripe.com/docs/api#list_charges).
    pub fn list(client: &Client, params: ChargeListParams<'_>) -> Response<Vec<Charge>> {
        client.get_query("/charges", &params)
    }
}
