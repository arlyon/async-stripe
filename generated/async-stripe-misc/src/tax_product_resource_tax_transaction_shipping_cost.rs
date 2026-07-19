#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxProductResourceTaxTransactionShippingCost {
    /// The shipping amount in the [smallest currency unit](https://docs.stripe.com/currencies#minor-units).
    /// If `tax_behavior=inclusive`, then this amount includes taxes.
    /// Otherwise, taxes were calculated on top of this amount.
    pub amount: i64,
    /// The amount of tax calculated for shipping, in the [smallest currency unit](https://docs.stripe.com/currencies#minor-units).
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxProductResourceTaxTransactionShippingCost {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TaxProductResourceTaxTransactionShippingCost").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: TaxProductResourceTaxTransactionShippingCostBuilder {
                    amount: Deserialize::default(),
                    amount_tax: Deserialize::default(),
                    shipping_rate: Deserialize::default(),
                    tax_behavior: Deserialize::default(),
                    tax_breakdown: Deserialize::default(),
                    tax_code: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.builder.amount),
                "amount_tax" => Deserialize::begin(&mut self.builder.amount_tax),
                "shipping_rate" => Deserialize::begin(&mut self.builder.shipping_rate),
                "tax_behavior" => Deserialize::begin(&mut self.builder.tax_behavior),
                "tax_breakdown" => Deserialize::begin(&mut self.builder.tax_breakdown),
                "tax_code" => Deserialize::begin(&mut self.builder.tax_code),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(amount),
                Some(amount_tax),
                Some(shipping_rate),
                Some(tax_behavior),
                Some(tax_breakdown),
                Some(tax_code),
            ) = (
                self.builder.amount,
                self.builder.amount_tax,
                self.builder.shipping_rate.take(),
                self.builder.tax_behavior.take(),
                self.builder.tax_breakdown.take(),
                self.builder.tax_code.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(TaxProductResourceTaxTransactionShippingCost {
                amount,
                amount_tax,
                shipping_rate,
                tax_behavior,
                tax_breakdown,
                tax_code,
            });
            Ok(())
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

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for TaxProductResourceTaxTransactionShippingCostTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxProductResourceTaxTransactionShippingCostTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(TaxProductResourceTaxTransactionShippingCostTaxBehavior))
            .finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for TaxProductResourceTaxTransactionShippingCostTaxBehavior {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<TaxProductResourceTaxTransactionShippingCostTaxBehavior>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TaxProductResourceTaxTransactionShippingCostTaxBehavior::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TaxProductResourceTaxTransactionShippingCostTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
