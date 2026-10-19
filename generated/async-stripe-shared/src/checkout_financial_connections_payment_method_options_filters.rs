#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CheckoutFinancialConnectionsPaymentMethodOptionsFilters {
    /// The account subcategories to use to filter for possible accounts to link.
    /// Valid subcategories are `checking` and `savings`.
    pub account_subcategories: Option<Vec<stripe_shared::PaymentMethodUsBankAccountAccountType>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CheckoutFinancialConnectionsPaymentMethodOptionsFilters {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CheckoutFinancialConnectionsPaymentMethodOptionsFilters")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct CheckoutFinancialConnectionsPaymentMethodOptionsFiltersBuilder {
    account_subcategories:
        Option<Option<Vec<stripe_shared::PaymentMethodUsBankAccountAccountType>>>,
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

    impl Deserialize for CheckoutFinancialConnectionsPaymentMethodOptionsFilters {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CheckoutFinancialConnectionsPaymentMethodOptionsFilters>,
        builder: CheckoutFinancialConnectionsPaymentMethodOptionsFiltersBuilder,
    }

    impl Visitor for Place<CheckoutFinancialConnectionsPaymentMethodOptionsFilters> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder:
                    CheckoutFinancialConnectionsPaymentMethodOptionsFiltersBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for CheckoutFinancialConnectionsPaymentMethodOptionsFiltersBuilder {
        type Out = CheckoutFinancialConnectionsPaymentMethodOptionsFilters;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account_subcategories" => Deserialize::begin(&mut self.account_subcategories),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { account_subcategories: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(account_subcategories),) = (self.account_subcategories.take(),) else {
                return None;
            };
            Some(Self::Out { account_subcategories })
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

    impl ObjectDeser for CheckoutFinancialConnectionsPaymentMethodOptionsFilters {
        type Builder = CheckoutFinancialConnectionsPaymentMethodOptionsFiltersBuilder;
    }

    impl FromValueOpt for CheckoutFinancialConnectionsPaymentMethodOptionsFilters {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                CheckoutFinancialConnectionsPaymentMethodOptionsFiltersBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "account_subcategories" => {
                        b.account_subcategories = FromValueOpt::from_value(v)
                    }
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The account subcategories to use to filter for possible accounts to link.
/// Valid subcategories are `checking` and `savings`.
pub type CheckoutFinancialConnectionsPaymentMethodOptionsFiltersAccountSubcategories =
    stripe_shared::PaymentMethodUsBankAccountAccountType;
