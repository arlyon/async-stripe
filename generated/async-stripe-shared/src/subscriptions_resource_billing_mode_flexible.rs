#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionsResourceBillingModeFlexible {
    /// Controls how invoices and invoice items display proration amounts and discount amounts.
    pub proration_discounts: Option<SubscriptionsResourceBillingModeFlexibleProrationDiscounts>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionsResourceBillingModeFlexible {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionsResourceBillingModeFlexible").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SubscriptionsResourceBillingModeFlexibleBuilder {
    proration_discounts: Option<Option<SubscriptionsResourceBillingModeFlexibleProrationDiscounts>>,
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
                builder: SubscriptionsResourceBillingModeFlexibleBuilder {
                    proration_discounts: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "proration_discounts" => Deserialize::begin(&mut self.builder.proration_discounts),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(proration_discounts),) = (self.builder.proration_discounts.take(),) else {
                return Ok(());
            };
            *self.out = Some(SubscriptionsResourceBillingModeFlexible { proration_discounts });
            Ok(())
        }
    }
};
/// Controls how invoices and invoice items display proration amounts and discount amounts.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum SubscriptionsResourceBillingModeFlexibleProrationDiscounts {
    Included,
    Itemized,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl SubscriptionsResourceBillingModeFlexibleProrationDiscounts {
    pub fn as_str(&self) -> &str {
        use SubscriptionsResourceBillingModeFlexibleProrationDiscounts::*;
        match self {
            Included => "included",
            Itemized => "itemized",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for SubscriptionsResourceBillingModeFlexibleProrationDiscounts {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionsResourceBillingModeFlexibleProrationDiscounts::*;
        match s {
            "included" => Ok(Included),
            "itemized" => Ok(Itemized),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "SubscriptionsResourceBillingModeFlexibleProrationDiscounts"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for SubscriptionsResourceBillingModeFlexibleProrationDiscounts {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for SubscriptionsResourceBillingModeFlexibleProrationDiscounts {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionsResourceBillingModeFlexibleProrationDiscounts {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(SubscriptionsResourceBillingModeFlexibleProrationDiscounts))
            .finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for SubscriptionsResourceBillingModeFlexibleProrationDiscounts {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<SubscriptionsResourceBillingModeFlexibleProrationDiscounts>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            SubscriptionsResourceBillingModeFlexibleProrationDiscounts::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for SubscriptionsResourceBillingModeFlexibleProrationDiscounts {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
