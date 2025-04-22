#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct LinkedAccountOptionsCommon {
pub filters: Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFilters>,
    /// The list of permissions to request. The `payment_method` permission must be included.
pub permissions: Option<Vec<LinkedAccountOptionsCommonPermissions>>,
    /// Data features requested to be retrieved upon account creation.
pub prefetch: Option<Vec<LinkedAccountOptionsCommonPrefetch>>,
        /// For webview integrations only.
    /// Upon completing OAuth login in the native browser, the user will be redirected to this URL to return to your app.
pub return_url: Option<String>,

}
#[doc(hidden)]
pub struct LinkedAccountOptionsCommonBuilder {
    filters: Option<Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFilters>>,
permissions: Option<Option<Vec<LinkedAccountOptionsCommonPermissions>>>,
prefetch: Option<Option<Vec<LinkedAccountOptionsCommonPrefetch>>>,
return_url: Option<Option<String>>,

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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for LinkedAccountOptionsCommon {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<LinkedAccountOptionsCommon>,
        builder: LinkedAccountOptionsCommonBuilder,
    }

    impl Visitor for Place<LinkedAccountOptionsCommon> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: LinkedAccountOptionsCommonBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for LinkedAccountOptionsCommonBuilder {
        type Out = LinkedAccountOptionsCommon;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "filters" => Deserialize::begin(&mut self.filters),
                "permissions" => Deserialize::begin(&mut self.permissions),
                "prefetch" => Deserialize::begin(&mut self.prefetch),
                "return_url" => Deserialize::begin(&mut self.return_url),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                filters: Deserialize::default(),
                permissions: Deserialize::default(),
                prefetch: Deserialize::default(),
                return_url: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(filters), Some(permissions), Some(prefetch), Some(return_url)) = (
                self.filters.take(),
                self.permissions.take(),
                self.prefetch.take(),
                self.return_url.take(),
            ) else {
                return None;
            };
            Some(Self::Out { filters, permissions, prefetch, return_url })
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

    impl ObjectDeser for LinkedAccountOptionsCommon {
        type Builder = LinkedAccountOptionsCommonBuilder;
    }

    impl FromValueOpt for LinkedAccountOptionsCommon {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = LinkedAccountOptionsCommonBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "filters" => b.filters = FromValueOpt::from_value(v),
                    "permissions" => b.permissions = FromValueOpt::from_value(v),
                    "prefetch" => b.prefetch = FromValueOpt::from_value(v),
                    "return_url" => b.return_url = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The list of permissions to request. The `payment_method` permission must be included.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum LinkedAccountOptionsCommonPermissions {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}
impl LinkedAccountOptionsCommonPermissions {
    pub fn as_str(self) -> &'static str {
        use LinkedAccountOptionsCommonPermissions::*;
        match self {
            Balances => "balances",
            Ownership => "ownership",
            PaymentMethod => "payment_method",
            Transactions => "transactions",
        }
    }
}

impl std::str::FromStr for LinkedAccountOptionsCommonPermissions {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use LinkedAccountOptionsCommonPermissions::*;
        match s {
            "balances" => Ok(Balances),
            "ownership" => Ok(Ownership),
            "payment_method" => Ok(PaymentMethod),
            "transactions" => Ok(Transactions),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for LinkedAccountOptionsCommonPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for LinkedAccountOptionsCommonPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for LinkedAccountOptionsCommonPermissions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for LinkedAccountOptionsCommonPermissions {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<LinkedAccountOptionsCommonPermissions> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(LinkedAccountOptionsCommonPermissions::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(LinkedAccountOptionsCommonPermissions);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for LinkedAccountOptionsCommonPermissions {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for LinkedAccountOptionsCommonPermissions")
        })
    }
}
/// Data features requested to be retrieved upon account creation.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum LinkedAccountOptionsCommonPrefetch {
    Balances,
    Ownership,
    Transactions,
}
impl LinkedAccountOptionsCommonPrefetch {
    pub fn as_str(self) -> &'static str {
        use LinkedAccountOptionsCommonPrefetch::*;
        match self {
            Balances => "balances",
            Ownership => "ownership",
            Transactions => "transactions",
        }
    }
}

impl std::str::FromStr for LinkedAccountOptionsCommonPrefetch {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use LinkedAccountOptionsCommonPrefetch::*;
        match s {
            "balances" => Ok(Balances),
            "ownership" => Ok(Ownership),
            "transactions" => Ok(Transactions),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for LinkedAccountOptionsCommonPrefetch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for LinkedAccountOptionsCommonPrefetch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for LinkedAccountOptionsCommonPrefetch {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for LinkedAccountOptionsCommonPrefetch {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<LinkedAccountOptionsCommonPrefetch> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(LinkedAccountOptionsCommonPrefetch::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(LinkedAccountOptionsCommonPrefetch);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for LinkedAccountOptionsCommonPrefetch {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for LinkedAccountOptionsCommonPrefetch")
        })
    }
}
