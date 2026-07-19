#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct OutboundTransfersPaymentMethodDetails {
    pub billing_details: stripe_treasury::TreasurySharedResourceBillingDetails,
    pub financial_account:
        Option<stripe_treasury::OutboundTransfersPaymentMethodDetailsFinancialAccount>,
    /// The type of the payment method used in the OutboundTransfer.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: OutboundTransfersPaymentMethodDetailsType,
    pub us_bank_account:
        Option<stripe_treasury::OutboundTransfersPaymentMethodDetailsUsBankAccount>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for OutboundTransfersPaymentMethodDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("OutboundTransfersPaymentMethodDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct OutboundTransfersPaymentMethodDetailsBuilder {
    billing_details: Option<stripe_treasury::TreasurySharedResourceBillingDetails>,
    financial_account:
        Option<Option<stripe_treasury::OutboundTransfersPaymentMethodDetailsFinancialAccount>>,
    type_: Option<OutboundTransfersPaymentMethodDetailsType>,
    us_bank_account:
        Option<Option<stripe_treasury::OutboundTransfersPaymentMethodDetailsUsBankAccount>>,
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

    impl Deserialize for OutboundTransfersPaymentMethodDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<OutboundTransfersPaymentMethodDetails>,
        builder: OutboundTransfersPaymentMethodDetailsBuilder,
    }

    impl Visitor for Place<OutboundTransfersPaymentMethodDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: OutboundTransfersPaymentMethodDetailsBuilder {
                    billing_details: Deserialize::default(),
                    financial_account: Deserialize::default(),
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
                "financial_account" => Deserialize::begin(&mut self.builder.financial_account),
                "type" => Deserialize::begin(&mut self.builder.type_),
                "us_bank_account" => Deserialize::begin(&mut self.builder.us_bank_account),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(billing_details),
                Some(financial_account),
                Some(type_),
                Some(us_bank_account),
            ) = (
                self.builder.billing_details.take(),
                self.builder.financial_account.take(),
                self.builder.type_.take(),
                self.builder.us_bank_account.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(OutboundTransfersPaymentMethodDetails {
                billing_details,
                financial_account,
                type_,
                us_bank_account,
            });
            Ok(())
        }
    }
};
/// The type of the payment method used in the OutboundTransfer.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum OutboundTransfersPaymentMethodDetailsType {
    FinancialAccount,
    UsBankAccount,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl OutboundTransfersPaymentMethodDetailsType {
    pub fn as_str(&self) -> &str {
        use OutboundTransfersPaymentMethodDetailsType::*;
        match self {
            FinancialAccount => "financial_account",
            UsBankAccount => "us_bank_account",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for OutboundTransfersPaymentMethodDetailsType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use OutboundTransfersPaymentMethodDetailsType::*;
        match s {
            "financial_account" => Ok(FinancialAccount),
            "us_bank_account" => Ok(UsBankAccount),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "OutboundTransfersPaymentMethodDetailsType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for OutboundTransfersPaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for OutboundTransfersPaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for OutboundTransfersPaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(OutboundTransfersPaymentMethodDetailsType))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for OutboundTransfersPaymentMethodDetailsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for OutboundTransfersPaymentMethodDetailsType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<OutboundTransfersPaymentMethodDetailsType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(OutboundTransfersPaymentMethodDetailsType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for OutboundTransfersPaymentMethodDetailsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
