#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionSavedPaymentMethodOptions {
    /// Controls which payment methods are eligible to be redisplayed to returning customers.
    /// Corresponds to `allow_redisplay` on the payment method.
    pub allow_redisplay_filters:
        Option<Vec<PaymentPagesCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFilters>>,
    /// Enable customers to choose if they wish to save their payment method for future use.
    pub payment_method_save:
        Option<PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodSave>,
}
#[doc(hidden)]
pub struct PaymentPagesCheckoutSessionSavedPaymentMethodOptionsBuilder {
    allow_redisplay_filters: Option<
        Option<Vec<PaymentPagesCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFilters>>,
    >,
    payment_method_save:
        Option<Option<PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodSave>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: PaymentPagesCheckoutSessionSavedPaymentMethodOptionsBuilder::deser_default(
                ),
            }))
        }
    }

    impl MapBuilder for PaymentPagesCheckoutSessionSavedPaymentMethodOptionsBuilder {
        type Out = PaymentPagesCheckoutSessionSavedPaymentMethodOptions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "allow_redisplay_filters" => Deserialize::begin(&mut self.allow_redisplay_filters),
                "payment_method_save" => Deserialize::begin(&mut self.payment_method_save),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                allow_redisplay_filters: Deserialize::default(),
                payment_method_save: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                allow_redisplay_filters: self.allow_redisplay_filters.take()?,
                payment_method_save: self.payment_method_save?,
            })
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

    impl ObjectDeser for PaymentPagesCheckoutSessionSavedPaymentMethodOptions {
        type Builder = PaymentPagesCheckoutSessionSavedPaymentMethodOptionsBuilder;
    }

    impl FromValueOpt for PaymentPagesCheckoutSessionSavedPaymentMethodOptions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                PaymentPagesCheckoutSessionSavedPaymentMethodOptionsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "allow_redisplay_filters" => {
                        b.allow_redisplay_filters = Some(FromValueOpt::from_value(v)?)
                    }
                    "payment_method_save" => {
                        b.payment_method_save = Some(FromValueOpt::from_value(v)?)
                    }

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Controls which payment methods are eligible to be redisplayed to returning customers.
/// Corresponds to `allow_redisplay` on the payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentPagesCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFilters {
    Always,
    Limited,
    Unspecified,
}
impl PaymentPagesCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFilters {
    pub fn as_str(self) -> &'static str {
        use PaymentPagesCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFilters::*;
        match self {
            Always => "always",
            Limited => "limited",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr
    for PaymentPagesCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFilters
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentPagesCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFilters::*;
        match s {
            "always" => Ok(Always),
            "limited" => Ok(Limited),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
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

impl std::fmt::Debug for PaymentPagesCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFilters {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize
    for PaymentPagesCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFilters
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<PaymentPagesCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFilters>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentPagesCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFilters::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    PaymentPagesCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFilters
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for PaymentPagesCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFilters
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentPagesCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFilters"))
    }
}
/// Enable customers to choose if they wish to save their payment method for future use.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodSave {
    Disabled,
    Enabled,
}
impl PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodSave {
    pub fn as_str(self) -> &'static str {
        use PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodSave::*;
        match self {
            Disabled => "disabled",
            Enabled => "enabled",
        }
    }
}

impl std::str::FromStr for PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodSave {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodSave::*;
        match s {
            "disabled" => Ok(Disabled),
            "enabled" => Ok(Enabled),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodSave {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodSave {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize
    for PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodSave
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodSave>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodSave::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodSave
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodSave
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodSave"))
    }
}
