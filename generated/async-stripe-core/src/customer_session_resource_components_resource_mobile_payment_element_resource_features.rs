/// This hash contains the features the mobile payment element supports.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeatures {
        /// A list of [`allow_redisplay`](https://docs.stripe.com/api/payment_methods/object#payment_method_object-allow_redisplay) values that controls which saved payment methods the mobile payment element displays by filtering to only show payment methods with an `allow_redisplay` value that is present in this list.
    ///
        /// If not specified, defaults to ["always"].
    /// In order to display all saved payment methods, specify ["always", "limited", "unspecified"].
pub payment_method_allow_redisplay_filters: Option<Vec<CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodAllowRedisplayFilters>>,
    /// Controls whether or not the mobile payment element shows saved payment methods.
pub payment_method_redisplay: Option<CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodRedisplay>,
    /// Controls whether the mobile payment element displays the option to remove a saved payment method."
    ///
        /// Allowing buyers to remove their saved payment methods impacts subscriptions that depend on that payment method.
    /// Removing the payment method detaches the [`customer` object](https://docs.stripe.com/api/payment_methods/object#payment_method_object-customer) from that [PaymentMethod](https://docs.stripe.com/api/payment_methods).
pub payment_method_remove: Option<CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodRemove>,
        /// Controls whether the mobile payment element displays a checkbox offering to save a new payment method.
    ///
        /// If a customer checks the box, the [`allow_redisplay`](https://docs.stripe.com/api/payment_methods/object#payment_method_object-allow_redisplay) value on the PaymentMethod is set to `'always'` at confirmation time.
    /// For PaymentIntents, the [`setup_future_usage`](https://docs.stripe.com/api/payment_intents/object#payment_intent_object-setup_future_usage) value is also set to the value defined in `payment_method_save_usage`.
pub payment_method_save: Option<CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodSave>,
        /// Allows overriding the value of allow_override when saving a new payment method when payment_method_save is set to disabled.
    /// Use values: "always", "limited", or "unspecified".
    ///
    /// If not specified, defaults to `nil` (no override value).
pub payment_method_save_allow_redisplay_override: Option<CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodSaveAllowRedisplayOverride>,

}
#[doc(hidden)]
pub struct CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesBuilder {
    payment_method_allow_redisplay_filters: Option<Option<Vec<CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodAllowRedisplayFilters>>>,
payment_method_redisplay: Option<Option<CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodRedisplay>>,
payment_method_remove: Option<Option<CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodRemove>>,
payment_method_save: Option<Option<CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodSave>>,
payment_method_save_allow_redisplay_override: Option<Option<CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodSaveAllowRedisplayOverride>>,

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

    impl Deserialize for CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeatures {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<
            CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeatures,
        >,
        builder:
            CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesBuilder,
    }

