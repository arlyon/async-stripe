#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CheckoutFinancialConnectionsPaymentMethodOptions {
    pub filters: Option<stripe_shared::CheckoutFinancialConnectionsPaymentMethodOptionsFilters>,
    /// The list of permissions to request. The `payment_method` permission must be included.
    pub permissions: Option<Vec<CheckoutFinancialConnectionsPaymentMethodOptionsPermissions>>,
    /// Data features requested to be retrieved upon account creation.
    pub prefetch: Option<Vec<CheckoutFinancialConnectionsPaymentMethodOptionsPrefetch>>,
    /// For webview integrations only.
    /// Upon completing OAuth login in the native browser, the user will be redirected to this URL to return to your app.
    pub return_url: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CheckoutFinancialConnectionsPaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CheckoutFinancialConnectionsPaymentMethodOptions").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct CheckoutFinancialConnectionsPaymentMethodOptionsBuilder {
    filters: Option<Option<stripe_shared::CheckoutFinancialConnectionsPaymentMethodOptionsFilters>>,
    permissions: Option<Option<Vec<CheckoutFinancialConnectionsPaymentMethodOptionsPermissions>>>,
    prefetch: Option<Option<Vec<CheckoutFinancialConnectionsPaymentMethodOptionsPrefetch>>>,
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
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for CheckoutFinancialConnectionsPaymentMethodOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CheckoutFinancialConnectionsPaymentMethodOptions>,
        builder: CheckoutFinancialConnectionsPaymentMethodOptionsBuilder,
    }

    impl Visitor for Place<CheckoutFinancialConnectionsPaymentMethodOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CheckoutFinancialConnectionsPaymentMethodOptionsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for CheckoutFinancialConnectionsPaymentMethodOptionsBuilder {
        type Out = CheckoutFinancialConnectionsPaymentMethodOptions;
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
                filters: Some(None),
                permissions: Some(None),
                prefetch: Some(None),
                return_url: Some(None),
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

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for CheckoutFinancialConnectionsPaymentMethodOptions {
        type Builder = CheckoutFinancialConnectionsPaymentMethodOptionsBuilder;
    }

    impl FromValueOpt for CheckoutFinancialConnectionsPaymentMethodOptions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = CheckoutFinancialConnectionsPaymentMethodOptionsBuilder::deser_default();
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CheckoutFinancialConnectionsPaymentMethodOptionsPermissions {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CheckoutFinancialConnectionsPaymentMethodOptionsPermissions {
    pub fn as_str(&self) -> &str {
        use CheckoutFinancialConnectionsPaymentMethodOptionsPermissions::*;
        match self {
            Balances => "balances",
            Ownership => "ownership",
            PaymentMethod => "payment_method",
            Transactions => "transactions",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CheckoutFinancialConnectionsPaymentMethodOptionsPermissions {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutFinancialConnectionsPaymentMethodOptionsPermissions::*;
        match s {
            "balances" => Ok(Balances),
            "ownership" => Ok(Ownership),
            "payment_method" => Ok(PaymentMethod),
            "transactions" => Ok(Transactions),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CheckoutFinancialConnectionsPaymentMethodOptionsPermissions"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CheckoutFinancialConnectionsPaymentMethodOptionsPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CheckoutFinancialConnectionsPaymentMethodOptionsPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CheckoutFinancialConnectionsPaymentMethodOptionsPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CheckoutFinancialConnectionsPaymentMethodOptionsPermissions))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CheckoutFinancialConnectionsPaymentMethodOptionsPermissions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CheckoutFinancialConnectionsPaymentMethodOptionsPermissions {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<CheckoutFinancialConnectionsPaymentMethodOptionsPermissions>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            CheckoutFinancialConnectionsPaymentMethodOptionsPermissions::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    CheckoutFinancialConnectionsPaymentMethodOptionsPermissions
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CheckoutFinancialConnectionsPaymentMethodOptionsPermissions {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Data features requested to be retrieved upon account creation.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CheckoutFinancialConnectionsPaymentMethodOptionsPrefetch {
    Balances,
    Ownership,
    Transactions,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CheckoutFinancialConnectionsPaymentMethodOptionsPrefetch {
    pub fn as_str(&self) -> &str {
        use CheckoutFinancialConnectionsPaymentMethodOptionsPrefetch::*;
        match self {
            Balances => "balances",
            Ownership => "ownership",
            Transactions => "transactions",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CheckoutFinancialConnectionsPaymentMethodOptionsPrefetch {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutFinancialConnectionsPaymentMethodOptionsPrefetch::*;
        match s {
            "balances" => Ok(Balances),
            "ownership" => Ok(Ownership),
            "transactions" => Ok(Transactions),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CheckoutFinancialConnectionsPaymentMethodOptionsPrefetch"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CheckoutFinancialConnectionsPaymentMethodOptionsPrefetch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CheckoutFinancialConnectionsPaymentMethodOptionsPrefetch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CheckoutFinancialConnectionsPaymentMethodOptionsPrefetch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CheckoutFinancialConnectionsPaymentMethodOptionsPrefetch))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CheckoutFinancialConnectionsPaymentMethodOptionsPrefetch {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CheckoutFinancialConnectionsPaymentMethodOptionsPrefetch {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<CheckoutFinancialConnectionsPaymentMethodOptionsPrefetch>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            CheckoutFinancialConnectionsPaymentMethodOptionsPrefetch::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    CheckoutFinancialConnectionsPaymentMethodOptionsPrefetch
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CheckoutFinancialConnectionsPaymentMethodOptionsPrefetch {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
