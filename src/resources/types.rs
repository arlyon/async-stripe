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

/// An enum representing the possible values of an `Account`'s `type` field.
///
/// For more details see [https://stripe.com/docs/api/accounts/object#account_object-type](https://stripe.com/docs/api/accounts/object#account_object-type)
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AccountType {
    Standard,
    Express,
    Custom,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Address {
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
}

/// An enum representing the possible values of a `BalanceTransaction`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BalanceTransactionStatus {
    Available,
    Pending,
}

/// An enum representing the possible values of a `BankAccount`'s `status` field.
///
/// For more details see [https://stripe.com/docs/api/customer_bank_accounts/object#customer_bank_account_object-status](https://stripe.com/docs/api/customer_bank_accounts/object#customer_bank_account_object-status)
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BankAccountStatus {
    /// A bank account that hasn’t had any activity or validation performed is new.
    New,
    /// If Stripe can determine that the bank account exists, its status will be validated.
    ///
    /// Note that there often isn’t enough information to know (e.g., for smaller credit unions),
    /// and the validation is not always run.
    Validated,
    /// If customer bank account verification has succeeded, the bank account status will be verified.
    Verified,
    /// If the verification failed for any reason, such as microdeposit failure, the status will be verification_failed.
    VerificationFailed,
    /// If a transfer sent to this bank account fails, we’ll set the status to errored and will not continue to send transfers until the bank details are updated.
    Errored,
}

// TODO: Implement
/// This type is a stub that still needs to be implemented.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BillingDetails {}

/// An enum representing the possible values of an `Account`'s `business_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BusinessType {
    Individual,
    Business,
}

/// An enum representing the possible values of the `AccountCapabilities` fields.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityStatus {
    Standard,
    Express,
    Custom,
}

/// An enum representing the possible values of an `Fee`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FeeType {
    ApplicationFee,
    StripeFee,
    Tax,
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

// TODO: Implement
/// This type is a stub that still needs to be implemented.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LegalEntityJapanAddress {}

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
