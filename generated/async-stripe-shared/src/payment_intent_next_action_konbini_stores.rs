#[derive(Clone, Debug)]
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
#[doc(hidden)]
pub struct PaymentIntentNextActionKonbiniStoresBuilder {
    familymart: Option<Option<stripe_shared::PaymentIntentNextActionKonbiniFamilymart>>,
    lawson: Option<Option<stripe_shared::PaymentIntentNextActionKonbiniLawson>>,
    ministop: Option<Option<stripe_shared::PaymentIntentNextActionKonbiniMinistop>>,
    seicomart: Option<Option<stripe_shared::PaymentIntentNextActionKonbiniSeicomart>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: PaymentIntentNextActionKonbiniStoresBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentIntentNextActionKonbiniStoresBuilder {
        type Out = PaymentIntentNextActionKonbiniStores;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "familymart" => Deserialize::begin(&mut self.familymart),
                "lawson" => Deserialize::begin(&mut self.lawson),
                "ministop" => Deserialize::begin(&mut self.ministop),
                "seicomart" => Deserialize::begin(&mut self.seicomart),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                familymart: Deserialize::default(),
                lawson: Deserialize::default(),
                ministop: Deserialize::default(),
                seicomart: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                familymart: self.familymart.take()?,
                lawson: self.lawson.take()?,
                ministop: self.ministop.take()?,
                seicomart: self.seicomart.take()?,
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

    impl ObjectDeser for PaymentIntentNextActionKonbiniStores {
        type Builder = PaymentIntentNextActionKonbiniStoresBuilder;
    }

    impl FromValueOpt for PaymentIntentNextActionKonbiniStores {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentIntentNextActionKonbiniStoresBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "familymart" => b.familymart = Some(FromValueOpt::from_value(v)?),
                    "lawson" => b.lawson = Some(FromValueOpt::from_value(v)?),
                    "ministop" => b.ministop = Some(FromValueOpt::from_value(v)?),
                    "seicomart" => b.seicomart = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
