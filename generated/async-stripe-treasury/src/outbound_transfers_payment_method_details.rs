#[derive(Clone, Debug)]
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
                builder: OutboundTransfersPaymentMethodDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for OutboundTransfersPaymentMethodDetailsBuilder {
        type Out = OutboundTransfersPaymentMethodDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "billing_details" => Deserialize::begin(&mut self.billing_details),
                "financial_account" => Deserialize::begin(&mut self.financial_account),
                "type" => Deserialize::begin(&mut self.type_),
                "us_bank_account" => Deserialize::begin(&mut self.us_bank_account),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                billing_details: Deserialize::default(),
                financial_account: Deserialize::default(),
                type_: Deserialize::default(),
                us_bank_account: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(billing_details),
                Some(financial_account),
                Some(type_),
                Some(us_bank_account),
            ) = (
                self.billing_details.take(),
                self.financial_account.take(),
                self.type_,
                self.us_bank_account.take(),
            )
            else {
                return None;
            };
            Some(Self::Out { billing_details, financial_account, type_, us_bank_account })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for OutboundTransfersPaymentMethodDetails {
        type Builder = OutboundTransfersPaymentMethodDetailsBuilder;
    }

    impl FromValueOpt for OutboundTransfersPaymentMethodDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = OutboundTransfersPaymentMethodDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "billing_details" => b.billing_details = FromValueOpt::from_value(v),
                    "financial_account" => b.financial_account = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    "us_bank_account" => b.us_bank_account = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The type of the payment method used in the OutboundTransfer.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum OutboundTransfersPaymentMethodDetailsType {
    FinancialAccount,
    UsBankAccount,
}
impl OutboundTransfersPaymentMethodDetailsType {
    pub fn as_str(self) -> &'static str {
        use OutboundTransfersPaymentMethodDetailsType::*;
        match self {
            FinancialAccount => "financial_account",
            UsBankAccount => "us_bank_account",
        }
    }
}

impl std::str::FromStr for OutboundTransfersPaymentMethodDetailsType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use OutboundTransfersPaymentMethodDetailsType::*;
        match s {
            "financial_account" => Ok(FinancialAccount),
            "us_bank_account" => Ok(UsBankAccount),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for OutboundTransfersPaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for OutboundTransfersPaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for OutboundTransfersPaymentMethodDetailsType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<OutboundTransfersPaymentMethodDetailsType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            OutboundTransfersPaymentMethodDetailsType::from_str(s).map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(OutboundTransfersPaymentMethodDetailsType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for OutboundTransfersPaymentMethodDetailsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for OutboundTransfersPaymentMethodDetailsType")
        })
    }
}
