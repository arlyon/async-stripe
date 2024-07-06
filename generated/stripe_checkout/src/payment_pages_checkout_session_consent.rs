#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionConsent {
    /// If `opt_in`, the customer consents to receiving promotional communications
    /// from the merchant about this Checkout Session.
    pub promotions: Option<PaymentPagesCheckoutSessionConsentPromotions>,
    /// If `accepted`, the customer in this Checkout Session has agreed to the merchant's terms of service.
    pub terms_of_service: Option<PaymentPagesCheckoutSessionConsentTermsOfService>,
}
#[doc(hidden)]
pub struct PaymentPagesCheckoutSessionConsentBuilder {
    promotions: Option<Option<PaymentPagesCheckoutSessionConsentPromotions>>,
    terms_of_service: Option<Option<PaymentPagesCheckoutSessionConsentTermsOfService>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentPagesCheckoutSessionConsent {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentPagesCheckoutSessionConsent>,
        builder: PaymentPagesCheckoutSessionConsentBuilder,
    }

    impl Visitor for Place<PaymentPagesCheckoutSessionConsent> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentPagesCheckoutSessionConsentBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentPagesCheckoutSessionConsentBuilder {
        type Out = PaymentPagesCheckoutSessionConsent;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "promotions" => Deserialize::begin(&mut self.promotions),
                "terms_of_service" => Deserialize::begin(&mut self.terms_of_service),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { promotions: Deserialize::default(), terms_of_service: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                promotions: self.promotions?,
                terms_of_service: self.terms_of_service?,
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

    impl ObjectDeser for PaymentPagesCheckoutSessionConsent {
        type Builder = PaymentPagesCheckoutSessionConsentBuilder;
    }

    impl FromValueOpt for PaymentPagesCheckoutSessionConsent {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentPagesCheckoutSessionConsentBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "promotions" => b.promotions = Some(FromValueOpt::from_value(v)?),
                    "terms_of_service" => b.terms_of_service = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// If `opt_in`, the customer consents to receiving promotional communications
/// from the merchant about this Checkout Session.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentPagesCheckoutSessionConsentPromotions {
    OptIn,
    OptOut,
}
impl PaymentPagesCheckoutSessionConsentPromotions {
    pub fn as_str(self) -> &'static str {
        use PaymentPagesCheckoutSessionConsentPromotions::*;
        match self {
            OptIn => "opt_in",
            OptOut => "opt_out",
        }
    }
}

impl std::str::FromStr for PaymentPagesCheckoutSessionConsentPromotions {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentPagesCheckoutSessionConsentPromotions::*;
        match s {
            "opt_in" => Ok(OptIn),
            "opt_out" => Ok(OptOut),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PaymentPagesCheckoutSessionConsentPromotions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentPagesCheckoutSessionConsentPromotions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentPagesCheckoutSessionConsentPromotions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentPagesCheckoutSessionConsentPromotions {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentPagesCheckoutSessionConsentPromotions> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentPagesCheckoutSessionConsentPromotions::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentPagesCheckoutSessionConsentPromotions);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentPagesCheckoutSessionConsentPromotions {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentPagesCheckoutSessionConsentPromotions",
            )
        })
    }
}
/// If `accepted`, the customer in this Checkout Session has agreed to the merchant's terms of service.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentPagesCheckoutSessionConsentTermsOfService {
    Accepted,
}
impl PaymentPagesCheckoutSessionConsentTermsOfService {
    pub fn as_str(self) -> &'static str {
        use PaymentPagesCheckoutSessionConsentTermsOfService::*;
        match self {
            Accepted => "accepted",
        }
    }
}

impl std::str::FromStr for PaymentPagesCheckoutSessionConsentTermsOfService {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentPagesCheckoutSessionConsentTermsOfService::*;
        match s {
            "accepted" => Ok(Accepted),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PaymentPagesCheckoutSessionConsentTermsOfService {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentPagesCheckoutSessionConsentTermsOfService {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentPagesCheckoutSessionConsentTermsOfService {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentPagesCheckoutSessionConsentTermsOfService {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentPagesCheckoutSessionConsentTermsOfService> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentPagesCheckoutSessionConsentTermsOfService::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentPagesCheckoutSessionConsentTermsOfService);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentPagesCheckoutSessionConsentTermsOfService {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentPagesCheckoutSessionConsentTermsOfService",
            )
        })
    }
}
