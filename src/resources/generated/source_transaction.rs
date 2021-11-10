// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::ids::ChargeId;
use crate::params::{Object, Timestamp};
use crate::resources::Currency;

/// The resource representing a Stripe "SourceTransaction".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceTransaction {
    /// Unique identifier for the object.
    pub id: ChargeId,

    pub ach_credit_transfer: Box<Option<SourceTransactionAchCreditTransferData>>,

    /// A positive integer in the smallest currency unit (that is, 100 cents for $1.00, or 1 for ¥1, Japanese Yen being a zero-decimal currency) representing the amount your customer has pushed to the receiver.
    pub amount: i64,

    pub chf_credit_transfer: Box<Option<SourceTransactionChfCreditTransferData>>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    pub gbp_credit_transfer: Box<Option<SourceTransactionGbpCreditTransferData>>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    pub paper_check: Box<Option<SourceTransactionPaperCheckData>>,

    pub sepa_credit_transfer: Box<Option<SourceTransactionSepaCreditTransferData>>,

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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceTransactionAchCreditTransferData {
    /// Customer data associated with the transfer.
    pub customer_data: Box<Option<String>>,

    /// Bank account fingerprint associated with the transfer.
    pub fingerprint: Box<Option<String>>,

    /// Last 4 digits of the account number associated with the transfer.
    pub last4: Box<Option<String>>,

    /// Routing number associated with the transfer.
    pub routing_number: Box<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceTransactionChfCreditTransferData {
    /// Reference associated with the transfer.
    pub reference: Box<Option<String>>,

    /// Sender's country address.
    pub sender_address_country: Box<Option<String>>,

    /// Sender's line 1 address.
    pub sender_address_line1: Box<Option<String>>,

    /// Sender's bank account IBAN.
    pub sender_iban: Box<Option<String>>,

    /// Sender's name.
    pub sender_name: Box<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceTransactionGbpCreditTransferData {
    /// Bank account fingerprint associated with the Stripe owned bank account receiving the transfer.
    pub fingerprint: Box<Option<String>>,

    /// The credit transfer rails the sender used to push this transfer.
    ///
    /// The possible rails are: Faster Payments, BACS, CHAPS, and wire transfers.
    /// Currently only Faster Payments is supported.
    pub funding_method: Box<Option<String>>,

    /// Last 4 digits of sender account number associated with the transfer.
    pub last4: Box<Option<String>>,

    /// Sender entered arbitrary information about the transfer.
    pub reference: Box<Option<String>>,

    /// Sender account number associated with the transfer.
    pub sender_account_number: Box<Option<String>>,

    /// Sender name associated with the transfer.
    pub sender_name: Box<Option<String>>,

    /// Sender sort code associated with the transfer.
    pub sender_sort_code: Box<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceTransactionPaperCheckData {
    /// Time at which the deposited funds will be available for use.
    ///
    /// Measured in seconds since the Unix epoch.
    pub available_at: Box<Option<String>>,

    /// Comma-separated list of invoice IDs associated with the paper check.
    pub invoices: Box<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceTransactionSepaCreditTransferData {
    /// Reference associated with the transfer.
    pub reference: Box<Option<String>>,

    /// Sender's bank account IBAN.
    pub sender_iban: Box<Option<String>>,

    /// Sender's name.
    pub sender_name: Box<Option<String>>,
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
