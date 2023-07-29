/// Some payment methods have no required amount that a customer must send.
/// Customers can be instructed to send any amount, and it can be made up of
/// multiple transactions.
///
/// As such, sources can have multiple associated transactions.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SourceTransaction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_credit_transfer:
        Option<stripe_types::source_transaction::ach_credit_transfer_data::AchCreditTransferData>,
    /// A positive integer in the smallest currency unit (that is, 100 cents for $1.00, or 1 for ¥1, Japanese Yen being a zero-decimal currency) representing the amount your customer has pushed to the receiver.
    pub amount: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chf_credit_transfer:
        Option<stripe_types::source_transaction::chf_credit_transfer_data::ChfCreditTransferData>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gbp_credit_transfer:
        Option<stripe_types::source_transaction::gbp_credit_transfer_data::GbpCreditTransferData>,
    /// Unique identifier for the object.
    pub id: stripe_types::source_transaction::SourceTransactionId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: SourceTransactionObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paper_check: Option<stripe_types::source_transaction::paper_check_data::PaperCheckData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_credit_transfer:
        Option<stripe_types::source_transaction::sepa_credit_transfer_data::SepaCreditTransferData>,
    /// The ID of the source this transaction is attached to.
    pub source: String,
    /// The status of the transaction, one of `succeeded`, `pending`, or `failed`.
    pub status: String,
    /// The type of source this transaction is attached to.
    #[serde(rename = "type")]
    pub type_: SourceTransactionType,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for SourceTransactionObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "source_transaction" => Ok(Self::SourceTransaction),

            _ => Err(()),
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
impl serde::Serialize for SourceTransactionObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SourceTransactionObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for SourceTransactionObject"))
    }
}
/// The type of source this transaction is attached to.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for SourceTransactionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ach_credit_transfer" => Ok(Self::AchCreditTransfer),
            "ach_debit" => Ok(Self::AchDebit),
            "alipay" => Ok(Self::Alipay),
            "bancontact" => Ok(Self::Bancontact),
            "card" => Ok(Self::Card),
            "card_present" => Ok(Self::CardPresent),
            "eps" => Ok(Self::Eps),
            "giropay" => Ok(Self::Giropay),
            "ideal" => Ok(Self::Ideal),
            "klarna" => Ok(Self::Klarna),
            "multibanco" => Ok(Self::Multibanco),
            "p24" => Ok(Self::P24),
            "sepa_debit" => Ok(Self::SepaDebit),
            "sofort" => Ok(Self::Sofort),
            "three_d_secure" => Ok(Self::ThreeDSecure),
            "wechat" => Ok(Self::Wechat),

            _ => Err(()),
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
impl serde::Serialize for SourceTransactionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SourceTransactionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for SourceTransactionType"))
    }
}
impl stripe_types::Object for SourceTransaction {
    type Id = stripe_types::source_transaction::SourceTransactionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(SourceTransactionId);
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
