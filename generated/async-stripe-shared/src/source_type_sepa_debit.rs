#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SourceTypeSepaDebit {
    pub bank_code: Option<String>,
    pub branch_code: Option<String>,
    pub country: Option<String>,
    pub fingerprint: Option<String>,
    pub last4: Option<String>,
    pub mandate_reference: Option<String>,
    pub mandate_url: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SourceTypeSepaDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SourceTypeSepaDebit").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SourceTypeSepaDebitBuilder {
    bank_code: Option<Option<String>>,
    branch_code: Option<Option<String>>,
    country: Option<Option<String>>,
    fingerprint: Option<Option<String>>,
    last4: Option<Option<String>>,
    mandate_reference: Option<Option<String>>,
    mandate_url: Option<Option<String>>,
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

    impl Deserialize for SourceTypeSepaDebit {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceTypeSepaDebit>,
        builder: SourceTypeSepaDebitBuilder,
    }

    impl Visitor for Place<SourceTypeSepaDebit> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SourceTypeSepaDebitBuilder {
                    bank_code: Deserialize::default(),
                    branch_code: Deserialize::default(),
                    country: Deserialize::default(),
                    fingerprint: Deserialize::default(),
                    last4: Deserialize::default(),
                    mandate_reference: Deserialize::default(),
                    mandate_url: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bank_code" => Deserialize::begin(&mut self.builder.bank_code),
                "branch_code" => Deserialize::begin(&mut self.builder.branch_code),
                "country" => Deserialize::begin(&mut self.builder.country),
                "fingerprint" => Deserialize::begin(&mut self.builder.fingerprint),
                "last4" => Deserialize::begin(&mut self.builder.last4),
                "mandate_reference" => Deserialize::begin(&mut self.builder.mandate_reference),
                "mandate_url" => Deserialize::begin(&mut self.builder.mandate_url),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(bank_code),
                Some(branch_code),
                Some(country),
                Some(fingerprint),
                Some(last4),
                Some(mandate_reference),
                Some(mandate_url),
            ) = (
                self.builder.bank_code.take(),
                self.builder.branch_code.take(),
                self.builder.country.take(),
                self.builder.fingerprint.take(),
                self.builder.last4.take(),
                self.builder.mandate_reference.take(),
                self.builder.mandate_url.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(SourceTypeSepaDebit {
                bank_code,
                branch_code,
                country,
                fingerprint,
                last4,
                mandate_reference,
                mandate_url,
            });
            Ok(())
        }
    }
};
