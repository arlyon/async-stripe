/// Each customer has a [`balance`](https://stripe.com/docs/api/customers/object#customer_object-balance) that is.
/// automatically applied to future invoices and payments using the `customer_balance` payment method.
/// Customers can fund this balance by initiating a bank transfer to any account in the
/// `financial_addresses` field.
/// Related guide: [Customer balance funding instructions](https://stripe.com/docs/payments/customer-balance/funding-instructions).
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct FundingInstructions {
    pub bank_transfer: stripe_shared::FundingInstructionsBankTransfer,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The `funding_type` of the returned instructions
    pub funding_type: FundingInstructionsFundingType,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
}
#[doc(hidden)]
pub struct FundingInstructionsBuilder {
    bank_transfer: Option<stripe_shared::FundingInstructionsBankTransfer>,
    currency: Option<stripe_types::Currency>,
    funding_type: Option<FundingInstructionsFundingType>,
    livemode: Option<bool>,
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
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for FundingInstructions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<FundingInstructions>,
        builder: FundingInstructionsBuilder,
    }

    impl Visitor for Place<FundingInstructions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: FundingInstructionsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for FundingInstructionsBuilder {
        type Out = FundingInstructions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bank_transfer" => Deserialize::begin(&mut self.bank_transfer),
                "currency" => Deserialize::begin(&mut self.currency),
                "funding_type" => Deserialize::begin(&mut self.funding_type),
                "livemode" => Deserialize::begin(&mut self.livemode),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                bank_transfer: Deserialize::default(),
                currency: Deserialize::default(),
                funding_type: Deserialize::default(),
                livemode: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(bank_transfer), Some(currency), Some(funding_type), Some(livemode)) =
                (self.bank_transfer.take(), self.currency.take(), self.funding_type, self.livemode)
            else {
                return None;
            };
            Some(Self::Out { bank_transfer, currency, funding_type, livemode })
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

    impl ObjectDeser for FundingInstructions {
        type Builder = FundingInstructionsBuilder;
    }

    impl FromValueOpt for FundingInstructions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = FundingInstructionsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "bank_transfer" => b.bank_transfer = FromValueOpt::from_value(v),
                    "currency" => b.currency = FromValueOpt::from_value(v),
                    "funding_type" => b.funding_type = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for FundingInstructions {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("FundingInstructions", 5)?;
        s.serialize_field("bank_transfer", &self.bank_transfer)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("funding_type", &self.funding_type)?;
        s.serialize_field("livemode", &self.livemode)?;

        s.serialize_field("object", "funding_instructions")?;
        s.end()
    }
}
/// The `funding_type` of the returned instructions
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum FundingInstructionsFundingType {
    BankTransfer,
}
impl FundingInstructionsFundingType {
    pub fn as_str(self) -> &'static str {
        use FundingInstructionsFundingType::*;
        match self {
            BankTransfer => "bank_transfer",
        }
    }
}

impl std::str::FromStr for FundingInstructionsFundingType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FundingInstructionsFundingType::*;
        match s {
            "bank_transfer" => Ok(BankTransfer),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for FundingInstructionsFundingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for FundingInstructionsFundingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for FundingInstructionsFundingType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for FundingInstructionsFundingType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<FundingInstructionsFundingType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(FundingInstructionsFundingType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(FundingInstructionsFundingType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for FundingInstructionsFundingType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for FundingInstructionsFundingType")
        })
    }
}
