/// This hash contains the features the Payment Element supports.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CustomerSessionResourceComponentsResourcePaymentElementResourceFeatures {
        /// A list of [`allow_redisplay`](https://docs.stripe.com/api/payment_methods/object#payment_method_object-allow_redisplay) values that controls which saved payment methods the Payment Element displays by filtering to only show payment methods with an `allow_redisplay` value that is present in this list.
    ///
        /// If not specified, defaults to ["always"].
    /// In order to display all saved payment methods, specify ["always", "limited", "unspecified"].
pub payment_method_allow_redisplay_filters: Vec<CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodAllowRedisplayFilters>,
        /// Controls whether or not the Payment Element shows saved payment methods.
    /// This parameter defaults to `disabled`.
pub payment_method_redisplay: CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRedisplay,
        /// Determines the max number of saved payment methods for the Payment Element to display.
    /// This parameter defaults to `3`.
pub payment_method_redisplay_limit: Option<i64>,
        /// Controls whether the Payment Element displays the option to remove a saved payment method.
    /// This parameter defaults to `disabled`.
    ///
        /// Allowing buyers to remove their saved payment methods impacts subscriptions that depend on that payment method.
    /// Removing the payment method detaches the [`customer` object](https://docs.stripe.com/api/payment_methods/object#payment_method_object-customer) from that [PaymentMethod](https://docs.stripe.com/api/payment_methods).
pub payment_method_remove: CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRemove,
        /// Controls whether the Payment Element displays a checkbox offering to save a new payment method.
    /// This parameter defaults to `disabled`.
    ///
        /// If a customer checks the box, the [`allow_redisplay`](https://docs.stripe.com/api/payment_methods/object#payment_method_object-allow_redisplay) value on the PaymentMethod is set to `'always'` at confirmation time.
    /// For PaymentIntents, the [`setup_future_usage`](https://docs.stripe.com/api/payment_intents/object#payment_intent_object-setup_future_usage) value is also set to the value defined in `payment_method_save_usage`.
pub payment_method_save: CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSave,
        /// When using PaymentIntents and the customer checks the save checkbox, this field determines the [`setup_future_usage`](https://docs.stripe.com/api/payment_intents/object#payment_intent_object-setup_future_usage) value used to confirm the PaymentIntent.
    ///
        /// When using SetupIntents, directly configure the [`usage`](https://docs.stripe.com/api/setup_intents/object#setup_intent_object-usage) value on SetupIntent creation.
pub payment_method_save_usage: Option<CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSaveUsage>,

}
#[doc(hidden)]
pub struct CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesBuilder {
    payment_method_allow_redisplay_filters: Option<Vec<CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodAllowRedisplayFilters>>,
payment_method_redisplay: Option<CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRedisplay>,
payment_method_redisplay_limit: Option<Option<i64>>,
payment_method_remove: Option<CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRemove>,
payment_method_save: Option<CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSave>,
payment_method_save_usage: Option<Option<CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSaveUsage>>,

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

    impl Deserialize for CustomerSessionResourceComponentsResourcePaymentElementResourceFeatures {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out:
            &'a mut Option<CustomerSessionResourceComponentsResourcePaymentElementResourceFeatures>,
        builder: CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesBuilder,
    }

