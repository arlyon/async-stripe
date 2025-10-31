#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictions {
    /// Specify the card brands to block in the Checkout Session.
    /// If a customer enters or selects a card belonging to a blocked brand, they can't complete the Session.
    pub brands_blocked:
        Option<Vec<PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBrandsBlocked>>,
}
#[doc(hidden)]
pub struct PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBuilder {
    brands_blocked: Option<
        Option<Vec<PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBrandsBlocked>>,
    >,
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

    impl Deserialize for PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictions>,
        builder: PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBuilder,
    }

    impl Visitor for Place<PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBuilder {
        type Out = PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "brands_blocked" => Deserialize::begin(&mut self.brands_blocked),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { brands_blocked: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(brands_blocked),) = (self.brands_blocked.take(),) else {
                return None;
            };
            Some(Self::Out { brands_blocked })
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

    impl ObjectDeser for PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictions {
        type Builder = PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBuilder;
    }

    impl FromValueOpt for PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "brands_blocked" => b.brands_blocked = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Specify the card brands to block in the Checkout Session.
/// If a customer enters or selects a card belonging to a blocked brand, they can't complete the Session.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBrandsBlocked {
    AmericanExpress,
    DiscoverGlobalNetwork,
    Mastercard,
    Visa,
}
impl PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBrandsBlocked {
    pub fn as_str(self) -> &'static str {
        use PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBrandsBlocked::*;
        match self {
            AmericanExpress => "american_express",
            DiscoverGlobalNetwork => "discover_global_network",
            Mastercard => "mastercard",
            Visa => "visa",
        }
    }
}

impl std::str::FromStr
    for PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBrandsBlocked
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBrandsBlocked::*;
        match s {
            "american_express" => Ok(AmericanExpress),
            "discover_global_network" => Ok(DiscoverGlobalNetwork),
            "mastercard" => Ok(Mastercard),
            "visa" => Ok(Visa),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBrandsBlocked
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBrandsBlocked
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize
    for PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBrandsBlocked
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize
    for PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBrandsBlocked
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBrandsBlocked>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBrandsBlocked::from_str(
                s,
            )
            .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBrandsBlocked
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBrandsBlocked
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBrandsBlocked"))
    }
}
