#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingCardholderIndividual {
    /// Information related to the card_issuing program for this cardholder.
    pub card_issuing: Option<stripe_shared::IssuingCardholderCardIssuing>,
    /// The date of birth of this cardholder.
    pub dob: Option<stripe_shared::IssuingCardholderIndividualDob>,
    /// The first name of this cardholder.
    /// Required before activating Cards.
    /// This field cannot contain any numbers, special characters (except periods, commas, hyphens, spaces and apostrophes) or non-latin letters.
    pub first_name: Option<String>,
    /// The last name of this cardholder.
    /// Required before activating Cards.
    /// This field cannot contain any numbers, special characters (except periods, commas, hyphens, spaces and apostrophes) or non-latin letters.
    pub last_name: Option<String>,
    /// Government-issued ID document for this cardholder.
    pub verification: Option<stripe_shared::IssuingCardholderVerification>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingCardholderIndividual {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingCardholderIndividual").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingCardholderIndividualBuilder {
    card_issuing: Option<Option<stripe_shared::IssuingCardholderCardIssuing>>,
    dob: Option<Option<stripe_shared::IssuingCardholderIndividualDob>>,
    first_name: Option<Option<String>>,
    last_name: Option<Option<String>>,
    verification: Option<Option<stripe_shared::IssuingCardholderVerification>>,
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

    impl Deserialize for IssuingCardholderIndividual {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingCardholderIndividual>,
        builder: IssuingCardholderIndividualBuilder,
    }

    impl Visitor for Place<IssuingCardholderIndividual> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingCardholderIndividualBuilder {
                    card_issuing: Deserialize::default(),
                    dob: Deserialize::default(),
                    first_name: Deserialize::default(),
                    last_name: Deserialize::default(),
                    verification: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "card_issuing" => Deserialize::begin(&mut self.builder.card_issuing),
                "dob" => Deserialize::begin(&mut self.builder.dob),
                "first_name" => Deserialize::begin(&mut self.builder.first_name),
                "last_name" => Deserialize::begin(&mut self.builder.last_name),
                "verification" => Deserialize::begin(&mut self.builder.verification),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(card_issuing),
                Some(dob),
                Some(first_name),
                Some(last_name),
                Some(verification),
            ) = (
                self.builder.card_issuing.take(),
                self.builder.dob,
                self.builder.first_name.take(),
                self.builder.last_name.take(),
                self.builder.verification.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(IssuingCardholderIndividual {
                card_issuing,
                dob,
                first_name,
                last_name,
                verification,
            });
            Ok(())
        }
    }
};
