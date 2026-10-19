#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxProductResourceTaxSettingsDefaults {
    /// The tax calculation provider this account uses.
    /// Defaults to `stripe` when not using a [third-party provider](/tax/third-party-apps).
    pub provider: stripe_shared::Provider,
    /// Default [tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#tax-behavior) used to specify whether the price is considered inclusive of taxes or exclusive of taxes.
    /// If the item's price has a tax behavior set, it will take precedence over the default tax behavior.
    pub tax_behavior: Option<stripe_shared::UpdateTaxSettingsDefaultsTaxBehavior>,
    /// Default [tax code](https://stripe.com/docs/tax/tax-categories) used to classify your products and prices.
    pub tax_code: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxProductResourceTaxSettingsDefaults {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TaxProductResourceTaxSettingsDefaults").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TaxProductResourceTaxSettingsDefaultsBuilder {
    provider: Option<stripe_shared::Provider>,
    tax_behavior: Option<Option<stripe_shared::UpdateTaxSettingsDefaultsTaxBehavior>>,
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
            Self { provider: None, tax_behavior: Some(None), tax_code: Some(None) }
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
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
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
pub type TaxProductResourceTaxSettingsDefaultsProvider = stripe_shared::Provider;
/// Default [tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#tax-behavior) used to specify whether the price is considered inclusive of taxes or exclusive of taxes.
/// If the item's price has a tax behavior set, it will take precedence over the default tax behavior.
pub type TaxProductResourceTaxSettingsDefaultsTaxBehavior =
    stripe_shared::UpdateTaxSettingsDefaultsTaxBehavior;
