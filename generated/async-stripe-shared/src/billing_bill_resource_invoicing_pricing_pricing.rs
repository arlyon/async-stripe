#[derive(Clone, Debug)]
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
                builder: BillingBillResourceInvoicingPricingPricingBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BillingBillResourceInvoicingPricingPricingBuilder {
        type Out = BillingBillResourceInvoicingPricingPricing;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "price_details" => Deserialize::begin(&mut self.price_details),
                "type" => Deserialize::begin(&mut self.type_),
                "unit_amount_decimal" => Deserialize::begin(&mut self.unit_amount_decimal),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                price_details: Deserialize::default(),
                type_: Deserialize::default(),
                unit_amount_decimal: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(price_details), Some(type_), Some(unit_amount_decimal)) =
                (self.price_details.take(), self.type_, self.unit_amount_decimal.take())
            else {
                return None;
            };
            Some(Self::Out { price_details, type_, unit_amount_decimal })
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

    impl ObjectDeser for BillingBillResourceInvoicingPricingPricing {
        type Builder = BillingBillResourceInvoicingPricingPricingBuilder;
    }

    impl FromValueOpt for BillingBillResourceInvoicingPricingPricing {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BillingBillResourceInvoicingPricingPricingBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "price_details" => b.price_details = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    "unit_amount_decimal" => b.unit_amount_decimal = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The type of the pricing details.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BillingBillResourceInvoicingPricingPricingType {
    PriceDetails,
}
impl BillingBillResourceInvoicingPricingPricingType {
    pub fn as_str(self) -> &'static str {
        use BillingBillResourceInvoicingPricingPricingType::*;
        match self {
            PriceDetails => "price_details",
        }
    }
}

impl std::str::FromStr for BillingBillResourceInvoicingPricingPricingType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BillingBillResourceInvoicingPricingPricingType::*;
        match s {
            "price_details" => Ok(PriceDetails),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for BillingBillResourceInvoicingPricingPricingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for BillingBillResourceInvoicingPricingPricingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for BillingBillResourceInvoicingPricingPricingType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<BillingBillResourceInvoicingPricingPricingType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            BillingBillResourceInvoicingPricingPricingType::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(BillingBillResourceInvoicingPricingPricingType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BillingBillResourceInvoicingPricingPricingType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for BillingBillResourceInvoicingPricingPricingType",
            )
        })
    }
}
