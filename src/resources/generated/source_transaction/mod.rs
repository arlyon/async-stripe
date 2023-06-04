/// Some payment methods have no required amount that a customer must send.
/// Customers can be instructed to send any amount, and it can be made up of
/// multiple transactions.
///
/// As such, sources can have multiple associated transactions.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SourceTransaction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_credit_transfer:
        Option<crate::source_transaction::ach_credit_transfer_data::AchCreditTransferData>,
    /// A positive integer in the smallest currency unit (that is, 100 cents for $1.00, or 1 for ¥1, Japanese Yen being a zero-decimal currency) representing the amount your customer has pushed to the receiver.
    pub amount: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chf_credit_transfer:
        Option<crate::source_transaction::chf_credit_transfer_data::ChfCreditTransferData>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::Currency,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gbp_credit_transfer:
        Option<crate::source_transaction::gbp_credit_transfer_data::GbpCreditTransferData>,
    /// Unique identifier for the object.
    pub id: crate::source_transaction::SourceTransactionId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: SourceTransactionObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paper_check: Option<crate::source_transaction::paper_check_data::PaperCheckData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_credit_transfer:
        Option<crate::source_transaction::sepa_credit_transfer_data::SepaCreditTransferData>,
    /// The ID of the source this transaction is attached to.
    pub source: String,
    /// The status of the transaction, one of `succeeded`, `pending`, or `failed`.
    pub status: String,
    /// The type of source this transaction is attached to.
    #[serde(rename = "type")]
    pub type_: SourceTransactionType,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SourceTransaction {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum SourceTransactionObject {
    SourceTransaction,
}

impl SourceTransactionObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SourceTransaction => "source_transaction",
        }
    }
}

impl AsRef<str> for SourceTransactionObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SourceTransactionObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The type of source this transaction is attached to.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
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
            Self::AchCreditTransfer => "ach_credit_transfer",
            Self::AchDebit => "ach_debit",
            Self::Alipay => "alipay",
            Self::Bancontact => "bancontact",
            Self::Card => "card",
            Self::CardPresent => "card_present",
            Self::Eps => "eps",
            Self::Giropay => "giropay",
            Self::Ideal => "ideal",
            Self::Klarna => "klarna",
            Self::Multibanco => "multibanco",
            Self::P24 => "p24",
            Self::SepaDebit => "sepa_debit",
            Self::Sofort => "sofort",
            Self::ThreeDSecure => "three_d_secure",
            Self::Wechat => "wechat",
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
impl crate::Object for SourceTransaction {
    type Id = crate::source_transaction::SourceTransactionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
crate::def_id!(SourceTransactionId);
pub mod ach_credit_transfer_data;
pub use ach_credit_transfer_data::AchCreditTransferData;
pub mod chf_credit_transfer_data;
pub use chf_credit_transfer_data::ChfCreditTransferData;
pub mod gbp_credit_transfer_data;
pub use gbp_credit_transfer_data::GbpCreditTransferData;
pub mod paper_check_data;
pub use paper_check_data::PaperCheckData;
pub mod sepa_credit_transfer_data;
pub use sepa_credit_transfer_data::SepaCreditTransferData;
