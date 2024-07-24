#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxProductResourceTaxSettingsDefaults {
    /// Default [tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#tax-behavior) used to specify whether the price is considered inclusive of taxes or exclusive of taxes.
    /// If the item's price has a tax behavior set, it will take precedence over the default tax behavior.
    pub tax_behavior: Option<TaxProductResourceTaxSettingsDefaultsTaxBehavior>,
    /// Default [tax code](https://stripe.com/docs/tax/tax-categories) used to classify your products and prices.
    pub tax_code: Option<String>,
}
#[doc(hidden)]
pub struct TaxProductResourceTaxSettingsDefaultsBuilder {
    tax_behavior: Option<Option<TaxProductResourceTaxSettingsDefaultsTaxBehavior>>,
    tax_code: Option<Option<String>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TaxProductResourceTaxSettingsDefaults {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductResourceTaxSettingsDefaults>,
        builder: TaxProductResourceTaxSettingsDefaultsBuilder,
    }

    impl Visitor for Place<TaxProductResourceTaxSettingsDefaults> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TaxProductResourceTaxSettingsDefaultsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TaxProductResourceTaxSettingsDefaultsBuilder {
        type Out = TaxProductResourceTaxSettingsDefaults;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "tax_behavior" => Deserialize::begin(&mut self.tax_behavior),
                "tax_code" => Deserialize::begin(&mut self.tax_code),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { tax_behavior: Deserialize::default(), tax_code: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out { tax_behavior: self.tax_behavior?, tax_code: self.tax_code.take()? })
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

    impl ObjectDeser for TaxProductResourceTaxSettingsDefaults {
        type Builder = TaxProductResourceTaxSettingsDefaultsBuilder;
    }

    impl FromValueOpt for TaxProductResourceTaxSettingsDefaults {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TaxProductResourceTaxSettingsDefaultsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "tax_behavior" => b.tax_behavior = Some(FromValueOpt::from_value(v)?),
                    "tax_code" => b.tax_code = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Default [tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#tax-behavior) used to specify whether the price is considered inclusive of taxes or exclusive of taxes.
/// If the item's price has a tax behavior set, it will take precedence over the default tax behavior.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxProductResourceTaxSettingsDefaultsTaxBehavior {
    Exclusive,
    Inclusive,
    InferredByCurrency,
}
impl TaxProductResourceTaxSettingsDefaultsTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use TaxProductResourceTaxSettingsDefaultsTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            InferredByCurrency => "inferred_by_currency",
        }
    }
}

impl std::str::FromStr for TaxProductResourceTaxSettingsDefaultsTaxBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductResourceTaxSettingsDefaultsTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "inferred_by_currency" => Ok(InferredByCurrency),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for TaxProductResourceTaxSettingsDefaultsTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxProductResourceTaxSettingsDefaultsTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TaxProductResourceTaxSettingsDefaultsTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TaxProductResourceTaxSettingsDefaultsTaxBehavior {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<TaxProductResourceTaxSettingsDefaultsTaxBehavior> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TaxProductResourceTaxSettingsDefaultsTaxBehavior::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TaxProductResourceTaxSettingsDefaultsTaxBehavior);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TaxProductResourceTaxSettingsDefaultsTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for TaxProductResourceTaxSettingsDefaultsTaxBehavior",
            )
        })
    }
}
