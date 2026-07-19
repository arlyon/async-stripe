#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictions {
    /// Specify the card brands to block in the Checkout Session.
    /// If a customer enters or selects a card belonging to a blocked brand, they can't complete the Session.
    pub brands_blocked:
        Option<Vec<PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBrandsBlocked>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictions")
            .finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBuilder {
                    brands_blocked: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "brands_blocked" => Deserialize::begin(&mut self.builder.brands_blocked),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(brands_blocked),) = (self.builder.brands_blocked.take(),) else {
                return Ok(());
            };
            *self.out = Some(PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictions {
                brands_blocked,
            });
            Ok(())
        }
    }
};
/// Specify the card brands to block in the Checkout Session.
/// If a customer enters or selects a card belonging to a blocked brand, they can't complete the Session.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBrandsBlocked {
    AmericanExpress,
    DiscoverGlobalNetwork,
    Mastercard,
    Visa,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBrandsBlocked {
    pub fn as_str(&self) -> &str {
        use PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBrandsBlocked::*;
        match self {
            AmericanExpress => "american_express",
            DiscoverGlobalNetwork => "discover_global_network",
            Mastercard => "mastercard",
            Visa => "visa",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBrandsBlocked
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBrandsBlocked::*;
        match s {
            "american_express" => Ok(AmericanExpress),
            "discover_global_network" => Ok(DiscoverGlobalNetwork),
            "mastercard" => Ok(Mastercard),
            "visa" => Ok(Visa),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBrandsBlocked"
                );
                Ok(Unknown(v.to_owned()))
            }
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

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug
    for PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBrandsBlocked
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug
    for PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBrandsBlocked
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(
            PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBrandsBlocked
        ))
        .finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize
    for PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBrandsBlocked
{
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBrandsBlocked>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBrandsBlocked::from_str(
                s,
            )
            .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBrandsBlocked
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
