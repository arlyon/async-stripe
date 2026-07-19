#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SourceTypeAchDebit {
    pub bank_name: Option<String>,
    pub country: Option<String>,
    pub fingerprint: Option<String>,
    pub last4: Option<String>,
    pub routing_number: Option<String>,
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SourceTypeAchDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SourceTypeAchDebit").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SourceTypeAchDebitBuilder {
    bank_name: Option<Option<String>>,
    country: Option<Option<String>>,
    fingerprint: Option<Option<String>>,
    last4: Option<Option<String>>,
    routing_number: Option<Option<String>>,
    type_: Option<Option<String>>,
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

    impl Deserialize for SourceTypeAchDebit {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceTypeAchDebit>,
        builder: SourceTypeAchDebitBuilder,
    }

    impl Visitor for Place<SourceTypeAchDebit> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SourceTypeAchDebitBuilder {
                    bank_name: Deserialize::default(),
                    country: Deserialize::default(),
                    fingerprint: Deserialize::default(),
                    last4: Deserialize::default(),
                    routing_number: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bank_name" => Deserialize::begin(&mut self.builder.bank_name),
                "country" => Deserialize::begin(&mut self.builder.country),
                "fingerprint" => Deserialize::begin(&mut self.builder.fingerprint),
                "last4" => Deserialize::begin(&mut self.builder.last4),
                "routing_number" => Deserialize::begin(&mut self.builder.routing_number),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(bank_name),
                Some(country),
                Some(fingerprint),
                Some(last4),
                Some(routing_number),
                Some(type_),
            ) = (
                self.builder.bank_name.take(),
                self.builder.country.take(),
                self.builder.fingerprint.take(),
                self.builder.last4.take(),
                self.builder.routing_number.take(),
                self.builder.type_.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(SourceTypeAchDebit {
                bank_name,
                country,
                fingerprint,
                last4,
                routing_number,
                type_,
            });
            Ok(())
        }
    }
};
