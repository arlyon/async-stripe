#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentFlowsPrivatePaymentMethodsPaypalAmountDetailsLineItemPaymentMethodOptions {
    /// Type of the line item.
    pub category: Option<
        PaymentFlowsPrivatePaymentMethodsPaypalAmountDetailsLineItemPaymentMethodOptionsCategory,
    >,
    /// Description of the line item.
    pub description: Option<String>,
    /// The Stripe account ID of the connected account that sells the item.
    /// This is only needed when using [Separate Charges and Transfers](https://docs.stripe.com/connect/separate-charges-and-transfers).
    pub sold_by: Option<String>,
}
#[doc(hidden)]
pub struct PaymentFlowsPrivatePaymentMethodsPaypalAmountDetailsLineItemPaymentMethodOptionsBuilder {
    category: Option<Option<PaymentFlowsPrivatePaymentMethodsPaypalAmountDetailsLineItemPaymentMethodOptionsCategory>>,
description: Option<Option<String>>,
sold_by: Option<Option<String>>,

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

    impl Deserialize
        for PaymentFlowsPrivatePaymentMethodsPaypalAmountDetailsLineItemPaymentMethodOptions
    {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<
            PaymentFlowsPrivatePaymentMethodsPaypalAmountDetailsLineItemPaymentMethodOptions,
        >,
        builder:
            PaymentFlowsPrivatePaymentMethodsPaypalAmountDetailsLineItemPaymentMethodOptionsBuilder,
    }

    impl Visitor
        for Place<PaymentFlowsPrivatePaymentMethodsPaypalAmountDetailsLineItemPaymentMethodOptions>
    {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: PaymentFlowsPrivatePaymentMethodsPaypalAmountDetailsLineItemPaymentMethodOptionsBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder
        for PaymentFlowsPrivatePaymentMethodsPaypalAmountDetailsLineItemPaymentMethodOptionsBuilder
    {
        type Out = PaymentFlowsPrivatePaymentMethodsPaypalAmountDetailsLineItemPaymentMethodOptions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "category" => Deserialize::begin(&mut self.category),
                "description" => Deserialize::begin(&mut self.description),
                "sold_by" => Deserialize::begin(&mut self.sold_by),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                category: Deserialize::default(),
                description: Deserialize::default(),
                sold_by: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(category), Some(description), Some(sold_by)) =
                (self.category.take(), self.description.take(), self.sold_by.take())
            else {
                return None;
            };
            Some(Self::Out { category, description, sold_by })
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

    impl ObjectDeser
        for PaymentFlowsPrivatePaymentMethodsPaypalAmountDetailsLineItemPaymentMethodOptions
    {
        type Builder =
            PaymentFlowsPrivatePaymentMethodsPaypalAmountDetailsLineItemPaymentMethodOptionsBuilder;
    }

    impl FromValueOpt
        for PaymentFlowsPrivatePaymentMethodsPaypalAmountDetailsLineItemPaymentMethodOptions
    {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentFlowsPrivatePaymentMethodsPaypalAmountDetailsLineItemPaymentMethodOptionsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "category" => b.category = FromValueOpt::from_value(v),
                    "description" => b.description = FromValueOpt::from_value(v),
                    "sold_by" => b.sold_by = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Type of the line item.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentFlowsPrivatePaymentMethodsPaypalAmountDetailsLineItemPaymentMethodOptionsCategory {
    DigitalGoods,
    Donation,
    PhysicalGoods,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentFlowsPrivatePaymentMethodsPaypalAmountDetailsLineItemPaymentMethodOptionsCategory {
    pub fn as_str(&self) -> &str {
        use PaymentFlowsPrivatePaymentMethodsPaypalAmountDetailsLineItemPaymentMethodOptionsCategory::*;
        match self {
            DigitalGoods => "digital_goods",
            Donation => "donation",
            PhysicalGoods => "physical_goods",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for PaymentFlowsPrivatePaymentMethodsPaypalAmountDetailsLineItemPaymentMethodOptionsCategory
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentFlowsPrivatePaymentMethodsPaypalAmountDetailsLineItemPaymentMethodOptionsCategory::*;
        match s {
            "digital_goods" => Ok(DigitalGoods),
            "donation" => Ok(Donation),
            "physical_goods" => Ok(PhysicalGoods),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentFlowsPrivatePaymentMethodsPaypalAmountDetailsLineItemPaymentMethodOptionsCategory"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display
    for PaymentFlowsPrivatePaymentMethodsPaypalAmountDetailsLineItemPaymentMethodOptionsCategory
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for PaymentFlowsPrivatePaymentMethodsPaypalAmountDetailsLineItemPaymentMethodOptionsCategory
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize
    for PaymentFlowsPrivatePaymentMethodsPaypalAmountDetailsLineItemPaymentMethodOptionsCategory
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize
    for PaymentFlowsPrivatePaymentMethodsPaypalAmountDetailsLineItemPaymentMethodOptionsCategory
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<
        PaymentFlowsPrivatePaymentMethodsPaypalAmountDetailsLineItemPaymentMethodOptionsCategory,
    >
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentFlowsPrivatePaymentMethodsPaypalAmountDetailsLineItemPaymentMethodOptionsCategory::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    PaymentFlowsPrivatePaymentMethodsPaypalAmountDetailsLineItemPaymentMethodOptionsCategory
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for PaymentFlowsPrivatePaymentMethodsPaypalAmountDetailsLineItemPaymentMethodOptionsCategory
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
