#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptions {
    pub filters:
        Option<stripe_shared::InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsFilters>,
    /// The list of permissions to request. The `payment_method` permission must be included.
    pub permissions:
        Option<Vec<InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions>>,
    /// Data features requested to be retrieved upon account creation.
    pub prefetch: Option<Vec<InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptions")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsBuilder {
    filters: Option<
        Option<stripe_shared::InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsFilters>,
    >,
    permissions: Option<
        Option<Vec<InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions>>,
    >,
    prefetch:
        Option<Option<Vec<InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch>>>,
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

    impl Deserialize for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptions>,
        builder: InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsBuilder,
    }

    impl Visitor for Place<InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsBuilder {
                    filters: Deserialize::default(),
                    permissions: Deserialize::default(),
                    prefetch: Deserialize::default(),
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
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(filters), Some(permissions), Some(prefetch)) = (
                self.builder.filters.take(),
                self.builder.permissions.take(),
                self.builder.prefetch.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptions {
                filters,
                permissions,
                prefetch,
            });
            Ok(())
        }
    }
};
/// The list of permissions to request. The `payment_method` permission must be included.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions {
    pub fn as_str(&self) -> &str {
        use InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions::*;
        match self {
            Balances => "balances",
            Ownership => "ownership",
            PaymentMethod => "payment_method",
            Transactions => "transactions",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions::*;
        match s {
            "balances" => Ok(Balances),
            "ownership" => Ok(Ownership),
            "payment_method" => Ok(PaymentMethod),
            "transactions" => Ok(Transactions),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(
            InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions
        ))
        .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize
    for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions
{
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Data features requested to be retrieved upon account creation.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch {
    Balances,
    Ownership,
    Transactions,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch {
    pub fn as_str(&self) -> &str {
        use InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch::*;
        match self {
            Balances => "balances",
            Ownership => "ownership",
            Transactions => "transactions",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch::*;
        match s {
            "balances" => Ok(Balances),
            "ownership" => Ok(Ownership),
            "transactions" => Ok(Transactions),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(
            InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch
        ))
        .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize
    for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch
{
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
