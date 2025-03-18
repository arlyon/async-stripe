// ======================================
// This file was automatically generated.
// ======================================

use crate::client::{Client, Response};
use crate::ids::MandateId;
use crate::params::{Expand, Expandable, Object, Timestamp};
use crate::resources::{Currency, PaymentMethod};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "Mandate".
///
/// For more details see <https://stripe.com/docs/api/mandates/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Mandate {
    /// Unique identifier for the object.
    pub id: MandateId,

    pub customer_acceptance: CustomerAcceptance,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_use: Option<MandateMultiUse>,

    /// The account (if any) that the mandate is intended for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,

    /// ID of the payment method associated with this mandate.
    pub payment_method: Expandable<PaymentMethod>,

    pub payment_method_details: MandatePaymentMethodDetails,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_use: Option<MandateSingleUse>,

    /// The mandate status indicates whether or not you can use it to initiate a payment.
    pub status: MandateStatus,

    /// The type of the mandate.
    #[serde(rename = "type")]
    pub type_: MandateType,
}

impl Mandate {
    /// Retrieves a Mandate object.
    pub fn retrieve(client: &Client, id: &MandateId, expand: &[&str]) -> Response<Mandate> {
        client.get_query(&format!("/mandates/{}", id), Expand { expand })
    }
}

impl Object for Mandate {
    type Id = MandateId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "mandate"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CustomerAcceptance {
    /// The time that the customer accepts the mandate.
    pub accepted_at: Option<Timestamp>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<OfflineAcceptance>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub online: Option<OnlineAcceptance>,

    /// The mandate includes the type of customer acceptance information, such as: `online` or `offline`.
    #[serde(rename = "type")]
    pub type_: CustomerAcceptanceType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MandateMultiUse {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MandatePaymentMethodDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<MandateAcssDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<MandateAuBecsDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<MandateBacsDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CardMandatePaymentMethodDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<MandateCashapp>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<MandateLink>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<MandatePaypal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<MandateSepaDebit>,

    /// This mandate corresponds with a specific payment method type.
    ///
    /// The `payment_method_details` includes an additional hash with the same name and contains mandate information that's specific to that payment method.
    #[serde(rename = "type")]
    pub type_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<MandateUsBankAccount>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CardMandatePaymentMethodDetails {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MandateAcssDebit {
    /// List of Stripe products where this mandate can be selected automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_for: Option<Vec<MandateAcssDebitDefaultFor>>,

    /// Description of the interval.
    ///
    /// Only required if the 'payment_schedule' parameter is 'interval' or 'combined'.
    pub interval_description: Option<String>,

    /// Payment schedule for the mandate.
    pub payment_schedule: MandateAcssDebitPaymentSchedule,

    /// Transaction type of the mandate.
    pub transaction_type: MandateAcssDebitTransactionType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MandateAuBecsDebit {
    /// The URL of the mandate.
    ///
    /// This URL generally contains sensitive information about the customer and should be shared with them exclusively.
    pub url: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MandateBacsDebit {
    /// The status of the mandate on the Bacs network.
    ///
    /// Can be one of `pending`, `revoked`, `refused`, or `accepted`.
    pub network_status: MandateBacsDebitNetworkStatus,

    /// The unique reference identifying the mandate on the Bacs network.
    pub reference: String,

    /// When the mandate is revoked on the Bacs network this field displays the reason for the revocation.
    pub revocation_reason: Option<MandateBacsDebitRevocationReason>,

    /// The URL that will contain the mandate that the customer has signed.
    pub url: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MandateCashapp {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MandateLink {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MandatePaypal {
    /// The PayPal Billing Agreement ID (BAID).
    ///
    /// This is an ID generated by PayPal which represents the mandate between the merchant and the customer.
    pub billing_agreement_id: Option<String>,

    /// PayPal account PayerID.
    ///
    /// This identifier uniquely identifies the PayPal customer.
    pub payer_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MandateSepaDebit {
    /// The unique reference of the mandate.
    pub reference: String,

    /// The URL of the mandate.
    ///
    /// This URL generally contains sensitive information about the customer and should be shared with them exclusively.
    pub url: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MandateSingleUse {
    /// The amount of the payment on a single use mandate.
    pub amount: i64,

    /// The currency of the payment on a single use mandate.
    pub currency: Currency,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MandateUsBankAccount {
    /// Mandate collection method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<MandateUsBankAccountCollectionMethod>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OfflineAcceptance {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OnlineAcceptance {
    /// The customer accepts the mandate from this IP address.
    pub ip_address: Option<String>,

    /// The customer accepts the mandate using the user agent of the browser.
    pub user_agent: Option<String>,
}

/// An enum representing the possible values of an `CustomerAcceptance`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CustomerAcceptanceType {
    Offline,
    Online,
}

impl CustomerAcceptanceType {
    pub fn as_str(self) -> &'static str {
        match self {
            CustomerAcceptanceType::Offline => "offline",
            CustomerAcceptanceType::Online => "online",
        }
    }
}

impl AsRef<str> for CustomerAcceptanceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CustomerAcceptanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CustomerAcceptanceType {
    fn default() -> Self {
        Self::Offline
    }
}

/// An enum representing the possible values of an `MandateAcssDebit`'s `default_for` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MandateAcssDebitDefaultFor {
    Invoice,
    Subscription,
}

impl MandateAcssDebitDefaultFor {
    pub fn as_str(self) -> &'static str {
        match self {
            MandateAcssDebitDefaultFor::Invoice => "invoice",
            MandateAcssDebitDefaultFor::Subscription => "subscription",
        }
    }
}

impl AsRef<str> for MandateAcssDebitDefaultFor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for MandateAcssDebitDefaultFor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for MandateAcssDebitDefaultFor {
    fn default() -> Self {
        Self::Invoice
    }
}

/// An enum representing the possible values of an `MandateAcssDebit`'s `payment_schedule` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MandateAcssDebitPaymentSchedule {
    Combined,
    Interval,
    Sporadic,
}

impl MandateAcssDebitPaymentSchedule {
    pub fn as_str(self) -> &'static str {
        match self {
            MandateAcssDebitPaymentSchedule::Combined => "combined",
            MandateAcssDebitPaymentSchedule::Interval => "interval",
            MandateAcssDebitPaymentSchedule::Sporadic => "sporadic",
        }
    }
}

impl AsRef<str> for MandateAcssDebitPaymentSchedule {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for MandateAcssDebitPaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for MandateAcssDebitPaymentSchedule {
    fn default() -> Self {
        Self::Combined
    }
}

/// An enum representing the possible values of an `MandateAcssDebit`'s `transaction_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MandateAcssDebitTransactionType {
    Business,
    Personal,
}

impl MandateAcssDebitTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            MandateAcssDebitTransactionType::Business => "business",
            MandateAcssDebitTransactionType::Personal => "personal",
        }
    }
}

impl AsRef<str> for MandateAcssDebitTransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for MandateAcssDebitTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for MandateAcssDebitTransactionType {
    fn default() -> Self {
        Self::Business
    }
}

/// An enum representing the possible values of an `MandateBacsDebit`'s `network_status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MandateBacsDebitNetworkStatus {
    Accepted,
    Pending,
    Refused,
    Revoked,
}

impl MandateBacsDebitNetworkStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            MandateBacsDebitNetworkStatus::Accepted => "accepted",
            MandateBacsDebitNetworkStatus::Pending => "pending",
            MandateBacsDebitNetworkStatus::Refused => "refused",
            MandateBacsDebitNetworkStatus::Revoked => "revoked",
        }
    }
}

