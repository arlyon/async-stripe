#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionConsentCollection {
    /// If set to `hidden`, it will hide legal text related to the reuse of a payment method.
    pub payment_method_reuse_agreement:
        Option<stripe_checkout::PaymentPagesCheckoutSessionPaymentMethodReuseAgreement>,
    /// If set to `auto`, enables the collection of customer consent for promotional communications.
    /// The Checkout.
    /// Session will determine whether to display an option to opt into promotional communication
    /// from the merchant depending on the customer's locale. Only available to US merchants.
    pub promotions: Option<PaymentPagesCheckoutSessionConsentCollectionPromotions>,
    /// If set to `required`, it requires customers to accept the terms of service before being able to pay.
    pub terms_of_service: Option<PaymentPagesCheckoutSessionConsentCollectionTermsOfService>,
}
#[doc(hidden)]
pub struct PaymentPagesCheckoutSessionConsentCollectionBuilder {
    payment_method_reuse_agreement:
        Option<Option<stripe_checkout::PaymentPagesCheckoutSessionPaymentMethodReuseAgreement>>,
    promotions: Option<Option<PaymentPagesCheckoutSessionConsentCollectionPromotions>>,
    terms_of_service: Option<Option<PaymentPagesCheckoutSessionConsentCollectionTermsOfService>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentPagesCheckoutSessionConsentCollection {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentPagesCheckoutSessionConsentCollection>,
        builder: PaymentPagesCheckoutSessionConsentCollectionBuilder,
    }

    impl Visitor for Place<PaymentPagesCheckoutSessionConsentCollection> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentPagesCheckoutSessionConsentCollectionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentPagesCheckoutSessionConsentCollectionBuilder {
        type Out = PaymentPagesCheckoutSessionConsentCollection;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "payment_method_reuse_agreement" => {
                    Deserialize::begin(&mut self.payment_method_reuse_agreement)
                }
                "promotions" => Deserialize::begin(&mut self.promotions),
                "terms_of_service" => Deserialize::begin(&mut self.terms_of_service),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                payment_method_reuse_agreement: Deserialize::default(),
                promotions: Deserialize::default(),
                terms_of_service: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                payment_method_reuse_agreement: self.payment_method_reuse_agreement?,
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

    impl ObjectDeser for PaymentPagesCheckoutSessionConsentCollection {
        type Builder = PaymentPagesCheckoutSessionConsentCollectionBuilder;
    }

    impl FromValueOpt for PaymentPagesCheckoutSessionConsentCollection {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentPagesCheckoutSessionConsentCollectionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "payment_method_reuse_agreement" => {
                        b.payment_method_reuse_agreement = Some(FromValueOpt::from_value(v)?)
                    }
                    "promotions" => b.promotions = Some(FromValueOpt::from_value(v)?),
                    "terms_of_service" => b.terms_of_service = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// If set to `auto`, enables the collection of customer consent for promotional communications.
/// The Checkout.
/// Session will determine whether to display an option to opt into promotional communication
/// from the merchant depending on the customer's locale. Only available to US merchants.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentPagesCheckoutSessionConsentCollectionPromotions {
    Auto,
    None,
}
impl PaymentPagesCheckoutSessionConsentCollectionPromotions {
    pub fn as_str(self) -> &'static str {
        use PaymentPagesCheckoutSessionConsentCollectionPromotions::*;
        match self {
            Auto => "auto",
            None => "none",
        }
    }
}

impl std::str::FromStr for PaymentPagesCheckoutSessionConsentCollectionPromotions {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentPagesCheckoutSessionConsentCollectionPromotions::*;
        match s {
            "auto" => Ok(Auto),
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PaymentPagesCheckoutSessionConsentCollectionPromotions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentPagesCheckoutSessionConsentCollectionPromotions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentPagesCheckoutSessionConsentCollectionPromotions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentPagesCheckoutSessionConsentCollectionPromotions {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<PaymentPagesCheckoutSessionConsentCollectionPromotions>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentPagesCheckoutSessionConsentCollectionPromotions::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentPagesCheckoutSessionConsentCollectionPromotions);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentPagesCheckoutSessionConsentCollectionPromotions {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentPagesCheckoutSessionConsentCollectionPromotions",
            )
        })
    }
}
/// If set to `required`, it requires customers to accept the terms of service before being able to pay.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentPagesCheckoutSessionConsentCollectionTermsOfService {
    None,
    Required,
}
impl PaymentPagesCheckoutSessionConsentCollectionTermsOfService {
    pub fn as_str(self) -> &'static str {
        use PaymentPagesCheckoutSessionConsentCollectionTermsOfService::*;
        match self {
            None => "none",
            Required => "required",
        }
    }
}

impl std::str::FromStr for PaymentPagesCheckoutSessionConsentCollectionTermsOfService {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentPagesCheckoutSessionConsentCollectionTermsOfService::*;
        match s {
            "none" => Ok(None),
            "required" => Ok(Required),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PaymentPagesCheckoutSessionConsentCollectionTermsOfService {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentPagesCheckoutSessionConsentCollectionTermsOfService {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentPagesCheckoutSessionConsentCollectionTermsOfService {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentPagesCheckoutSessionConsentCollectionTermsOfService {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<PaymentPagesCheckoutSessionConsentCollectionTermsOfService>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentPagesCheckoutSessionConsentCollectionTermsOfService::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    PaymentPagesCheckoutSessionConsentCollectionTermsOfService
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentPagesCheckoutSessionConsentCollectionTermsOfService {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentPagesCheckoutSessionConsentCollectionTermsOfService",
            )
        })
    }
}
