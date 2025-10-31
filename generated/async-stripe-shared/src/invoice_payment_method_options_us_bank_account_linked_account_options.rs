#[derive(Clone, Debug)]
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
            builder: InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsBuilder {
        type Out = InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "filters" => Deserialize::begin(&mut self.filters),
                "permissions" => Deserialize::begin(&mut self.permissions),
                "prefetch" => Deserialize::begin(&mut self.prefetch),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                filters: Deserialize::default(),
                permissions: Deserialize::default(),
                prefetch: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(filters), Some(permissions), Some(prefetch)) =
                (self.filters.take(), self.permissions.take(), self.prefetch.take())
            else {
                return None;
            };
            Some(Self::Out { filters, permissions, prefetch })
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

    impl ObjectDeser for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptions {
        type Builder = InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsBuilder;
    }

    impl FromValueOpt for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsBuilder::deser_default(
                );
            for (k, v) in obj {
                match k.as_str() {
                    "filters" => b.filters = FromValueOpt::from_value(v),
                    "permissions" => b.permissions = FromValueOpt::from_value(v),
                    "prefetch" => b.prefetch = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The list of permissions to request. The `payment_method` permission must be included.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}
impl InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions {
    pub fn as_str(self) -> &'static str {
        use InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions::*;
        match self {
            Balances => "balances",
            Ownership => "ownership",
            PaymentMethod => "payment_method",
            Transactions => "transactions",
        }
    }
}

impl std::str::FromStr for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions::*;
        match s {
            "balances" => Ok(Balances),
            "ownership" => Ok(Ownership),
            "payment_method" => Ok(PaymentMethod),
            "transactions" => Ok(Transactions),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize
    for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissions"))
    }
}
/// Data features requested to be retrieved upon account creation.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch {
    Balances,
    Ownership,
    Transactions,
}
impl InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch {
    pub fn as_str(self) -> &'static str {
        use InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch::*;
        match self {
            Balances => "balances",
            Ownership => "ownership",
            Transactions => "transactions",
        }
    }
}

impl std::str::FromStr for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch::*;
        match s {
            "balances" => Ok(Balances),
            "ownership" => Ok(Ownership),
            "transactions" => Ok(Transactions),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize
    for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetch"))
    }
}
