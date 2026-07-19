#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentIntentNextActionKonbiniStores {
    /// FamilyMart instruction details.
    pub familymart: Option<stripe_shared::PaymentIntentNextActionKonbiniFamilymart>,
    /// Lawson instruction details.
    pub lawson: Option<stripe_shared::PaymentIntentNextActionKonbiniLawson>,
    /// Ministop instruction details.
    pub ministop: Option<stripe_shared::PaymentIntentNextActionKonbiniMinistop>,
    /// Seicomart instruction details.
    pub seicomart: Option<stripe_shared::PaymentIntentNextActionKonbiniSeicomart>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentIntentNextActionKonbiniStores {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentIntentNextActionKonbiniStores").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentIntentNextActionKonbiniStoresBuilder {
    familymart: Option<Option<stripe_shared::PaymentIntentNextActionKonbiniFamilymart>>,
    lawson: Option<Option<stripe_shared::PaymentIntentNextActionKonbiniLawson>>,
    ministop: Option<Option<stripe_shared::PaymentIntentNextActionKonbiniMinistop>>,
    seicomart: Option<Option<stripe_shared::PaymentIntentNextActionKonbiniSeicomart>>,
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

    impl Deserialize for PaymentIntentNextActionKonbiniStores {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentNextActionKonbiniStores>,
        builder: PaymentIntentNextActionKonbiniStoresBuilder,
    }

    impl Visitor for Place<PaymentIntentNextActionKonbiniStores> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentIntentNextActionKonbiniStoresBuilder {
                    familymart: Deserialize::default(),
                    lawson: Deserialize::default(),
                    ministop: Deserialize::default(),
                    seicomart: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "familymart" => Deserialize::begin(&mut self.builder.familymart),
                "lawson" => Deserialize::begin(&mut self.builder.lawson),
                "ministop" => Deserialize::begin(&mut self.builder.ministop),
                "seicomart" => Deserialize::begin(&mut self.builder.seicomart),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(familymart), Some(lawson), Some(ministop), Some(seicomart)) = (
                self.builder.familymart.take(),
                self.builder.lawson.take(),
                self.builder.ministop.take(),
                self.builder.seicomart.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(PaymentIntentNextActionKonbiniStores {
                familymart,
                lawson,
                ministop,
                seicomart,
            });
            Ok(())
        }
    }
};
