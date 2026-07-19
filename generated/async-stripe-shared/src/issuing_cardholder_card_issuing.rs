#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingCardholderCardIssuing {
    /// Information about cardholder acceptance of Celtic [Authorized User Terms](https://stripe.com/docs/issuing/cards#accept-authorized-user-terms).
    /// Required for cards backed by a Celtic program.
    pub user_terms_acceptance: Option<stripe_shared::IssuingCardholderUserTermsAcceptance>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingCardholderCardIssuing {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingCardholderCardIssuing").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingCardholderCardIssuingBuilder {
    user_terms_acceptance: Option<Option<stripe_shared::IssuingCardholderUserTermsAcceptance>>,
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
                builder: IssuingCardholderCardIssuingBuilder {
                    user_terms_acceptance: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "user_terms_acceptance" => {
                    Deserialize::begin(&mut self.builder.user_terms_acceptance)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(user_terms_acceptance),) = (self.builder.user_terms_acceptance.take(),)
            else {
                return Ok(());
            };
            *self.out = Some(IssuingCardholderCardIssuing { user_terms_acceptance });
            Ok(())
        }
    }
};
