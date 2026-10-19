#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BankConnectionsResourceLinkAccountSessionFilters {
    /// Restricts the Session to subcategories of accounts that can be linked.
    /// Valid subcategories are: `checking`, `savings`, `mortgage`, `line_of_credit`, `credit_card`.
    pub account_subcategories: Option<Vec<stripe_shared::AccountSubcategories>>,
    /// List of countries from which to filter accounts.
    pub countries: Option<Vec<String>>,
    /// Country from which to filter accounts.
    pub country: Option<String>,
    /// Whether the Session should require that linked accounts support payments and retrieve account numbers before completion.
    pub require_payment_method_support: Option<stripe_shared::RequirePaymentMethodSupport>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BankConnectionsResourceLinkAccountSessionFilters {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BankConnectionsResourceLinkAccountSessionFilters").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct BankConnectionsResourceLinkAccountSessionFiltersBuilder {
    account_subcategories: Option<Option<Vec<stripe_shared::AccountSubcategories>>>,
    countries: Option<Option<Vec<String>>>,
    country: Option<Option<String>>,
    require_payment_method_support: Option<Option<stripe_shared::RequirePaymentMethodSupport>>,
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

    impl Deserialize for BankConnectionsResourceLinkAccountSessionFilters {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BankConnectionsResourceLinkAccountSessionFilters>,
        builder: BankConnectionsResourceLinkAccountSessionFiltersBuilder,
    }

    impl Visitor for Place<BankConnectionsResourceLinkAccountSessionFilters> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BankConnectionsResourceLinkAccountSessionFiltersBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BankConnectionsResourceLinkAccountSessionFiltersBuilder {
        type Out = BankConnectionsResourceLinkAccountSessionFilters;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account_subcategories" => Deserialize::begin(&mut self.account_subcategories),
                "countries" => Deserialize::begin(&mut self.countries),
                "country" => Deserialize::begin(&mut self.country),
                "require_payment_method_support" => {
                    Deserialize::begin(&mut self.require_payment_method_support)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                account_subcategories: Some(None),
                countries: Some(None),
                country: Some(None),
                require_payment_method_support: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(account_subcategories),
                Some(countries),
                Some(country),
                Some(require_payment_method_support),
            ) = (
                self.account_subcategories.take(),
                self.countries.take(),
                self.country.take(),
                self.require_payment_method_support.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                account_subcategories,
                countries,
                country,
                require_payment_method_support,
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

    impl ObjectDeser for BankConnectionsResourceLinkAccountSessionFilters {
        type Builder = BankConnectionsResourceLinkAccountSessionFiltersBuilder;
    }

    impl FromValueOpt for BankConnectionsResourceLinkAccountSessionFilters {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BankConnectionsResourceLinkAccountSessionFiltersBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "account_subcategories" => {
                        b.account_subcategories = FromValueOpt::from_value(v)
                    }
                    "countries" => b.countries = FromValueOpt::from_value(v),
                    "country" => b.country = FromValueOpt::from_value(v),
                    "require_payment_method_support" => {
                        b.require_payment_method_support = FromValueOpt::from_value(v)
                    }
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Restricts the Session to subcategories of accounts that can be linked.
/// Valid subcategories are: `checking`, `savings`, `mortgage`, `line_of_credit`, `credit_card`.
pub type BankConnectionsResourceLinkAccountSessionFiltersAccountSubcategories =
    stripe_shared::AccountSubcategories;
/// Whether the Session should require that linked accounts support payments and retrieve account numbers before completion.
pub type BankConnectionsResourceLinkAccountSessionFiltersRequirePaymentMethodSupport =
    stripe_shared::RequirePaymentMethodSupport;
