#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxProductResourceTaxSettingsDefaults {
    /// The tax calculation provider this account uses.
    /// Defaults to `stripe` when not using a [third-party provider](/tax/third-party-apps).
    pub provider: TaxProductResourceTaxSettingsDefaultsProvider,
    /// Default [tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#tax-behavior) used to specify whether the price is considered inclusive of taxes or exclusive of taxes.
    /// If the item's price has a tax behavior set, it will take precedence over the default tax behavior.
    pub tax_behavior: Option<TaxProductResourceTaxSettingsDefaultsTaxBehavior>,
    /// Default [tax code](https://stripe.com/docs/tax/tax-categories) used to classify your products and prices.
    pub tax_code: Option<String>,
}
#[doc(hidden)]
pub struct TaxProductResourceTaxSettingsDefaultsBuilder {
    provider: Option<TaxProductResourceTaxSettingsDefaultsProvider>,
    tax_behavior: Option<Option<TaxProductResourceTaxSettingsDefaultsTaxBehavior>>,
    tax_code: Option<Option<String>>,
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
                "provider" => Deserialize::begin(&mut self.provider),
                "tax_behavior" => Deserialize::begin(&mut self.tax_behavior),
                "tax_code" => Deserialize::begin(&mut self.tax_code),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                provider: Deserialize::default(),
                tax_behavior: Deserialize::default(),
                tax_code: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(provider), Some(tax_behavior), Some(tax_code)) =
                (self.provider.take(), self.tax_behavior.take(), self.tax_code.take())
            else {
                return None;
            };
            Some(Self::Out { provider, tax_behavior, tax_code })
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
                    "provider" => b.provider = FromValueOpt::from_value(v),
                    "tax_behavior" => b.tax_behavior = FromValueOpt::from_value(v),
                    "tax_code" => b.tax_code = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The tax calculation provider this account uses.
/// Defaults to `stripe` when not using a [third-party provider](/tax/third-party-apps).
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TaxProductResourceTaxSettingsDefaultsProvider {
    Anrok,
    Avalara,
    Sphere,
    Stripe,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TaxProductResourceTaxSettingsDefaultsProvider {
    pub fn as_str(&self) -> &str {
        use TaxProductResourceTaxSettingsDefaultsProvider::*;
        match self {
            Anrok => "anrok",
            Avalara => "avalara",
            Sphere => "sphere",
            Stripe => "stripe",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TaxProductResourceTaxSettingsDefaultsProvider {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductResourceTaxSettingsDefaultsProvider::*;
        match s {
            "anrok" => Ok(Anrok),
            "avalara" => Ok(Avalara),
            "sphere" => Ok(Sphere),
            "stripe" => Ok(Stripe),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TaxProductResourceTaxSettingsDefaultsProvider"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TaxProductResourceTaxSettingsDefaultsProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxProductResourceTaxSettingsDefaultsProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TaxProductResourceTaxSettingsDefaultsProvider {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TaxProductResourceTaxSettingsDefaultsProvider {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<TaxProductResourceTaxSettingsDefaultsProvider> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(TaxProductResourceTaxSettingsDefaultsProvider::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TaxProductResourceTaxSettingsDefaultsProvider);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TaxProductResourceTaxSettingsDefaultsProvider {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Default [tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#tax-behavior) used to specify whether the price is considered inclusive of taxes or exclusive of taxes.
/// If the item's price has a tax behavior set, it will take precedence over the default tax behavior.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TaxProductResourceTaxSettingsDefaultsTaxBehavior {
    Exclusive,
    Inclusive,
    InferredByCurrency,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TaxProductResourceTaxSettingsDefaultsTaxBehavior {
    pub fn as_str(&self) -> &str {
        use TaxProductResourceTaxSettingsDefaultsTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            InferredByCurrency => "inferred_by_currency",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TaxProductResourceTaxSettingsDefaultsTaxBehavior {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductResourceTaxSettingsDefaultsTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "inferred_by_currency" => Ok(InferredByCurrency),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TaxProductResourceTaxSettingsDefaultsTaxBehavior"
                );
                Ok(Unknown(v.to_owned()))
            }
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
            TaxProductResourceTaxSettingsDefaultsTaxBehavior::from_str(s).expect("infallible"),
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
