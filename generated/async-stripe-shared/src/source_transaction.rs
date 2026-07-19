/// Some payment methods have no required amount that a customer must send.
/// Customers can be instructed to send any amount, and it can be made up of
/// multiple transactions. As such, sources can have multiple associated
/// transactions.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SourceTransaction {
    pub ach_credit_transfer: Option<stripe_shared::SourceTransactionAchCreditTransferData>,
    /// A positive integer in the smallest currency unit (that is, 100 cents for $1.00, or 1 for ¥1, Japanese Yen being a zero-decimal currency) representing the amount your customer has pushed to the receiver.
    pub amount: i64,
    pub chf_credit_transfer: Option<stripe_shared::SourceTransactionChfCreditTransferData>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    pub gbp_credit_transfer: Option<stripe_shared::SourceTransactionGbpCreditTransferData>,
    /// Unique identifier for the object.
    pub id: stripe_shared::SourceTransactionId,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    pub paper_check: Option<stripe_shared::SourceTransactionPaperCheckData>,
    pub sepa_credit_transfer: Option<stripe_shared::SourceTransactionSepaCreditTransferData>,
    /// The ID of the source this transaction is attached to.
    pub source: String,
    /// The status of the transaction, one of `succeeded`, `pending`, or `failed`.
    pub status: String,
    /// The type of source this transaction is attached to.
    #[cfg_attr(feature = "deserialize", serde(rename = "type"))]
    pub type_: SourceTransactionType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SourceTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SourceTransaction").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SourceTransactionBuilder {
    ach_credit_transfer: Option<Option<stripe_shared::SourceTransactionAchCreditTransferData>>,
    amount: Option<i64>,
    chf_credit_transfer: Option<Option<stripe_shared::SourceTransactionChfCreditTransferData>>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<stripe_types::Currency>,
    gbp_credit_transfer: Option<Option<stripe_shared::SourceTransactionGbpCreditTransferData>>,
    id: Option<stripe_shared::SourceTransactionId>,
    livemode: Option<bool>,
    paper_check: Option<Option<stripe_shared::SourceTransactionPaperCheckData>>,
    sepa_credit_transfer: Option<Option<stripe_shared::SourceTransactionSepaCreditTransferData>>,
    source: Option<String>,
    status: Option<String>,
    type_: Option<SourceTransactionType>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

    make_place!(Place);

    impl Deserialize for SourceTransaction {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceTransaction>,
        builder: SourceTransactionBuilder,
    }

    impl Visitor for Place<SourceTransaction> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SourceTransactionBuilder {
                    ach_credit_transfer: Deserialize::default(),
                    amount: Deserialize::default(),
                    chf_credit_transfer: Deserialize::default(),
                    created: Deserialize::default(),
                    currency: Deserialize::default(),
                    gbp_credit_transfer: Deserialize::default(),
                    id: Deserialize::default(),
                    livemode: Deserialize::default(),
                    paper_check: Deserialize::default(),
                    sepa_credit_transfer: Deserialize::default(),
                    source: Deserialize::default(),
                    status: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "ach_credit_transfer" => Deserialize::begin(&mut self.builder.ach_credit_transfer),
                "amount" => Deserialize::begin(&mut self.builder.amount),
                "chf_credit_transfer" => Deserialize::begin(&mut self.builder.chf_credit_transfer),
                "created" => Deserialize::begin(&mut self.builder.created),
                "currency" => Deserialize::begin(&mut self.builder.currency),
                "gbp_credit_transfer" => Deserialize::begin(&mut self.builder.gbp_credit_transfer),
                "id" => Deserialize::begin(&mut self.builder.id),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "paper_check" => Deserialize::begin(&mut self.builder.paper_check),
                "sepa_credit_transfer" => {
                    Deserialize::begin(&mut self.builder.sepa_credit_transfer)
                }
                "source" => Deserialize::begin(&mut self.builder.source),
                "status" => Deserialize::begin(&mut self.builder.status),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(ach_credit_transfer),
                Some(amount),
                Some(chf_credit_transfer),
                Some(created),
                Some(currency),
                Some(gbp_credit_transfer),
                Some(id),
                Some(livemode),
                Some(paper_check),
                Some(sepa_credit_transfer),
                Some(source),
                Some(status),
                Some(type_),
            ) = (
                self.builder.ach_credit_transfer.take(),
                self.builder.amount,
                self.builder.chf_credit_transfer.take(),
                self.builder.created,
                self.builder.currency.take(),
                self.builder.gbp_credit_transfer.take(),
                self.builder.id.take(),
                self.builder.livemode,
                self.builder.paper_check.take(),
                self.builder.sepa_credit_transfer.take(),
                self.builder.source.take(),
                self.builder.status.take(),
                self.builder.type_.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(SourceTransaction {
                ach_credit_transfer,
                amount,
                chf_credit_transfer,
                created,
                currency,
                gbp_credit_transfer,
                id,
                livemode,
                paper_check,
                sepa_credit_transfer,
                source,
                status,
                type_,
            });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for SourceTransaction {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("SourceTransaction", 14)?;
        s.serialize_field("ach_credit_transfer", &self.ach_credit_transfer)?;
        s.serialize_field("amount", &self.amount)?;
        s.serialize_field("chf_credit_transfer", &self.chf_credit_transfer)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("gbp_credit_transfer", &self.gbp_credit_transfer)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("paper_check", &self.paper_check)?;
        s.serialize_field("sepa_credit_transfer", &self.sepa_credit_transfer)?;
        s.serialize_field("source", &self.source)?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("type", &self.type_)?;

        s.serialize_field("object", "source_transaction")?;
        s.end()
    }
}
/// The type of source this transaction is attached to.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
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
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl SourceTransactionType {
    pub fn as_str(&self) -> &str {
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
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for SourceTransactionType {
    type Err = std::convert::Infallible;
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
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "SourceTransactionType");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for SourceTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for SourceTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SourceTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(SourceTransactionType)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SourceTransactionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for SourceTransactionType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<SourceTransactionType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SourceTransactionType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for SourceTransactionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
impl stripe_types::Object for SourceTransaction {
    type Id = stripe_shared::SourceTransactionId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(SourceTransactionId);
