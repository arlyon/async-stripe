/// Each customer has a [`balance`](https://docs.stripe.com/api/customers/object#customer_object-balance) that is.
/// automatically applied to future invoices and payments using the `customer_balance` payment method.
/// Customers can fund this balance by initiating a bank transfer to any account in the
/// `financial_addresses` field.
/// Related guide: [Customer balance funding instructions](https://docs.stripe.com/payments/customer-balance/funding-instructions).
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct FundingInstructions {
    pub bank_transfer: stripe_shared::FundingInstructionsBankTransfer,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The `funding_type` of the returned instructions
    pub funding_type: FundingInstructionsFundingType,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FundingInstructions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FundingInstructions").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: FundingInstructionsBuilder {
                    bank_transfer: Deserialize::default(),
                    currency: Deserialize::default(),
                    funding_type: Deserialize::default(),
                    livemode: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bank_transfer" => Deserialize::begin(&mut self.builder.bank_transfer),
                "currency" => Deserialize::begin(&mut self.builder.currency),
                "funding_type" => Deserialize::begin(&mut self.builder.funding_type),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(bank_transfer), Some(currency), Some(funding_type), Some(livemode)) = (
                self.builder.bank_transfer.take(),
                self.builder.currency.take(),
                self.builder.funding_type.take(),
                self.builder.livemode,
            ) else {
                return Ok(());
            };
            *self.out =
                Some(FundingInstructions { bank_transfer, currency, funding_type, livemode });
            Ok(())
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum FundingInstructionsFundingType {
    BankTransfer,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl FundingInstructionsFundingType {
    pub fn as_str(&self) -> &str {
        use FundingInstructionsFundingType::*;
        match self {
            BankTransfer => "bank_transfer",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for FundingInstructionsFundingType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FundingInstructionsFundingType::*;
        match s {
            "bank_transfer" => Ok(BankTransfer),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "FundingInstructionsFundingType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for FundingInstructionsFundingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for FundingInstructionsFundingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FundingInstructionsFundingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(FundingInstructionsFundingType)).finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for FundingInstructionsFundingType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<FundingInstructionsFundingType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(FundingInstructionsFundingType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for FundingInstructionsFundingType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
