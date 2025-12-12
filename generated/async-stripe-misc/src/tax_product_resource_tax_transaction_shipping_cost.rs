#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxProductResourceTaxTransactionShippingCost {
    /// The shipping amount in the [smallest currency unit](https://docs.stripe.com/currencies#zero-decimal).
    /// If `tax_behavior=inclusive`, then this amount includes taxes.
    /// Otherwise, taxes were calculated on top of this amount.
    pub amount: i64,
    /// The amount of tax calculated for shipping, in the [smallest currency unit](https://docs.stripe.com/currencies#zero-decimal).
    pub amount_tax: i64,
    /// The ID of an existing [ShippingRate](https://docs.stripe.com/api/shipping_rates/object).
    pub shipping_rate: Option<String>,
    /// Specifies whether the `amount` includes taxes.
    /// If `tax_behavior=inclusive`, then the amount includes taxes.
    pub tax_behavior: TaxProductResourceTaxTransactionShippingCostTaxBehavior,
    /// Detailed account of taxes relevant to shipping cost.
    /// (It is not populated for the transaction resource object and will be removed in the next API version.).
    pub tax_breakdown: Option<Vec<stripe_misc::TaxProductResourceLineItemTaxBreakdown>>,
    /// The [tax code](https://docs.stripe.com/tax/tax-categories) ID used for shipping.
    pub tax_code: String,
}
#[doc(hidden)]
pub struct TaxProductResourceTaxTransactionShippingCostBuilder {
    amount: Option<i64>,
    amount_tax: Option<i64>,
    shipping_rate: Option<Option<String>>,
    tax_behavior: Option<TaxProductResourceTaxTransactionShippingCostTaxBehavior>,
    tax_breakdown: Option<Option<Vec<stripe_misc::TaxProductResourceLineItemTaxBreakdown>>>,
    tax_code: Option<String>,
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

    impl Deserialize for TaxProductResourceTaxTransactionShippingCost {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductResourceTaxTransactionShippingCost>,
        builder: TaxProductResourceTaxTransactionShippingCostBuilder,
    }

    impl Visitor for Place<TaxProductResourceTaxTransactionShippingCost> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TaxProductResourceTaxTransactionShippingCostBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TaxProductResourceTaxTransactionShippingCostBuilder {
        type Out = TaxProductResourceTaxTransactionShippingCost;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "amount_tax" => Deserialize::begin(&mut self.amount_tax),
                "shipping_rate" => Deserialize::begin(&mut self.shipping_rate),
                "tax_behavior" => Deserialize::begin(&mut self.tax_behavior),
                "tax_breakdown" => Deserialize::begin(&mut self.tax_breakdown),
                "tax_code" => Deserialize::begin(&mut self.tax_code),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                amount_tax: Deserialize::default(),
                shipping_rate: Deserialize::default(),
                tax_behavior: Deserialize::default(),
                tax_breakdown: Deserialize::default(),
                tax_code: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(amount),
                Some(amount_tax),
                Some(shipping_rate),
                Some(tax_behavior),
                Some(tax_breakdown),
                Some(tax_code),
            ) = (
                self.amount,
                self.amount_tax,
                self.shipping_rate.take(),
                self.tax_behavior.take(),
                self.tax_breakdown.take(),
                self.tax_code.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                amount,
                amount_tax,
                shipping_rate,
                tax_behavior,
                tax_breakdown,
                tax_code,
            })
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

    impl ObjectDeser for TaxProductResourceTaxTransactionShippingCost {
        type Builder = TaxProductResourceTaxTransactionShippingCostBuilder;
    }

    impl FromValueOpt for TaxProductResourceTaxTransactionShippingCost {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TaxProductResourceTaxTransactionShippingCostBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "amount_tax" => b.amount_tax = FromValueOpt::from_value(v),
                    "shipping_rate" => b.shipping_rate = FromValueOpt::from_value(v),
                    "tax_behavior" => b.tax_behavior = FromValueOpt::from_value(v),
                    "tax_breakdown" => b.tax_breakdown = FromValueOpt::from_value(v),
                    "tax_code" => b.tax_code = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Specifies whether the `amount` includes taxes.
/// If `tax_behavior=inclusive`, then the amount includes taxes.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TaxProductResourceTaxTransactionShippingCostTaxBehavior {
    Exclusive,
    Inclusive,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TaxProductResourceTaxTransactionShippingCostTaxBehavior {
    pub fn as_str(&self) -> &str {
        use TaxProductResourceTaxTransactionShippingCostTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TaxProductResourceTaxTransactionShippingCostTaxBehavior {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductResourceTaxTransactionShippingCostTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TaxProductResourceTaxTransactionShippingCostTaxBehavior"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TaxProductResourceTaxTransactionShippingCostTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxProductResourceTaxTransactionShippingCostTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TaxProductResourceTaxTransactionShippingCostTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TaxProductResourceTaxTransactionShippingCostTaxBehavior {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<TaxProductResourceTaxTransactionShippingCostTaxBehavior>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TaxProductResourceTaxTransactionShippingCostTaxBehavior::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TaxProductResourceTaxTransactionShippingCostTaxBehavior);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TaxProductResourceTaxTransactionShippingCostTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
