/// This hash contains the features the customer sheet supports.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CustomerSessionResourceComponentsResourceCustomerSheetResourceFeatures {
        /// A list of [`allow_redisplay`](https://docs.stripe.com/api/payment_methods/object#payment_method_object-allow_redisplay) values that controls which saved payment methods the customer sheet displays by filtering to only show payment methods with an `allow_redisplay` value that is present in this list.
    ///
        /// If not specified, defaults to ["always"].
    /// In order to display all saved payment methods, specify ["always", "limited", "unspecified"].
pub payment_method_allow_redisplay_filters: Option<Vec<CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodAllowRedisplayFilters>>,
    /// Controls whether the customer sheet displays the option to remove a saved payment method."
    ///
        /// Allowing buyers to remove their saved payment methods impacts subscriptions that depend on that payment method.
    /// Removing the payment method detaches the [`customer` object](https://docs.stripe.com/api/payment_methods/object#payment_method_object-customer) from that [PaymentMethod](https://docs.stripe.com/api/payment_methods).
pub payment_method_remove: Option<CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodRemove>,

}
#[doc(hidden)]
pub struct CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesBuilder {
    payment_method_allow_redisplay_filters: Option<Option<Vec<CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodAllowRedisplayFilters>>>,
payment_method_remove: Option<Option<CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodRemove>>,

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

    impl Deserialize for CustomerSessionResourceComponentsResourceCustomerSheetResourceFeatures {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CustomerSessionResourceComponentsResourceCustomerSheetResourceFeatures>,
        builder: CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesBuilder,
    }

    impl Visitor for Place<CustomerSessionResourceComponentsResourceCustomerSheetResourceFeatures> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesBuilder {
        type Out = CustomerSessionResourceComponentsResourceCustomerSheetResourceFeatures;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "payment_method_allow_redisplay_filters" => {
                    Deserialize::begin(&mut self.payment_method_allow_redisplay_filters)
                }
                "payment_method_remove" => Deserialize::begin(&mut self.payment_method_remove),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                payment_method_allow_redisplay_filters: Deserialize::default(),
                payment_method_remove: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(payment_method_allow_redisplay_filters), Some(payment_method_remove)) =
                (self.payment_method_allow_redisplay_filters.take(), self.payment_method_remove)
            else {
                return None;
            };
            Some(Self::Out { payment_method_allow_redisplay_filters, payment_method_remove })
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

    impl ObjectDeser for CustomerSessionResourceComponentsResourceCustomerSheetResourceFeatures {
        type Builder =
            CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesBuilder;
    }

    impl FromValueOpt for CustomerSessionResourceComponentsResourceCustomerSheetResourceFeatures {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "payment_method_allow_redisplay_filters" => {
                        b.payment_method_allow_redisplay_filters = FromValueOpt::from_value(v)
                    }
                    "payment_method_remove" => {
                        b.payment_method_remove = FromValueOpt::from_value(v)
                    }
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// A list of [`allow_redisplay`](https://docs.stripe.com/api/payment_methods/object#payment_method_object-allow_redisplay) values that controls which saved payment methods the customer sheet displays by filtering to only show payment methods with an `allow_redisplay` value that is present in this list.
///
/// If not specified, defaults to ["always"].
/// In order to display all saved payment methods, specify ["always", "limited", "unspecified"].
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodAllowRedisplayFilters
{
    Always,
    Limited,
    Unspecified,
}
impl CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodAllowRedisplayFilters {
    pub fn as_str(self) -> &'static str {
        use CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodAllowRedisplayFilters::*;
        match self {
Always => "always",
Limited => "limited",
Unspecified => "unspecified",

        }
    }
}

impl std::str::FromStr for CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodAllowRedisplayFilters {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodAllowRedisplayFilters::*;
        match s {
    "always" => Ok(Always),
"limited" => Ok(Limited),
"unspecified" => Ok(Unspecified),
_ => Err(stripe_types::StripeParseError)

        }
    }
}
impl std::fmt::Display for CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodAllowRedisplayFilters {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodAllowRedisplayFilters {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodAllowRedisplayFilters {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodAllowRedisplayFilters {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodAllowRedisplayFilters> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodAllowRedisplayFilters::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodAllowRedisplayFilters);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodAllowRedisplayFilters {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodAllowRedisplayFilters"))
    }
}
/// Controls whether the customer sheet displays the option to remove a saved payment method."
///
/// Allowing buyers to remove their saved payment methods impacts subscriptions that depend on that payment method.
/// Removing the payment method detaches the [`customer` object](https://docs.stripe.com/api/payment_methods/object#payment_method_object-customer) from that [PaymentMethod](https://docs.stripe.com/api/payment_methods).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodRemove {
    Disabled,
    Enabled,
}
impl CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodRemove {
    pub fn as_str(self) -> &'static str {
        use CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodRemove::*;
        match self {
            Disabled => "disabled",
            Enabled => "enabled",
        }
    }
}

impl std::str::FromStr
    for CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodRemove
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodRemove::*;
        match s {
            "disabled" => Ok(Disabled),
            "enabled" => Ok(Enabled),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodRemove
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodRemove
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize
    for CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodRemove
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize
    for CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodRemove
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<
        CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodRemove,
    >
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodRemove::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodRemove
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodRemove
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodRemove"))
    }
}
