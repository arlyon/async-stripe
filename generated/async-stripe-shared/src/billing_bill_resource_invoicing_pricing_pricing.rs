#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingBillResourceInvoicingPricingPricing {
    pub price_details:
        Option<stripe_shared::BillingBillResourceInvoicingPricingPricingPriceDetails>,
    /// The type of the pricing details.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: BillingBillResourceInvoicingPricingPricingType,
    /// The unit amount (in the `currency` specified) of the item which contains a decimal value with at most 12 decimal places.
    pub unit_amount_decimal: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingBillResourceInvoicingPricingPricing {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BillingBillResourceInvoicingPricingPricing").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct BillingBillResourceInvoicingPricingPricingBuilder {
    price_details:
        Option<Option<stripe_shared::BillingBillResourceInvoicingPricingPricingPriceDetails>>,
    type_: Option<BillingBillResourceInvoicingPricingPricingType>,
    unit_amount_decimal: Option<Option<String>>,
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

    impl Deserialize for BillingBillResourceInvoicingPricingPricing {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BillingBillResourceInvoicingPricingPricing>,
        builder: BillingBillResourceInvoicingPricingPricingBuilder,
    }

    impl Visitor for Place<BillingBillResourceInvoicingPricingPricing> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BillingBillResourceInvoicingPricingPricingBuilder {
                    price_details: Deserialize::default(),
                    type_: Deserialize::default(),
                    unit_amount_decimal: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "price_details" => Deserialize::begin(&mut self.builder.price_details),
                "type" => Deserialize::begin(&mut self.builder.type_),
                "unit_amount_decimal" => Deserialize::begin(&mut self.builder.unit_amount_decimal),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(price_details), Some(type_), Some(unit_amount_decimal)) = (
                self.builder.price_details.take(),
                self.builder.type_.take(),
                self.builder.unit_amount_decimal.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(BillingBillResourceInvoicingPricingPricing {
                price_details,
                type_,
                unit_amount_decimal,
            });
            Ok(())
        }
    }
};
/// The type of the pricing details.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum BillingBillResourceInvoicingPricingPricingType {
    PriceDetails,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl BillingBillResourceInvoicingPricingPricingType {
    pub fn as_str(&self) -> &str {
        use BillingBillResourceInvoicingPricingPricingType::*;
        match self {
            PriceDetails => "price_details",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for BillingBillResourceInvoicingPricingPricingType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BillingBillResourceInvoicingPricingPricingType::*;
        match s {
            "price_details" => Ok(PriceDetails),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "BillingBillResourceInvoicingPricingPricingType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for BillingBillResourceInvoicingPricingPricingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for BillingBillResourceInvoicingPricingPricingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingBillResourceInvoicingPricingPricingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(BillingBillResourceInvoicingPricingPricingType))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for BillingBillResourceInvoicingPricingPricingType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for BillingBillResourceInvoicingPricingPricingType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<BillingBillResourceInvoicingPricingPricingType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(BillingBillResourceInvoicingPricingPricingType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BillingBillResourceInvoicingPricingPricingType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
