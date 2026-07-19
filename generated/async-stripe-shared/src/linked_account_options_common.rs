#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for LinkedAccountOptionsCommon {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("LinkedAccountOptionsCommon").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: LinkedAccountOptionsCommonBuilder {
                    filters: Deserialize::default(),
                    permissions: Deserialize::default(),
                    prefetch: Deserialize::default(),
                    return_url: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "filters" => Deserialize::begin(&mut self.builder.filters),
                "permissions" => Deserialize::begin(&mut self.builder.permissions),
                "prefetch" => Deserialize::begin(&mut self.builder.prefetch),
                "return_url" => Deserialize::begin(&mut self.builder.return_url),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(filters), Some(permissions), Some(prefetch), Some(return_url)) = (
                self.builder.filters.take(),
                self.builder.permissions.take(),
                self.builder.prefetch.take(),
                self.builder.return_url.take(),
            ) else {
                return Ok(());
            };
            *self.out =
                Some(LinkedAccountOptionsCommon { filters, permissions, prefetch, return_url });
            Ok(())
        }
    }
};
/// The list of permissions to request. The `payment_method` permission must be included.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum LinkedAccountOptionsCommonPermissions {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl LinkedAccountOptionsCommonPermissions {
    pub fn as_str(&self) -> &str {
        use LinkedAccountOptionsCommonPermissions::*;
        match self {
            Balances => "balances",
            Ownership => "ownership",
            PaymentMethod => "payment_method",
            Transactions => "transactions",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for LinkedAccountOptionsCommonPermissions {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use LinkedAccountOptionsCommonPermissions::*;
        match s {
            "balances" => Ok(Balances),
            "ownership" => Ok(Ownership),
            "payment_method" => Ok(PaymentMethod),
            "transactions" => Ok(Transactions),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "LinkedAccountOptionsCommonPermissions"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for LinkedAccountOptionsCommonPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for LinkedAccountOptionsCommonPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for LinkedAccountOptionsCommonPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(LinkedAccountOptionsCommonPermissions)).finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for LinkedAccountOptionsCommonPermissions {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<LinkedAccountOptionsCommonPermissions> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(LinkedAccountOptionsCommonPermissions::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for LinkedAccountOptionsCommonPermissions {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Data features requested to be retrieved upon account creation.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum LinkedAccountOptionsCommonPrefetch {
    Balances,
    Ownership,
    Transactions,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl LinkedAccountOptionsCommonPrefetch {
    pub fn as_str(&self) -> &str {
        use LinkedAccountOptionsCommonPrefetch::*;
        match self {
            Balances => "balances",
            Ownership => "ownership",
            Transactions => "transactions",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for LinkedAccountOptionsCommonPrefetch {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use LinkedAccountOptionsCommonPrefetch::*;
        match s {
            "balances" => Ok(Balances),
            "ownership" => Ok(Ownership),
            "transactions" => Ok(Transactions),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "LinkedAccountOptionsCommonPrefetch"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for LinkedAccountOptionsCommonPrefetch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for LinkedAccountOptionsCommonPrefetch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for LinkedAccountOptionsCommonPrefetch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(LinkedAccountOptionsCommonPrefetch)).finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for LinkedAccountOptionsCommonPrefetch {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<LinkedAccountOptionsCommonPrefetch> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(LinkedAccountOptionsCommonPrefetch::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for LinkedAccountOptionsCommonPrefetch {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
