#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFilters {
        /// The account subcategories to use to filter for possible accounts to link.
    /// Valid subcategories are `checking` and `savings`.
pub account_subcategories: Option<Vec<PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFiltersAccountSubcategories>>,

}
#[doc(hidden)]
pub struct PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFiltersBuilder {
    account_subcategories: Option<Option<Vec<PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFiltersAccountSubcategories>>>,

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

    impl Deserialize
        for PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFilters
    {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
    out: &'a mut Option<PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFilters>,
    builder: PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFiltersBuilder,
}

    impl Visitor
        for Place<
            PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFilters,
        >
    {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFiltersBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFiltersBuilder {
    type Out = PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFilters;
    fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
        Ok(match k {
            "account_subcategories" => Deserialize::begin(&mut self.account_subcategories),
            _ => <dyn Visitor>::ignore(),
        })
    }

    fn deser_default() -> Self {
        Self { account_subcategories: Deserialize::default(),
 }
    }

    fn take_out(&mut self) -> Option<Self::Out> {
        let (Some(account_subcategories),
) = (self.account_subcategories.take(),
) else {
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
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser
        for PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFilters
    {
        type Builder = PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFiltersBuilder;
    }

    impl FromValueOpt
        for PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFilters
    {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFiltersBuilder::deser_default();
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFiltersAccountSubcategories
{
    Checking,
    Savings,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFiltersAccountSubcategories {
    pub fn as_str(&self) -> &str {
        use PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFiltersAccountSubcategories::*;
        match self {
Checking => "checking",
Savings => "savings",
Unknown(v) => v,

        }
    }
}

impl std::str::FromStr for PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFiltersAccountSubcategories {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFiltersAccountSubcategories::*;
        match s {
    "checking" => Ok(Checking),
"savings" => Ok(Savings),
v => { tracing::warn!("Unknown value '{}' for enum '{}'", v, "PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFiltersAccountSubcategories"); Ok(Unknown(v.to_owned())) }

        }
    }
}
impl std::fmt::Display for PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFiltersAccountSubcategories {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFiltersAccountSubcategories {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFiltersAccountSubcategories {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFiltersAccountSubcategories {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFiltersAccountSubcategories> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFiltersAccountSubcategories::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFiltersAccountSubcategories);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFiltersAccountSubcategories {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
