#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InvoiceMandateOptionsCard {
    /// Amount to be charged for future payments.
    pub amount: Option<i64>,
    /// One of `fixed` or `maximum`.
    /// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
    /// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
    pub amount_type: Option<InvoiceMandateOptionsCardAmountType>,
    /// A description of the mandate or subscription that is meant to be displayed to the customer.
    pub description: Option<String>,
}
#[doc(hidden)]
pub struct InvoiceMandateOptionsCardBuilder {
    amount: Option<Option<i64>>,
    amount_type: Option<Option<InvoiceMandateOptionsCardAmountType>>,
    description: Option<Option<String>>,
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

    impl Deserialize for InvoiceMandateOptionsCard {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoiceMandateOptionsCard>,
        builder: InvoiceMandateOptionsCardBuilder,
    }

    impl Visitor for Place<InvoiceMandateOptionsCard> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InvoiceMandateOptionsCardBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InvoiceMandateOptionsCardBuilder {
        type Out = InvoiceMandateOptionsCard;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "amount_type" => Deserialize::begin(&mut self.amount_type),
                "description" => Deserialize::begin(&mut self.description),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                amount_type: Deserialize::default(),
                description: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(amount), Some(amount_type), Some(description)) =
                (self.amount, self.amount_type.take(), self.description.take())
            else {
                return None;
            };
            Some(Self::Out { amount, amount_type, description })
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

    impl ObjectDeser for InvoiceMandateOptionsCard {
        type Builder = InvoiceMandateOptionsCardBuilder;
    }

    impl FromValueOpt for InvoiceMandateOptionsCard {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InvoiceMandateOptionsCardBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "amount_type" => b.amount_type = FromValueOpt::from_value(v),
                    "description" => b.description = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// One of `fixed` or `maximum`.
/// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
/// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum InvoiceMandateOptionsCardAmountType {
    Fixed,
    Maximum,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl InvoiceMandateOptionsCardAmountType {
    pub fn as_str(&self) -> &str {
        use InvoiceMandateOptionsCardAmountType::*;
        match self {
            Fixed => "fixed",
            Maximum => "maximum",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for InvoiceMandateOptionsCardAmountType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InvoiceMandateOptionsCardAmountType::*;
        match s {
            "fixed" => Ok(Fixed),
            "maximum" => Ok(Maximum),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "InvoiceMandateOptionsCardAmountType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for InvoiceMandateOptionsCardAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InvoiceMandateOptionsCardAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for InvoiceMandateOptionsCardAmountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for InvoiceMandateOptionsCardAmountType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<InvoiceMandateOptionsCardAmountType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(InvoiceMandateOptionsCardAmountType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(InvoiceMandateOptionsCardAmountType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for InvoiceMandateOptionsCardAmountType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