    impl Visitor
        for Place<CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeatures>
    {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder
        for CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesBuilder
    {
        type Out = CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeatures;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "payment_method_allow_redisplay_filters" => {
                    Deserialize::begin(&mut self.payment_method_allow_redisplay_filters)
                }
                "payment_method_redisplay" => {
                    Deserialize::begin(&mut self.payment_method_redisplay)
                }
                "payment_method_remove" => Deserialize::begin(&mut self.payment_method_remove),
                "payment_method_save" => Deserialize::begin(&mut self.payment_method_save),
                "payment_method_save_allow_redisplay_override" => {
                    Deserialize::begin(&mut self.payment_method_save_allow_redisplay_override)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                payment_method_allow_redisplay_filters: Deserialize::default(),
                payment_method_redisplay: Deserialize::default(),
                payment_method_remove: Deserialize::default(),
                payment_method_save: Deserialize::default(),
                payment_method_save_allow_redisplay_override: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(payment_method_allow_redisplay_filters),
                Some(payment_method_redisplay),
                Some(payment_method_remove),
                Some(payment_method_save),
                Some(payment_method_save_allow_redisplay_override),
            ) = (
                self.payment_method_allow_redisplay_filters.take(),
                self.payment_method_redisplay.take(),
                self.payment_method_remove.take(),
                self.payment_method_save.take(),
                self.payment_method_save_allow_redisplay_override.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                payment_method_allow_redisplay_filters,
                payment_method_redisplay,
                payment_method_remove,
                payment_method_save,
                payment_method_save_allow_redisplay_override,
            })
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

    impl ObjectDeser for CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeatures {
        type Builder =
            CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesBuilder;
    }

    impl FromValueOpt
        for CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeatures
    {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "payment_method_allow_redisplay_filters" => {
                        b.payment_method_allow_redisplay_filters = FromValueOpt::from_value(v)
                    }
                    "payment_method_redisplay" => {
                        b.payment_method_redisplay = FromValueOpt::from_value(v)
                    }
                    "payment_method_remove" => {
                        b.payment_method_remove = FromValueOpt::from_value(v)
                    }
                    "payment_method_save" => b.payment_method_save = FromValueOpt::from_value(v),
                    "payment_method_save_allow_redisplay_override" => {
                        b.payment_method_save_allow_redisplay_override = FromValueOpt::from_value(v)
                    }
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// A list of [`allow_redisplay`](https://docs.stripe.com/api/payment_methods/object#payment_method_object-allow_redisplay) values that controls which saved payment methods the mobile payment element displays by filtering to only show payment methods with an `allow_redisplay` value that is present in this list.
///
/// If not specified, defaults to ["always"].
/// In order to display all saved payment methods, specify ["always", "limited", "unspecified"].
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodAllowRedisplayFilters
{
    Always,
    Limited,
    Unspecified,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodAllowRedisplayFilters {
    pub fn as_str(&self) -> &str {
        use CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodAllowRedisplayFilters::*;
        match self {
Always => "always",
Limited => "limited",
Unspecified => "unspecified",
Unknown(v) => v,

        }
    }
}

impl std::str::FromStr for CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodAllowRedisplayFilters {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodAllowRedisplayFilters::*;
        match s {
    "always" => Ok(Always),
"limited" => Ok(Limited),
"unspecified" => Ok(Unspecified),
v => { tracing::warn!("Unknown value '{}' for enum '{}'", v, "CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodAllowRedisplayFilters"); Ok(Unknown(v.to_owned())) }

        }
    }
}
impl std::fmt::Display for CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodAllowRedisplayFilters {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodAllowRedisplayFilters {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodAllowRedisplayFilters {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodAllowRedisplayFilters {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodAllowRedisplayFilters> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodAllowRedisplayFilters::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodAllowRedisplayFilters);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodAllowRedisplayFilters {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Controls whether or not the mobile payment element shows saved payment methods.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodRedisplay
{
    Disabled,
    Enabled,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodRedisplay {
    pub fn as_str(&self) -> &str {
        use CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodRedisplay::*;
        match self {
Disabled => "disabled",
Enabled => "enabled",
Unknown(v) => v,

        }
    }
}

impl std::str::FromStr for CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodRedisplay {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodRedisplay::*;
        match s {
    "disabled" => Ok(Disabled),
"enabled" => Ok(Enabled),
v => { tracing::warn!("Unknown value '{}' for enum '{}'", v, "CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodRedisplay"); Ok(Unknown(v.to_owned())) }

        }
    }
}
impl std::fmt::Display for CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodRedisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodRedisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodRedisplay {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodRedisplay {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodRedisplay> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodRedisplay::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodRedisplay);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodRedisplay {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Controls whether the mobile payment element displays the option to remove a saved payment method."
///
/// Allowing buyers to remove their saved payment methods impacts subscriptions that depend on that payment method.
/// Removing the payment method detaches the [`customer` object](https://docs.stripe.com/api/payment_methods/object#payment_method_object-customer) from that [PaymentMethod](https://docs.stripe.com/api/payment_methods).
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodRemove
{
    Disabled,
    Enabled,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl
    CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodRemove
{
    pub fn as_str(&self) -> &str {
        use CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodRemove::*;
        match self {
            Disabled => "disabled",
            Enabled => "enabled",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodRemove {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodRemove::*;
        match s {
    "disabled" => Ok(Disabled),
"enabled" => Ok(Enabled),
v => { tracing::warn!("Unknown value '{}' for enum '{}'", v, "CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodRemove"); Ok(Unknown(v.to_owned())) }

        }
    }
}
impl std::fmt::Display for CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodRemove {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodRemove {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodRemove {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodRemove {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodRemove> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodRemove::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodRemove);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodRemove {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Controls whether the mobile payment element displays a checkbox offering to save a new payment method.
///
/// If a customer checks the box, the [`allow_redisplay`](https://docs.stripe.com/api/payment_methods/object#payment_method_object-allow_redisplay) value on the PaymentMethod is set to `'always'` at confirmation time.
/// For PaymentIntents, the [`setup_future_usage`](https://docs.stripe.com/api/payment_intents/object#payment_intent_object-setup_future_usage) value is also set to the value defined in `payment_method_save_usage`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodSave
{
    Disabled,
    Enabled,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl
    CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodSave
{
    pub fn as_str(&self) -> &str {
        use CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodSave::*;
        match self {
            Disabled => "disabled",
            Enabled => "enabled",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodSave {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodSave::*;
        match s {
    "disabled" => Ok(Disabled),
"enabled" => Ok(Enabled),
v => { tracing::warn!("Unknown value '{}' for enum '{}'", v, "CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodSave"); Ok(Unknown(v.to_owned())) }

        }
    }
}
impl std::fmt::Display for CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodSave {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodSave {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodSave {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodSave {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodSave> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodSave::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodSave
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodSave {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Allows overriding the value of allow_override when saving a new payment method when payment_method_save is set to disabled.
/// Use values: "always", "limited", or "unspecified".
///
/// If not specified, defaults to `nil` (no override value).
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodSaveAllowRedisplayOverride
{
    Always,
    Limited,
    Unspecified,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodSaveAllowRedisplayOverride {
    pub fn as_str(&self) -> &str {
        use CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodSaveAllowRedisplayOverride::*;
        match self {
Always => "always",
Limited => "limited",
Unspecified => "unspecified",
Unknown(v) => v,

        }
    }
}

impl std::str::FromStr for CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodSaveAllowRedisplayOverride {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodSaveAllowRedisplayOverride::*;
        match s {
    "always" => Ok(Always),
"limited" => Ok(Limited),
"unspecified" => Ok(Unspecified),
v => { tracing::warn!("Unknown value '{}' for enum '{}'", v, "CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodSaveAllowRedisplayOverride"); Ok(Unknown(v.to_owned())) }

        }
    }
}
impl std::fmt::Display for CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodSaveAllowRedisplayOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodSaveAllowRedisplayOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodSaveAllowRedisplayOverride {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodSaveAllowRedisplayOverride {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodSaveAllowRedisplayOverride> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodSaveAllowRedisplayOverride::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodSaveAllowRedisplayOverride);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodSaveAllowRedisplayOverride {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
