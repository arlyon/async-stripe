#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingCardholderCardIssuing {
    /// Information about cardholder acceptance of Celtic [Authorized User Terms](https://stripe.com/docs/issuing/cards#accept-authorized-user-terms).
    /// Required for cards backed by a Celtic program.
    pub user_terms_acceptance: Option<stripe_shared::IssuingCardholderUserTermsAcceptance>,
}
#[doc(hidden)]
pub struct IssuingCardholderCardIssuingBuilder {
    user_terms_acceptance: Option<Option<stripe_shared::IssuingCardholderUserTermsAcceptance>>,
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

    impl Deserialize for IssuingCardholderCardIssuing {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingCardholderCardIssuing>,
        builder: IssuingCardholderCardIssuingBuilder,
    }

    impl Visitor for Place<IssuingCardholderCardIssuing> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingCardholderCardIssuingBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingCardholderCardIssuingBuilder {
        type Out = IssuingCardholderCardIssuing;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "user_terms_acceptance" => Deserialize::begin(&mut self.user_terms_acceptance),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { user_terms_acceptance: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(user_terms_acceptance),) = (self.user_terms_acceptance.take(),) else {
                return None;
            };
            Some(Self::Out { user_terms_acceptance })
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

    impl ObjectDeser for IssuingCardholderCardIssuing {
        type Builder = IssuingCardholderCardIssuingBuilder;
    }

    impl FromValueOpt for IssuingCardholderCardIssuing {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingCardholderCardIssuingBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "user_terms_acceptance" => {
                        b.user_terms_acceptance = FromValueOpt::from_value(v)
                    }

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
