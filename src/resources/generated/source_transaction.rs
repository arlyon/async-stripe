// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{ChargeId};
use crate::params::{Object, Timestamp};
use crate::resources::{Currency};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "SourceTransaction".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SourceTransaction {
    /// Unique identifier for the object.
    pub id: ChargeId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_credit_transfer: Option<SourceTransactionAchCreditTransferData>,

    /// A positive integer in the smallest currency unit (that is, 100 cents for $1.00, or 1 for ¥1, Japanese Yen being a zero-decimal currency) representing the amount your customer has pushed to the receiver.
    pub amount: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub chf_credit_transfer: Option<SourceTransactionChfCreditTransferData>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gbp_credit_transfer: Option<SourceTransactionGbpCreditTransferData>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub paper_check: Option<SourceTransactionPaperCheckData>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_credit_transfer: Option<SourceTransactionSepaCreditTransferData>,

    /// The ID of the source this transaction is attached to.
    pub source: String,

    /// The status of the transaction, one of `succeeded`, `pending`, or `failed`.
    pub status: String,

    /// The type of source this transaction is attached to.
    #[serde(rename = "type")]
    pub type_: SourceTransactionType,
}

impl Object for SourceTransaction {
    type Id = ChargeId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "source_transaction"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SourceTransactionAchCreditTransferData {

    /// Customer data associated with the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_data: Option<String>,

    /// Bank account fingerprint associated with the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,

    /// Last 4 digits of the account number associated with the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,

    /// Routing number associated with the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SourceTransactionChfCreditTransferData {

    /// Reference associated with the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,

    /// Sender's country address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_address_country: Option<String>,

    /// Sender's line 1 address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_address_line1: Option<String>,

    /// Sender's bank account IBAN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_iban: Option<String>,

    /// Sender's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_name: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SourceTransactionGbpCreditTransferData {

    /// Bank account fingerprint associated with the Stripe owned bank account receiving the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,

    /// The credit transfer rails the sender used to push this transfer.
    ///
    /// The possible rails are: Faster Payments, BACS, CHAPS, and wire transfers.
    /// Currently only Faster Payments is supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_method: Option<String>,

    /// Last 4 digits of sender account number associated with the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,

    /// Sender entered arbitrary information about the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,

    /// Sender account number associated with the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_account_number: Option<String>,

    /// Sender name associated with the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_name: Option<String>,

    /// Sender sort code associated with the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_sort_code: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SourceTransactionPaperCheckData {

    /// Time at which the deposited funds will be available for use.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_at: Option<String>,

    /// Comma-separated list of invoice IDs associated with the paper check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoices: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SourceTransactionSepaCreditTransferData {

    /// Reference associated with the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,

    /// Sender's bank account IBAN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_iban: Option<String>,

    /// Sender's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_name: Option<String>,
}

/// An enum representing the possible values of an `SourceTransaction`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SourceTransactionType {
    AchCreditTransfer,
    AchDebit,
    Alipay,
    Bancontact,
    Card,
    CardPresent,
    Eps,
    Giropay,
    Ideal,
    Klarna,
    Multibanco,
    P24,
    SepaDebit,
    Sofort,
    ThreeDSecure,
    Wechat,
}

impl SourceTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            SourceTransactionType::AchCreditTransfer => "ach_credit_transfer",
            SourceTransactionType::AchDebit => "ach_debit",
            SourceTransactionType::Alipay => "alipay",
            SourceTransactionType::Bancontact => "bancontact",
            SourceTransactionType::Card => "card",
            SourceTransactionType::CardPresent => "card_present",
            SourceTransactionType::Eps => "eps",
            SourceTransactionType::Giropay => "giropay",
            SourceTransactionType::Ideal => "ideal",
            SourceTransactionType::Klarna => "klarna",
            SourceTransactionType::Multibanco => "multibanco",
            SourceTransactionType::P24 => "p24",
            SourceTransactionType::SepaDebit => "sepa_debit",
            SourceTransactionType::Sofort => "sofort",
            SourceTransactionType::ThreeDSecure => "three_d_secure",
            SourceTransactionType::Wechat => "wechat",
        }
    }
}

impl AsRef<str> for SourceTransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SourceTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for SourceTransactionType {
    fn default() -> Self {
        Self::AchCreditTransfer
    }
}
