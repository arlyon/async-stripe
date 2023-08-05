/// Some payment methods have no required amount that a customer must send.
/// Customers can be instructed to send any amount, and it can be made up of
/// multiple transactions.
///
/// As such, sources can have multiple associated transactions.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SourceTransaction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_credit_transfer: Option<stripe_types::SourceTransactionAchCreditTransferData>,
    /// A positive integer in the smallest currency unit (that is, 100 cents for $1.00, or 1 for ¥1, Japanese Yen being a zero-decimal currency) representing the amount your customer has pushed to the receiver.
    pub amount: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chf_credit_transfer: Option<stripe_types::SourceTransactionChfCreditTransferData>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gbp_credit_transfer: Option<stripe_types::SourceTransactionGbpCreditTransferData>,
    /// Unique identifier for the object.
    pub id: stripe_types::source_transaction::SourceTransactionId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paper_check: Option<stripe_types::SourceTransactionPaperCheckData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_credit_transfer: Option<stripe_types::SourceTransactionSepaCreditTransferData>,
    /// The ID of the source this transaction is attached to.
    pub source: String,
    /// The status of the transaction, one of `succeeded`, `pending`, or `failed`.
    pub status: String,
    /// The type of source this transaction is attached to.
    #[serde(rename = "type")]
    pub type_: SourceTransactionType,
}
/// The type of source this transaction is attached to.
#[derive(Copy, Clone, Eq, PartialEq)]
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
        use SourceTransactionType::*;
        match self {
            AchCreditTransfer => "ach_credit_transfer",
            AchDebit => "ach_debit",
            Alipay => "alipay",
            Bancontact => "bancontact",
            Card => "card",
            CardPresent => "card_present",
            Eps => "eps",
            Giropay => "giropay",
            Ideal => "ideal",
            Klarna => "klarna",
            Multibanco => "multibanco",
            P24 => "p24",
            SepaDebit => "sepa_debit",
            Sofort => "sofort",
            ThreeDSecure => "three_d_secure",
            Wechat => "wechat",
        }
    }
}

impl std::str::FromStr for SourceTransactionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SourceTransactionType::*;
        match s {
            "ach_credit_transfer" => Ok(AchCreditTransfer),
            "ach_debit" => Ok(AchDebit),
            "alipay" => Ok(Alipay),
            "bancontact" => Ok(Bancontact),
            "card" => Ok(Card),
            "card_present" => Ok(CardPresent),
            "eps" => Ok(Eps),
            "giropay" => Ok(Giropay),
            "ideal" => Ok(Ideal),
            "klarna" => Ok(Klarna),
            "multibanco" => Ok(Multibanco),
            "p24" => Ok(P24),
            "sepa_debit" => Ok(SepaDebit),
            "sofort" => Ok(Sofort),
            "three_d_secure" => Ok(ThreeDSecure),
            "wechat" => Ok(Wechat),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SourceTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
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
