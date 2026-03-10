#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SourceTypeAuBecsDebit {
    pub bsb_number: Option<String>,
    pub fingerprint: Option<String>,
    pub last4: Option<String>,
}
#[doc(hidden)]
pub struct SourceTypeAuBecsDebitBuilder {
    bsb_number: Option<Option<String>>,
    fingerprint: Option<Option<String>>,
    last4: Option<Option<String>>,
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

    impl Deserialize for SourceTypeAuBecsDebit {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceTypeAuBecsDebit>,
        builder: SourceTypeAuBecsDebitBuilder,
    }

    impl Visitor for Place<SourceTypeAuBecsDebit> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SourceTypeAuBecsDebitBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SourceTypeAuBecsDebitBuilder {
        type Out = SourceTypeAuBecsDebit;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bsb_number" => Deserialize::begin(&mut self.bsb_number),
                "fingerprint" => Deserialize::begin(&mut self.fingerprint),
                "last4" => Deserialize::begin(&mut self.last4),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                bsb_number: Deserialize::default(),
                fingerprint: Deserialize::default(),
                last4: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(bsb_number), Some(fingerprint), Some(last4)) =
                (self.bsb_number.take(), self.fingerprint.take(), self.last4.take())
            else {
                return None;
            };
            Some(Self::Out { bsb_number, fingerprint, last4 })
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

    impl ObjectDeser for SourceTypeAuBecsDebit {
        type Builder = SourceTypeAuBecsDebitBuilder;
    }

    impl FromValueOpt for SourceTypeAuBecsDebit {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SourceTypeAuBecsDebitBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "bsb_number" => b.bsb_number = FromValueOpt::from_value(v),
                    "fingerprint" => b.fingerprint = FromValueOpt::from_value(v),
                    "last4" => b.last4 = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
