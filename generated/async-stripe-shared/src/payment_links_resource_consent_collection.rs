#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentLinksResourceConsentCollection {
    /// Settings related to the payment method reuse text shown in the Checkout UI.
    pub payment_method_reuse_agreement:
        Option<stripe_shared::PaymentLinksResourcePaymentMethodReuseAgreement>,
    /// If set to `auto`, enables the collection of customer consent for promotional communications.
    pub promotions: Option<stripe_shared::Promotions>,
    /// If set to `required`, it requires cutomers to accept the terms of service before being able to pay.
    /// If set to `none`, customers won't be shown a checkbox to accept the terms of service.
    pub terms_of_service: Option<stripe_shared::TermsOfService>,
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
    promotions: Option<Option<stripe_shared::Promotions>>,
    terms_of_service: Option<Option<stripe_shared::TermsOfService>>,
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
                builder: PaymentLinksResourceConsentCollectionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentLinksResourceConsentCollectionBuilder {
        type Out = PaymentLinksResourceConsentCollection;
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
                payment_method_reuse_agreement: Some(None),
                promotions: Some(None),
                terms_of_service: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(payment_method_reuse_agreement), Some(promotions), Some(terms_of_service)) = (
                self.payment_method_reuse_agreement.take(),
                self.promotions.take(),
                self.terms_of_service.take(),
            ) else {
                return None;
            };
            Some(Self::Out { payment_method_reuse_agreement, promotions, terms_of_service })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for PaymentLinksResourceConsentCollection {
        type Builder = PaymentLinksResourceConsentCollectionBuilder;
    }

    impl FromValueOpt for PaymentLinksResourceConsentCollection {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentLinksResourceConsentCollectionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "payment_method_reuse_agreement" => {
                        b.payment_method_reuse_agreement = FromValueOpt::from_value(v)
                    }
                    "promotions" => b.promotions = FromValueOpt::from_value(v),
                    "terms_of_service" => b.terms_of_service = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// If set to `auto`, enables the collection of customer consent for promotional communications.
pub type PaymentLinksResourceConsentCollectionPromotions = stripe_shared::Promotions;
/// If set to `required`, it requires cutomers to accept the terms of service before being able to pay.
/// If set to `none`, customers won't be shown a checkbox to accept the terms of service.
pub type PaymentLinksResourceConsentCollectionTermsOfService = stripe_shared::TermsOfService;
