/// For more details see <<https://stripe.com/docs/api/treasury/inbound_transfers>>.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InboundTransfers {
    pub billing_details: stripe_treasury::TreasurySharedResourceBillingDetails,
    /// The type of the payment method used in the InboundTransfer.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: InboundTransfersType,
    pub us_bank_account: Option<stripe_treasury::InboundTransfersPaymentMethodDetailsUsBankAccount>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InboundTransfers {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InboundTransfers").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct InboundTransfersBuilder {
    billing_details: Option<stripe_treasury::TreasurySharedResourceBillingDetails>,
    type_: Option<InboundTransfersType>,
    us_bank_account:
        Option<Option<stripe_treasury::InboundTransfersPaymentMethodDetailsUsBankAccount>>,
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

    impl Deserialize for InboundTransfers {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InboundTransfers>,
        builder: InboundTransfersBuilder,
    }

    impl Visitor for Place<InboundTransfers> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InboundTransfersBuilder {
                    billing_details: Deserialize::default(),
                    type_: Deserialize::default(),
                    us_bank_account: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "billing_details" => Deserialize::begin(&mut self.builder.billing_details),
                "type" => Deserialize::begin(&mut self.builder.type_),
                "us_bank_account" => Deserialize::begin(&mut self.builder.us_bank_account),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(billing_details), Some(type_), Some(us_bank_account)) = (
                self.builder.billing_details.take(),
                self.builder.type_.take(),
                self.builder.us_bank_account.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(InboundTransfers { billing_details, type_, us_bank_account });
            Ok(())
        }
    }
};
/// The type of the payment method used in the InboundTransfer.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum InboundTransfersType {
    UsBankAccount,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl InboundTransfersType {
    pub fn as_str(&self) -> &str {
        use InboundTransfersType::*;
        match self {
            UsBankAccount => "us_bank_account",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for InboundTransfersType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InboundTransfersType::*;
        match s {
            "us_bank_account" => Ok(UsBankAccount),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "InboundTransfersType");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for InboundTransfersType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for InboundTransfersType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InboundTransfersType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(InboundTransfersType)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for InboundTransfersType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for InboundTransfersType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<InboundTransfersType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(InboundTransfersType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for InboundTransfersType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
