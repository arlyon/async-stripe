// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::config::{Client, Response};
use crate::ids::MandateId;
use crate::params::{Expand, Expandable, Object, Timestamp};
use crate::resources::{Currency, PaymentMethod};

/// The resource representing a Stripe "Mandate".
///
/// For more details see <https://stripe.com/docs/api/mandates/object>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Mandate {
    /// Unique identifier for the object.
    pub id: MandateId,

    pub customer_acceptance: CustomerAcceptance,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_use: Option<MandateMultiUse>,

    /// ID of the payment method associated with this mandate.
    pub payment_method: Expandable<PaymentMethod>,

    pub payment_method_details: MandatePaymentMethodDetails,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_use: Option<MandateSingleUse>,

    /// The status of the mandate, which indicates whether it can be used to initiate a payment.
    pub status: MandateStatus,

    /// The type of the mandate.
    #[serde(rename = "type")]
    pub type_: MandateType,
}

impl Mandate {
    /// Retrieves a Mandate object.
    pub fn retrieve(client: &Client, id: &MandateId, expand: &[&str]) -> Response<Mandate> {
        client.get_query(&format!("/mandates/{}", id), &Expand { expand })
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CustomerAcceptance {
    /// The time at which the customer accepted the Mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_at: Option<Timestamp>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<OfflineAcceptance>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub online: Option<OnlineAcceptance>,

    /// The type of customer acceptance information included with the Mandate.
    ///
    /// One of `online` or `offline`.
    #[serde(rename = "type")]
    pub type_: CustomerAcceptanceType,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MandateMultiUse {}

#[derive(Clone, Debug, Deserialize, Serialize)]
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
    pub sepa_debit: Option<MandateSepaDebit>,

    /// The type of the payment method associated with this mandate.
    ///
    /// An additional hash is included on `payment_method_details` with a name matching this value.
    /// It contains mandate information specific to the payment method.
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CardMandatePaymentMethodDetails {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MandateAcssDebit {
    /// List of Stripe products where this mandate can be selected automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_for: Option<Vec<MandateAcssDebitDefaultFor>>,

    /// Description of the interval.
    ///
    /// Only required if the 'payment_schedule' parameter is 'interval' or 'combined'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_description: Option<String>,

    /// Payment schedule for the mandate.
    pub payment_schedule: MandateAcssDebitPaymentSchedule,

    /// Transaction type of the mandate.
    pub transaction_type: MandateAcssDebitTransactionType,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MandateAuBecsDebit {
    /// The URL of the mandate.
    ///
    /// This URL generally contains sensitive information about the customer and should be shared with them exclusively.
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MandateBacsDebit {
    /// The status of the mandate on the Bacs network.
    ///
    /// Can be one of `pending`, `revoked`, `refused`, or `accepted`.
    pub network_status: MandateBacsDebitNetworkStatus,

    /// The unique reference identifying the mandate on the Bacs network.
    pub reference: String,

    /// The URL that will contain the mandate that the customer has signed.
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MandateSepaDebit {
    /// The unique reference of the mandate.
    pub reference: String,

    /// The URL of the mandate.
    ///
    /// This URL generally contains sensitive information about the customer and should be shared with them exclusively.
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MandateSingleUse {
    /// On a single use mandate, the amount of the payment.
    pub amount: i64,

    /// On a single use mandate, the currency of the payment.
    pub currency: Currency,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OfflineAcceptance {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OnlineAcceptance {
    /// The IP address from which the Mandate was accepted by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,

    /// The user agent of the browser from which the Mandate was accepted by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
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
