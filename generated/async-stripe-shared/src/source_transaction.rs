/// Some payment methods have no required amount that a customer must send.
/// Customers can be instructed to send any amount, and it can be made up of
/// multiple transactions. As such, sources can have multiple associated
/// transactions.
#[derive(Clone, Debug)]
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
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
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
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: SourceTransactionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SourceTransactionBuilder {
        type Out = SourceTransaction;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "ach_credit_transfer" => Deserialize::begin(&mut self.ach_credit_transfer),
                "amount" => Deserialize::begin(&mut self.amount),
                "chf_credit_transfer" => Deserialize::begin(&mut self.chf_credit_transfer),
                "created" => Deserialize::begin(&mut self.created),
                "currency" => Deserialize::begin(&mut self.currency),
                "gbp_credit_transfer" => Deserialize::begin(&mut self.gbp_credit_transfer),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "paper_check" => Deserialize::begin(&mut self.paper_check),
                "sepa_credit_transfer" => Deserialize::begin(&mut self.sepa_credit_transfer),
                "source" => Deserialize::begin(&mut self.source),
                "status" => Deserialize::begin(&mut self.status),
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
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
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
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
                self.ach_credit_transfer.take(),
                self.amount,
                self.chf_credit_transfer.take(),
                self.created,
                self.currency,
                self.gbp_credit_transfer.take(),
                self.id.take(),
                self.livemode,
                self.paper_check.take(),
                self.sepa_credit_transfer.take(),
                self.source.take(),
                self.status.take(),
                self.type_.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
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
            })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for SourceTransaction {
        type Builder = SourceTransactionBuilder;
    }

    impl FromValueOpt for SourceTransaction {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SourceTransactionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "ach_credit_transfer" => b.ach_credit_transfer = FromValueOpt::from_value(v),
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "chf_credit_transfer" => b.chf_credit_transfer = FromValueOpt::from_value(v),
                    "created" => b.created = FromValueOpt::from_value(v),
                    "currency" => b.currency = FromValueOpt::from_value(v),
                    "gbp_credit_transfer" => b.gbp_credit_transfer = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "paper_check" => b.paper_check = FromValueOpt::from_value(v),
                    "sepa_credit_transfer" => b.sepa_credit_transfer = FromValueOpt::from_value(v),
                    "source" => b.source = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
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
            v => Ok(Unknown(v.to_owned())),
        }
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
#[cfg(feature = "serialize")]
impl serde::Serialize for SourceTransactionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for SourceTransactionType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<SourceTransactionType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SourceTransactionType::from_str(s).unwrap());
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(SourceTransactionType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for SourceTransactionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
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
