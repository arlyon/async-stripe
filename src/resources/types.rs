use crate::params::Timestamp;
use crate::resources::card::{CardBrand, CardType};
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

/// A date of birth.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Dob {
    pub day: i64,
    pub month: i64,
    pub year: i64,
}

/// An enum representing the possible values of a `FraudDetails`'s `report` fields.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FraudDetailsReport {
    Fraudulent,
    Safe,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PackageDimensions {
    pub height: f64,
    pub length: f64,
    pub weight: f64,
    pub width: f64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodCard {
    /// Can be `American Express`, `Diners Club`, `Discover`, `JCB`, `MasterCard`, `UnionPay`, `Visa`, or `Unknown`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<CardBrand>,

    /// Two-letter ISO code representing the country of the card.
    ///
    /// You could use this attribute to get a sense of the international breakdown of cards you've collected.
    pub country: String,

    /// Two-digit number representing the card's expiration month.
    pub exp_month: i64,

    /// Four-digit number representing the card's expiration year.
    pub exp_year: i64,

    /// Uniquely identifies this particular card number.
    ///
    /// You can use this attribute to check whether two customers who've signed up with you are using the same card number, for example.
    pub fingerprint: String,

    /// Card funding type.
    ///
    /// Can be `credit`, `debit`, `prepaid`, or `unknown`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding: Option<CardType>,

    /// The last four digits of the card.
    pub last4: String,
}

// TODO: Implement
/// This type is a stub that still needs to be implemented.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<PaymentMethodCard>,
}

/// Period is a structure representing a start and end dates.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Period {
    pub start: Timestamp,
    pub end: Timestamp,
}

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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionBillingThresholds {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_gte: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_billing_cycle_anchor: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionItemBillingThresholds {
    pub usage_gte: i64,
}

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

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum PaymentIntentOffSession {
    Exists(bool),
    Other(OffSessionOther),
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OffSessionOther {
    #[serde(rename = "one_off")]
    OneOff,
    #[serde(rename = "recurring")]
    Recurring,
}

impl PaymentIntentOffSession {
    pub fn exists(n: bool) -> Self {
        PaymentIntentOffSession::Exists(n)
    }
    pub fn frequency(n: OffSessionOther) -> Self {
        match n {
            OffSessionOther::OneOff => PaymentIntentOffSession::Other(OffSessionOther::OneOff),
            OffSessionOther::Recurring => {
                PaymentIntentOffSession::Other(OffSessionOther::Recurring)
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SetupIntentUsage {
    #[serde(rename = "on_session")]
    OnSession,
    #[serde(rename = "off_session")]
    OffSession,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BusinessType {
    Individual,
    Company,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ApiErrors {
    #[serde(rename = "api_connection_error")]
    ApiConnectionError,
    #[serde(rename = "api_error")]
    ApiError,
    #[serde(rename = "authentication_error")]
    AuthenticationError,
    #[serde(rename = "card_error")]
    CardError,
    #[serde(rename = "idempotency_error")]
    IdempotencyError,
    #[serde(rename = "invalid_request_error")]
    InvalidRequestError,
    #[serde(rename = "rate_limit_error")]
    RateLimitError,
}
