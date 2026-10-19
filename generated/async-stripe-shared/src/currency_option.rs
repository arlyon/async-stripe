#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CurrencyOption {
    /// When set, provides configuration for the amount to be adjusted by the customer during Checkout Sessions and Payment Links.
    pub custom_unit_amount: Option<stripe_shared::CustomUnitAmount>,
    /// Only required if a [default tax behavior](https://docs.stripe.com/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    pub tax_behavior: Option<stripe_shared::PriceTaxBehavior>,
    /// Each element represents a pricing tier.
    /// This parameter requires `billing_scheme` to be set to `tiered`.
    /// See also the documentation for `billing_scheme`.
    pub tiers: Option<Vec<stripe_shared::PriceTier>>,
    /// The unit amount in cents (or local equivalent) to be charged, represented as a whole integer if possible.
    /// Only set if `billing_scheme=per_unit`.
    pub unit_amount: Option<i64>,
    /// The unit amount in cents (or local equivalent) to be charged, represented as a decimal string with at most 12 decimal places.
    /// Only set if `billing_scheme=per_unit`.
    pub unit_amount_decimal: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CurrencyOption {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CurrencyOption").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct CurrencyOptionBuilder {
    custom_unit_amount: Option<Option<stripe_shared::CustomUnitAmount>>,
    tax_behavior: Option<Option<stripe_shared::PriceTaxBehavior>>,
    tiers: Option<Option<Vec<stripe_shared::PriceTier>>>,
    unit_amount: Option<Option<i64>>,
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

    impl Deserialize for CurrencyOption {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CurrencyOption>,
        builder: CurrencyOptionBuilder,
    }

    impl Visitor for Place<CurrencyOption> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CurrencyOptionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for CurrencyOptionBuilder {
        type Out = CurrencyOption;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "custom_unit_amount" => Deserialize::begin(&mut self.custom_unit_amount),
                "tax_behavior" => Deserialize::begin(&mut self.tax_behavior),
                "tiers" => Deserialize::begin(&mut self.tiers),
                "unit_amount" => Deserialize::begin(&mut self.unit_amount),
                "unit_amount_decimal" => Deserialize::begin(&mut self.unit_amount_decimal),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                custom_unit_amount: Some(None),
                tax_behavior: Some(None),
                tiers: Some(None),
                unit_amount: Some(None),
                unit_amount_decimal: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(custom_unit_amount),
                Some(tax_behavior),
                Some(tiers),
                Some(unit_amount),
                Some(unit_amount_decimal),
            ) = (
                self.custom_unit_amount,
                self.tax_behavior.take(),
                self.tiers.take(),
                self.unit_amount,
                self.unit_amount_decimal.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                custom_unit_amount,
                tax_behavior,
                tiers,
                unit_amount,
                unit_amount_decimal,
            })
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

    impl ObjectDeser for CurrencyOption {
        type Builder = CurrencyOptionBuilder;
    }

    impl FromValueOpt for CurrencyOption {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = CurrencyOptionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "custom_unit_amount" => b.custom_unit_amount = FromValueOpt::from_value(v),
                    "tax_behavior" => b.tax_behavior = FromValueOpt::from_value(v),
                    "tiers" => b.tiers = FromValueOpt::from_value(v),
                    "unit_amount" => b.unit_amount = FromValueOpt::from_value(v),
                    "unit_amount_decimal" => b.unit_amount_decimal = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Only required if a [default tax behavior](https://docs.stripe.com/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
pub type CurrencyOptionTaxBehavior = stripe_shared::PriceTaxBehavior;