    impl Visitor for Place<CustomerSessionResourceComponentsResourcePaymentElementResourceFeatures> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesBuilder {
        type Out = CustomerSessionResourceComponentsResourcePaymentElementResourceFeatures;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "payment_method_allow_redisplay_filters" => {
                    Deserialize::begin(&mut self.payment_method_allow_redisplay_filters)
                }
                "payment_method_redisplay" => {
                    Deserialize::begin(&mut self.payment_method_redisplay)
                }
                "payment_method_redisplay_limit" => {
                    Deserialize::begin(&mut self.payment_method_redisplay_limit)
                }
                "payment_method_remove" => Deserialize::begin(&mut self.payment_method_remove),
                "payment_method_save" => Deserialize::begin(&mut self.payment_method_save),
                "payment_method_save_usage" => {
                    Deserialize::begin(&mut self.payment_method_save_usage)
                }

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                payment_method_allow_redisplay_filters: Deserialize::default(),
                payment_method_redisplay: Deserialize::default(),
                payment_method_redisplay_limit: Deserialize::default(),
                payment_method_remove: Deserialize::default(),
                payment_method_save: Deserialize::default(),
                payment_method_save_usage: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(payment_method_allow_redisplay_filters),
                Some(payment_method_redisplay),
                Some(payment_method_redisplay_limit),
                Some(payment_method_remove),
                Some(payment_method_save),
                Some(payment_method_save_usage),
            ) = (
                self.payment_method_allow_redisplay_filters.take(),
                self.payment_method_redisplay,
                self.payment_method_redisplay_limit,
                self.payment_method_remove,
                self.payment_method_save,
                self.payment_method_save_usage,
            )
            else {
                return None;
            };
            Some(Self::Out {
                payment_method_allow_redisplay_filters,
                payment_method_redisplay,
                payment_method_redisplay_limit,
                payment_method_remove,
                payment_method_save,
                payment_method_save_usage,
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

    impl ObjectDeser for CustomerSessionResourceComponentsResourcePaymentElementResourceFeatures {
        type Builder =
            CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesBuilder;
    }

    impl FromValueOpt for CustomerSessionResourceComponentsResourcePaymentElementResourceFeatures {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "payment_method_allow_redisplay_filters" => {
                        b.payment_method_allow_redisplay_filters = FromValueOpt::from_value(v)
                    }
                    "payment_method_redisplay" => {
                        b.payment_method_redisplay = FromValueOpt::from_value(v)
                    }
                    "payment_method_redisplay_limit" => {
                        b.payment_method_redisplay_limit = FromValueOpt::from_value(v)
                    }
                    "payment_method_remove" => {
                        b.payment_method_remove = FromValueOpt::from_value(v)
                    }
                    "payment_method_save" => b.payment_method_save = FromValueOpt::from_value(v),
                    "payment_method_save_usage" => {
                        b.payment_method_save_usage = FromValueOpt::from_value(v)
                    }

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// A list of [`allow_redisplay`](https://docs.stripe.com/api/payment_methods/object#payment_method_object-allow_redisplay) values that controls which saved payment methods the Payment Element displays by filtering to only show payment methods with an `allow_redisplay` value that is present in this list.
///
/// If not specified, defaults to ["always"].
/// In order to display all saved payment methods, specify ["always", "limited", "unspecified"].
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodAllowRedisplayFilters
{
    Always,
    Limited,
    Unspecified,
}
impl CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodAllowRedisplayFilters {
    pub fn as_str(self) -> &'static str {
        use CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodAllowRedisplayFilters::*;
        match self {
Always => "always",
Limited => "limited",
Unspecified => "unspecified",

        }
    }
}

impl std::str::FromStr for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodAllowRedisplayFilters {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodAllowRedisplayFilters::*;
        match s {
    "always" => Ok(Always),
"limited" => Ok(Limited),
"unspecified" => Ok(Unspecified),
_ => Err(stripe_types::StripeParseError)

        }
    }
}
impl std::fmt::Display for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodAllowRedisplayFilters {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodAllowRedisplayFilters {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodAllowRedisplayFilters {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodAllowRedisplayFilters {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodAllowRedisplayFilters> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodAllowRedisplayFilters::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodAllowRedisplayFilters);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodAllowRedisplayFilters {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodAllowRedisplayFilters"))
    }
}
/// Controls whether or not the Payment Element shows saved payment methods.
/// This parameter defaults to `disabled`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRedisplay
{
    Disabled,
    Enabled,
}
impl CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRedisplay {
    pub fn as_str(self) -> &'static str {
        use CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRedisplay::*;
        match self {
            Disabled => "disabled",
            Enabled => "enabled",
        }
    }
}

impl std::str::FromStr for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRedisplay {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRedisplay::*;
        match s {
    "disabled" => Ok(Disabled),
"enabled" => Ok(Enabled),
_ => Err(stripe_types::StripeParseError)

        }
    }
}
impl std::fmt::Display for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRedisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRedisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRedisplay {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRedisplay {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRedisplay> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRedisplay::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRedisplay
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRedisplay {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRedisplay"))
    }
}
/// Controls whether the Payment Element displays the option to remove a saved payment method.
/// This parameter defaults to `disabled`.
///
/// Allowing buyers to remove their saved payment methods impacts subscriptions that depend on that payment method.
/// Removing the payment method detaches the [`customer` object](https://docs.stripe.com/api/payment_methods/object#payment_method_object-customer) from that [PaymentMethod](https://docs.stripe.com/api/payment_methods).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRemove
{
    Disabled,
    Enabled,
}
impl CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRemove {
    pub fn as_str(self) -> &'static str {
        use CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRemove::*;
        match self {
            Disabled => "disabled",
            Enabled => "enabled",
        }
    }
}

impl std::str::FromStr
    for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRemove
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRemove::*;
        match s {
            "disabled" => Ok(Disabled),
            "enabled" => Ok(Enabled),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRemove
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRemove
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize
    for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRemove
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize
    for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRemove
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<
        CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRemove,
    >
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRemove::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRemove
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRemove
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRemove"))
    }
}
/// Controls whether the Payment Element displays a checkbox offering to save a new payment method.
/// This parameter defaults to `disabled`.
///
/// If a customer checks the box, the [`allow_redisplay`](https://docs.stripe.com/api/payment_methods/object#payment_method_object-allow_redisplay) value on the PaymentMethod is set to `'always'` at confirmation time.
/// For PaymentIntents, the [`setup_future_usage`](https://docs.stripe.com/api/payment_intents/object#payment_intent_object-setup_future_usage) value is also set to the value defined in `payment_method_save_usage`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSave {
    Disabled,
    Enabled,
}
impl CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSave {
    pub fn as_str(self) -> &'static str {
        use CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSave::*;
        match self {
            Disabled => "disabled",
            Enabled => "enabled",
        }
    }
}

impl std::str::FromStr
    for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSave
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSave::*;
        match s {
            "disabled" => Ok(Disabled),
            "enabled" => Ok(Enabled),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSave
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSave
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize
    for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSave
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize
    for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSave
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<
        CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSave,
    >
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSave::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSave
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSave
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSave"))
    }
}
/// When using PaymentIntents and the customer checks the save checkbox, this field determines the [`setup_future_usage`](https://docs.stripe.com/api/payment_intents/object#payment_intent_object-setup_future_usage) value used to confirm the PaymentIntent.
///
/// When using SetupIntents, directly configure the [`usage`](https://docs.stripe.com/api/setup_intents/object#setup_intent_object-usage) value on SetupIntent creation.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSaveUsage
{
    OffSession,
    OnSession,
}
impl CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSaveUsage {
    pub fn as_str(self) -> &'static str {
        use CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSaveUsage::*;
        match self {
            OffSession => "off_session",
            OnSession => "on_session",
        }
    }
}

impl std::str::FromStr for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSaveUsage {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSaveUsage::*;
        match s {
    "off_session" => Ok(OffSession),
"on_session" => Ok(OnSession),
_ => Err(stripe_types::StripeParseError)

        }
    }
}
impl std::fmt::Display for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSaveUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSaveUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSaveUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSaveUsage {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSaveUsage> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSaveUsage::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSaveUsage
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSaveUsage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSaveUsage"))
    }
}
