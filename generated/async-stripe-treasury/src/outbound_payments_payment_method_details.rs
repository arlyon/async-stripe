#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct OutboundPaymentsPaymentMethodDetails {
    pub billing_details: stripe_treasury::TreasurySharedResourceBillingDetails,
    pub financial_account:
        Option<stripe_treasury::OutboundPaymentsPaymentMethodDetailsFinancialAccount>,
    /// The type of the payment method used in the OutboundPayment.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: OutboundPaymentsPaymentMethodDetailsType,
    pub us_bank_account: Option<stripe_treasury::OutboundPaymentsPaymentMethodDetailsUsBankAccount>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for OutboundPaymentsPaymentMethodDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("OutboundPaymentsPaymentMethodDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct OutboundPaymentsPaymentMethodDetailsBuilder {
    billing_details: Option<stripe_treasury::TreasurySharedResourceBillingDetails>,
    financial_account:
        Option<Option<stripe_treasury::OutboundPaymentsPaymentMethodDetailsFinancialAccount>>,
    type_: Option<OutboundPaymentsPaymentMethodDetailsType>,
    us_bank_account:
        Option<Option<stripe_treasury::OutboundPaymentsPaymentMethodDetailsUsBankAccount>>,
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

    impl Deserialize for OutboundPaymentsPaymentMethodDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<OutboundPaymentsPaymentMethodDetails>,
        builder: OutboundPaymentsPaymentMethodDetailsBuilder,
    }

    impl Visitor for Place<OutboundPaymentsPaymentMethodDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: OutboundPaymentsPaymentMethodDetailsBuilder {
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
            *self.out = Some(OutboundPaymentsPaymentMethodDetails {
                billing_details,
                financial_account,
                type_,
                us_bank_account,
            });
            Ok(())
        }
    }
};
/// The type of the payment method used in the OutboundPayment.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum OutboundPaymentsPaymentMethodDetailsType {
    FinancialAccount,
    UsBankAccount,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl OutboundPaymentsPaymentMethodDetailsType {
    pub fn as_str(&self) -> &str {
        use OutboundPaymentsPaymentMethodDetailsType::*;
        match self {
            FinancialAccount => "financial_account",
            UsBankAccount => "us_bank_account",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for OutboundPaymentsPaymentMethodDetailsType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use OutboundPaymentsPaymentMethodDetailsType::*;
        match s {
            "financial_account" => Ok(FinancialAccount),
            "us_bank_account" => Ok(UsBankAccount),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "OutboundPaymentsPaymentMethodDetailsType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for OutboundPaymentsPaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for OutboundPaymentsPaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for OutboundPaymentsPaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(OutboundPaymentsPaymentMethodDetailsType)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for OutboundPaymentsPaymentMethodDetailsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for OutboundPaymentsPaymentMethodDetailsType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<OutboundPaymentsPaymentMethodDetailsType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(OutboundPaymentsPaymentMethodDetailsType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for OutboundPaymentsPaymentMethodDetailsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
