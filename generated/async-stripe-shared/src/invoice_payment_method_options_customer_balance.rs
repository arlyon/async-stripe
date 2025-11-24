#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InvoicePaymentMethodOptionsCustomerBalance {
    pub bank_transfer:
        Option<stripe_shared::InvoicePaymentMethodOptionsCustomerBalanceBankTransfer>,
    /// The funding method type to be used when there are not enough funds in the customer balance.
    /// Permitted values include: `bank_transfer`.
    pub funding_type: Option<InvoicePaymentMethodOptionsCustomerBalanceFundingType>,
}
#[doc(hidden)]
pub struct InvoicePaymentMethodOptionsCustomerBalanceBuilder {
    bank_transfer:
        Option<Option<stripe_shared::InvoicePaymentMethodOptionsCustomerBalanceBankTransfer>>,
    funding_type: Option<Option<InvoicePaymentMethodOptionsCustomerBalanceFundingType>>,
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

    impl Deserialize for InvoicePaymentMethodOptionsCustomerBalance {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoicePaymentMethodOptionsCustomerBalance>,
        builder: InvoicePaymentMethodOptionsCustomerBalanceBuilder,
    }

    impl Visitor for Place<InvoicePaymentMethodOptionsCustomerBalance> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InvoicePaymentMethodOptionsCustomerBalanceBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InvoicePaymentMethodOptionsCustomerBalanceBuilder {
        type Out = InvoicePaymentMethodOptionsCustomerBalance;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bank_transfer" => Deserialize::begin(&mut self.bank_transfer),
                "funding_type" => Deserialize::begin(&mut self.funding_type),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { bank_transfer: Deserialize::default(), funding_type: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(bank_transfer), Some(funding_type)) =
                (self.bank_transfer.take(), self.funding_type.take())
            else {
                return None;
            };
            Some(Self::Out { bank_transfer, funding_type })
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

    impl ObjectDeser for InvoicePaymentMethodOptionsCustomerBalance {
        type Builder = InvoicePaymentMethodOptionsCustomerBalanceBuilder;
    }

    impl FromValueOpt for InvoicePaymentMethodOptionsCustomerBalance {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InvoicePaymentMethodOptionsCustomerBalanceBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "bank_transfer" => b.bank_transfer = FromValueOpt::from_value(v),
                    "funding_type" => b.funding_type = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The funding method type to be used when there are not enough funds in the customer balance.
/// Permitted values include: `bank_transfer`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum InvoicePaymentMethodOptionsCustomerBalanceFundingType {
    BankTransfer,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl InvoicePaymentMethodOptionsCustomerBalanceFundingType {
    pub fn as_str(&self) -> &str {
        use InvoicePaymentMethodOptionsCustomerBalanceFundingType::*;
        match self {
            BankTransfer => "bank_transfer",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for InvoicePaymentMethodOptionsCustomerBalanceFundingType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InvoicePaymentMethodOptionsCustomerBalanceFundingType::*;
        match s {
            "bank_transfer" => Ok(BankTransfer),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "InvoicePaymentMethodOptionsCustomerBalanceFundingType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for InvoicePaymentMethodOptionsCustomerBalanceFundingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InvoicePaymentMethodOptionsCustomerBalanceFundingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for InvoicePaymentMethodOptionsCustomerBalanceFundingType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for InvoicePaymentMethodOptionsCustomerBalanceFundingType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<InvoicePaymentMethodOptionsCustomerBalanceFundingType>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            InvoicePaymentMethodOptionsCustomerBalanceFundingType::from_str(s).expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(InvoicePaymentMethodOptionsCustomerBalanceFundingType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for InvoicePaymentMethodOptionsCustomerBalanceFundingType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