impl AsRef<str> for MandateBacsDebitNetworkStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for MandateBacsDebitNetworkStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for MandateBacsDebitNetworkStatus {
    fn default() -> Self {
        Self::Accepted
    }
}

/// An enum representing the possible values of an `MandateBacsDebit`'s `revocation_reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MandateBacsDebitRevocationReason {
    AccountClosed,
    BankAccountRestricted,
    BankOwnershipChanged,
    CouldNotProcess,
    DebitNotAuthorized,
}

impl MandateBacsDebitRevocationReason {
    pub fn as_str(self) -> &'static str {
        match self {
            MandateBacsDebitRevocationReason::AccountClosed => "account_closed",
            MandateBacsDebitRevocationReason::BankAccountRestricted => "bank_account_restricted",
            MandateBacsDebitRevocationReason::BankOwnershipChanged => "bank_ownership_changed",
            MandateBacsDebitRevocationReason::CouldNotProcess => "could_not_process",
            MandateBacsDebitRevocationReason::DebitNotAuthorized => "debit_not_authorized",
        }
    }
}

impl AsRef<str> for MandateBacsDebitRevocationReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for MandateBacsDebitRevocationReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for MandateBacsDebitRevocationReason {
    fn default() -> Self {
        Self::AccountClosed
    }
}

/// An enum representing the possible values of an `Mandate`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MandateStatus {
    Active,
    Inactive,
    Pending,
}

impl MandateStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            MandateStatus::Active => "active",
            MandateStatus::Inactive => "inactive",
            MandateStatus::Pending => "pending",
        }
    }
}

impl AsRef<str> for MandateStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for MandateStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for MandateStatus {
    fn default() -> Self {
        Self::Active
    }
}

/// An enum representing the possible values of an `Mandate`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MandateType {
    MultiUse,
    SingleUse,
}

impl MandateType {
    pub fn as_str(self) -> &'static str {
        match self {
            MandateType::MultiUse => "multi_use",
            MandateType::SingleUse => "single_use",
        }
    }
}

impl AsRef<str> for MandateType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for MandateType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for MandateType {
    fn default() -> Self {
        Self::MultiUse
    }
}

/// An enum representing the possible values of an `MandateUsBankAccount`'s `collection_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MandateUsBankAccountCollectionMethod {
    Paper,
}

impl MandateUsBankAccountCollectionMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            MandateUsBankAccountCollectionMethod::Paper => "paper",
        }
    }
}

impl AsRef<str> for MandateUsBankAccountCollectionMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for MandateUsBankAccountCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for MandateUsBankAccountCollectionMethod {
    fn default() -> Self {
        Self::Paper
    }
}
