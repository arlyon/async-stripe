#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SourceTypeAuBecsDebit {
    pub bsb_number: Option<String>,
    pub fingerprint: Option<String>,
    pub last4: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SourceTypeAuBecsDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SourceTypeAuBecsDebit").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: SourceTypeAuBecsDebitBuilder {
                    bsb_number: Deserialize::default(),
                    fingerprint: Deserialize::default(),
                    last4: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bsb_number" => Deserialize::begin(&mut self.builder.bsb_number),
                "fingerprint" => Deserialize::begin(&mut self.builder.fingerprint),
                "last4" => Deserialize::begin(&mut self.builder.last4),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(bsb_number), Some(fingerprint), Some(last4)) = (
                self.builder.bsb_number.take(),
                self.builder.fingerprint.take(),
                self.builder.last4.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(SourceTypeAuBecsDebit { bsb_number, fingerprint, last4 });
            Ok(())
        }
    }
};
