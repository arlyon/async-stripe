#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionsResourceBillingModeFlexible {
    /// Controls how invoices and invoice items display proration amounts and discount amounts.
    pub proration_discounts: Option<SubscriptionsResourceBillingModeFlexibleProrationDiscounts>,
}
#[doc(hidden)]
pub struct SubscriptionsResourceBillingModeFlexibleBuilder {
    proration_discounts: Option<Option<SubscriptionsResourceBillingModeFlexibleProrationDiscounts>>,
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

    impl Deserialize for SubscriptionsResourceBillingModeFlexible {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionsResourceBillingModeFlexible>,
        builder: SubscriptionsResourceBillingModeFlexibleBuilder,
    }

    impl Visitor for Place<SubscriptionsResourceBillingModeFlexible> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SubscriptionsResourceBillingModeFlexibleBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SubscriptionsResourceBillingModeFlexibleBuilder {
        type Out = SubscriptionsResourceBillingModeFlexible;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "proration_discounts" => Deserialize::begin(&mut self.proration_discounts),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { proration_discounts: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(proration_discounts),) = (self.proration_discounts,) else {
                return None;
            };
            Some(Self::Out { proration_discounts })
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

    impl ObjectDeser for SubscriptionsResourceBillingModeFlexible {
        type Builder = SubscriptionsResourceBillingModeFlexibleBuilder;
    }

    impl FromValueOpt for SubscriptionsResourceBillingModeFlexible {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SubscriptionsResourceBillingModeFlexibleBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "proration_discounts" => b.proration_discounts = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Controls how invoices and invoice items display proration amounts and discount amounts.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SubscriptionsResourceBillingModeFlexibleProrationDiscounts {
    Included,
    Itemized,
}
impl SubscriptionsResourceBillingModeFlexibleProrationDiscounts {
    pub fn as_str(self) -> &'static str {
        use SubscriptionsResourceBillingModeFlexibleProrationDiscounts::*;
        match self {
            Included => "included",
            Itemized => "itemized",
        }
    }
}

impl std::str::FromStr for SubscriptionsResourceBillingModeFlexibleProrationDiscounts {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionsResourceBillingModeFlexibleProrationDiscounts::*;
        match s {
            "included" => Ok(Included),
            "itemized" => Ok(Itemized),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for SubscriptionsResourceBillingModeFlexibleProrationDiscounts {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SubscriptionsResourceBillingModeFlexibleProrationDiscounts {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SubscriptionsResourceBillingModeFlexibleProrationDiscounts {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for SubscriptionsResourceBillingModeFlexibleProrationDiscounts {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<SubscriptionsResourceBillingModeFlexibleProrationDiscounts>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            SubscriptionsResourceBillingModeFlexibleProrationDiscounts::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    SubscriptionsResourceBillingModeFlexibleProrationDiscounts
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for SubscriptionsResourceBillingModeFlexibleProrationDiscounts {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for SubscriptionsResourceBillingModeFlexibleProrationDiscounts",
            )
        })
    }
}
