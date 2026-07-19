#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentLinksResourceConsentCollection {
    /// Settings related to the payment method reuse text shown in the Checkout UI.
    pub payment_method_reuse_agreement:
        Option<stripe_shared::PaymentLinksResourcePaymentMethodReuseAgreement>,
    /// If set to `auto`, enables the collection of customer consent for promotional communications.
    pub promotions: Option<PaymentLinksResourceConsentCollectionPromotions>,
    /// If set to `required`, it requires cutomers to accept the terms of service before being able to pay.
    /// If set to `none`, customers won't be shown a checkbox to accept the terms of service.
    pub terms_of_service: Option<PaymentLinksResourceConsentCollectionTermsOfService>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentLinksResourceConsentCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentLinksResourceConsentCollection").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentLinksResourceConsentCollectionBuilder {
    payment_method_reuse_agreement:
        Option<Option<stripe_shared::PaymentLinksResourcePaymentMethodReuseAgreement>>,
    promotions: Option<Option<PaymentLinksResourceConsentCollectionPromotions>>,
    terms_of_service: Option<Option<PaymentLinksResourceConsentCollectionTermsOfService>>,
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

    impl Deserialize for PaymentLinksResourceConsentCollection {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentLinksResourceConsentCollection>,
        builder: PaymentLinksResourceConsentCollectionBuilder,
    }

    impl Visitor for Place<PaymentLinksResourceConsentCollection> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentLinksResourceConsentCollectionBuilder {
                    payment_method_reuse_agreement: Deserialize::default(),
                    promotions: Deserialize::default(),
                    terms_of_service: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "payment_method_reuse_agreement" => {
                    Deserialize::begin(&mut self.builder.payment_method_reuse_agreement)
                }
                "promotions" => Deserialize::begin(&mut self.builder.promotions),
                "terms_of_service" => Deserialize::begin(&mut self.builder.terms_of_service),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(payment_method_reuse_agreement), Some(promotions), Some(terms_of_service)) = (
                self.builder.payment_method_reuse_agreement.take(),
                self.builder.promotions.take(),
                self.builder.terms_of_service.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(PaymentLinksResourceConsentCollection {
                payment_method_reuse_agreement,
                promotions,
                terms_of_service,
            });
            Ok(())
        }
    }
};
/// If set to `auto`, enables the collection of customer consent for promotional communications.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentLinksResourceConsentCollectionPromotions {
    Auto,
    None,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentLinksResourceConsentCollectionPromotions {
    pub fn as_str(&self) -> &str {
        use PaymentLinksResourceConsentCollectionPromotions::*;
        match self {
            Auto => "auto",
            None => "none",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentLinksResourceConsentCollectionPromotions {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentLinksResourceConsentCollectionPromotions::*;
        match s {
            "auto" => Ok(Auto),
            "none" => Ok(None),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentLinksResourceConsentCollectionPromotions"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentLinksResourceConsentCollectionPromotions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PaymentLinksResourceConsentCollectionPromotions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentLinksResourceConsentCollectionPromotions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PaymentLinksResourceConsentCollectionPromotions))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentLinksResourceConsentCollectionPromotions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for PaymentLinksResourceConsentCollectionPromotions {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<PaymentLinksResourceConsentCollectionPromotions> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(PaymentLinksResourceConsentCollectionPromotions::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentLinksResourceConsentCollectionPromotions {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// If set to `required`, it requires cutomers to accept the terms of service before being able to pay.
/// If set to `none`, customers won't be shown a checkbox to accept the terms of service.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentLinksResourceConsentCollectionTermsOfService {
    None,
    Required,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentLinksResourceConsentCollectionTermsOfService {
    pub fn as_str(&self) -> &str {
        use PaymentLinksResourceConsentCollectionTermsOfService::*;
        match self {
            None => "none",
            Required => "required",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentLinksResourceConsentCollectionTermsOfService {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentLinksResourceConsentCollectionTermsOfService::*;
        match s {
            "none" => Ok(None),
            "required" => Ok(Required),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentLinksResourceConsentCollectionTermsOfService"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentLinksResourceConsentCollectionTermsOfService {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PaymentLinksResourceConsentCollectionTermsOfService {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentLinksResourceConsentCollectionTermsOfService {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PaymentLinksResourceConsentCollectionTermsOfService))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentLinksResourceConsentCollectionTermsOfService {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for PaymentLinksResourceConsentCollectionTermsOfService {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<PaymentLinksResourceConsentCollectionTermsOfService> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentLinksResourceConsentCollectionTermsOfService::from_str(s).expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentLinksResourceConsentCollectionTermsOfService {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
