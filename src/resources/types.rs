use crate::params::Timestamp;
use serde_derive::{Deserialize, Serialize};

/// An enum representing the possible values of a `BankAccount`'s `account_holder_type` field.
///
/// For more details see [https://stripe.com/docs/api/customer_bank_accounts/object#customer_bank_account_object-account_holder_type](https://stripe.com/docs/api/customer_bank_accounts/object#customer_bank_account_object-account_holder_type)
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountHolderType {
    Individual,
    Company,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Address {
    /// Address line 1 or block/building number (e.g. Street address/PO Box/Company name)
    pub line1: Option<String>,
    /// Address line 2 or building details (e.g. Apartment/Suite/Unit/Building)
    pub line2: Option<String>,
    /// City (or Ward)
    pub city: Option<String>,
    /// State (or Prefecture)
    pub state: Option<String>,
    /// ZIP or postal code
    pub postal_code: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))
    pub country: Option<String>,
    /// The town/cho-me (Japan only)
    pub town: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BillingDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CustomField {
    pub name: String,
    pub value: String,
}

// A date of birth
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Dob {
    pub day: i64,
    pub month: i64,
    pub year: i64,
}

/// An enum representing the possible values of an `Fee`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FeeType {
    ApplicationFee,
    StripeFee,
    Tax,
}

/// An enum representing the possible values of a `FraudDetails`'s `report` fields.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FraudDetailsReport {
    Fraudulent,
    Safe,
}

/// An enum representing the possible values of the `IssuingAuthorizationVerificationData` fields.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingAuthorizationCheck {
    Match,
    Mismatch,
    NotProvided,
}

/// An enum representing the possible values of the `IssuingAuthorization`'s `authorization_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingAuthorizationMethod {
    KeyedIn,
    Swipe,
    Chip,
    Contactless,
    Online,
}

/// An enum representing the possible values of the `IssuingAuthorizationRequest`'s `reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingAuthorizationReason {
    AuthorizationControls,
    CardActive,
    CardInactive,
    InsufficientFunds,
    AccountComplianceDisabled,
    AccountInactive,
    SuspectedFraud,
    WebhookApproved,
    WebhookDeclined,
    WebhookTimeout,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PackageDimensions {
    pub height: f64,
    pub length: f64,
    pub weight: f64,
    pub width: f64,
}

// TODO: Implement
/// This type is a stub that still needs to be implemented.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodDetails {}

/// Period is a structure representing a start and end dates.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Period {
    pub start: Timestamp,
    pub end: Timestamp,
}

// TODO: Implement
/// This type is a stub that still needs to be implemented.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Person {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Shipping {
    pub name: String,
    pub address: Address,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>, // eg. Fedex, UPS, USPS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ShippingParams {
    pub address: Address,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SpendingLimit {
    /// Maximum amount allowed to spend per time interval.
    pub amount: i64,

    /// Array of strings containing [categories](https://stripe.com/docs/api#issuing_authorization_object-merchant_data-category) on which to apply the spending limit.
    ///
    /// Leave this blank to limit all charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,

    /// The time interval with which to apply this spending limit towards.
    ///
    /// Allowed values are `per_authorization`, `daily`, `weekly`, `monthly`, `yearly`, or `all_time`.
    pub interval: SpendingLimitInterval,
}

/// An enum representing the possible values of an `SpendingLimit`'s `interval` field.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SpendingLimitInterval {
    AllTime,
    Daily,
    Monthly,
    PerAuthorization,
    Weekly,
    Yearly,
}

// TODO: Implement
/// This type is a stub that still needs to be implemented.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionBillingThresholds {}

// TODO: Implement
/// This type is a stub that still needs to be implemented.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionItemBillingThresholds {}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum DelayDays {
    Days(u32),
    Other(DelayDaysOther),
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DelayDaysOther {
    Minimum,
}

impl DelayDays {
    pub fn days(n: u32) -> Self {
        DelayDays::Days(n)
    }
    pub fn minimum() -> Self {
        DelayDays::Other(DelayDaysOther::Minimum)
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum Scheduled {
    Timestamp(Timestamp),
    Other(ScheduledOther),
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ScheduledOther {
    Now,
}

impl Scheduled {
    pub fn at(ts: Timestamp) -> Self {
        Scheduled::Timestamp(ts)
    }
    pub fn now() -> Self {
        Scheduled::Other(ScheduledOther::Now)
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum UpTo {
    Max(u64),
    Other(UpToOther),
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpToOther {
    Inf,
}

impl UpTo {
    pub fn max(n: u64) -> Self {
        UpTo::Max(n)
    }
    pub fn now() -> Self {
        UpTo::Other(UpToOther::Inf)
    }
}

/// A day of the week.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Weekday {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}
