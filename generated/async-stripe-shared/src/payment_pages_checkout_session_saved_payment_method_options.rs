#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionSavedPaymentMethodOptions {
    /// Uses the `allow_redisplay` value of each saved payment method to filter the set presented to a returning customer.
    /// By default, only saved payment methods with ’allow_redisplay: ‘always’ are shown in Checkout.
    pub allow_redisplay_filters:
        Option<Vec<PaymentPagesCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFilters>>,
    /// Enable customers to choose if they wish to remove their saved payment methods. Disabled by default.
    pub payment_method_remove:
        Option<PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodRemove>,
    /// Enable customers to choose if they wish to save their payment method for future use.
    /// Disabled by default.
    pub payment_method_save:
        Option<PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodSave>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentPagesCheckoutSessionSavedPaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentPagesCheckoutSessionSavedPaymentMethodOptions")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentPagesCheckoutSessionSavedPaymentMethodOptionsBuilder {
    allow_redisplay_filters: Option<
        Option<Vec<PaymentPagesCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFilters>>,
    >,
    payment_method_remove:
        Option<Option<PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodRemove>>,
    payment_method_save:
        Option<Option<PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodSave>>,
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

    impl Deserialize for PaymentPagesCheckoutSessionSavedPaymentMethodOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentPagesCheckoutSessionSavedPaymentMethodOptions>,
        builder: PaymentPagesCheckoutSessionSavedPaymentMethodOptionsBuilder,
    }

    impl Visitor for Place<PaymentPagesCheckoutSessionSavedPaymentMethodOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentPagesCheckoutSessionSavedPaymentMethodOptionsBuilder {
                    allow_redisplay_filters: Deserialize::default(),
                    payment_method_remove: Deserialize::default(),
                    payment_method_save: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "allow_redisplay_filters" => {
                    Deserialize::begin(&mut self.builder.allow_redisplay_filters)
                }
                "payment_method_remove" => {
                    Deserialize::begin(&mut self.builder.payment_method_remove)
                }
                "payment_method_save" => Deserialize::begin(&mut self.builder.payment_method_save),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(allow_redisplay_filters),
                Some(payment_method_remove),
                Some(payment_method_save),
            ) = (
                self.builder.allow_redisplay_filters.take(),
                self.builder.payment_method_remove.take(),
                self.builder.payment_method_save.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(PaymentPagesCheckoutSessionSavedPaymentMethodOptions {
                allow_redisplay_filters,
                payment_method_remove,
                payment_method_save,
            });
            Ok(())
        }
    }
};
/// Uses the `allow_redisplay` value of each saved payment method to filter the set presented to a returning customer.
/// By default, only saved payment methods with ’allow_redisplay: ‘always’ are shown in Checkout.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentPagesCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFilters {
    Always,
    Limited,
    Unspecified,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentPagesCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFilters {
    pub fn as_str(&self) -> &str {
        use PaymentPagesCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFilters::*;
        match self {
            Always => "always",
            Limited => "limited",
            Unspecified => "unspecified",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for PaymentPagesCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFilters
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentPagesCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFilters::*;
        match s {
            "always" => Ok(Always),
            "limited" => Ok(Limited),
            "unspecified" => Ok(Unspecified),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentPagesCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFilters"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display
    for PaymentPagesCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFilters
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PaymentPagesCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFilters {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentPagesCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFilters {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(
            PaymentPagesCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFilters
        ))
        .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize
    for PaymentPagesCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFilters
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize
    for PaymentPagesCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFilters
{
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<PaymentPagesCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFilters>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentPagesCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFilters::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for PaymentPagesCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFilters
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Enable customers to choose if they wish to remove their saved payment methods. Disabled by default.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodRemove {
    Disabled,
    Enabled,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodRemove {
    pub fn as_str(&self) -> &str {
        use PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodRemove::*;
        match self {
            Disabled => "disabled",
            Enabled => "enabled",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodRemove {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodRemove::*;
        match s {
            "disabled" => Ok(Disabled),
            "enabled" => Ok(Enabled),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodRemove"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodRemove {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodRemove {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodRemove {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(
            PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodRemove
        ))
        .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodRemove {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize
    for PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodRemove
{
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodRemove>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodRemove::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodRemove
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Enable customers to choose if they wish to save their payment method for future use.
/// Disabled by default.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodSave {
    Disabled,
    Enabled,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodSave {
    pub fn as_str(&self) -> &str {
        use PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodSave::*;
        match self {
            Disabled => "disabled",
            Enabled => "enabled",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodSave {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodSave::*;
        match s {
            "disabled" => Ok(Disabled),
            "enabled" => Ok(Enabled),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodSave"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodSave {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodSave {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodSave {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(
            PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodSave
        ))
        .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodSave {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize
    for PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodSave
{
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodSave>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodSave::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodSave
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
