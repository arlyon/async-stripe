#[derive(Clone, Debug)]
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
                builder: SourceTypeSepaDebitBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SourceTypeSepaDebitBuilder {
        type Out = SourceTypeSepaDebit;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bank_code" => Deserialize::begin(&mut self.bank_code),
                "branch_code" => Deserialize::begin(&mut self.branch_code),
                "country" => Deserialize::begin(&mut self.country),
                "fingerprint" => Deserialize::begin(&mut self.fingerprint),
                "last4" => Deserialize::begin(&mut self.last4),
                "mandate_reference" => Deserialize::begin(&mut self.mandate_reference),
                "mandate_url" => Deserialize::begin(&mut self.mandate_url),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                bank_code: Deserialize::default(),
                branch_code: Deserialize::default(),
                country: Deserialize::default(),
                fingerprint: Deserialize::default(),
                last4: Deserialize::default(),
                mandate_reference: Deserialize::default(),
                mandate_url: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(bank_code),
                Some(branch_code),
                Some(country),
                Some(fingerprint),
                Some(last4),
                Some(mandate_reference),
                Some(mandate_url),
            ) = (
                self.bank_code.take(),
                self.branch_code.take(),
                self.country.take(),
                self.fingerprint.take(),
                self.last4.take(),
                self.mandate_reference.take(),
                self.mandate_url.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                bank_code,
                branch_code,
                country,
                fingerprint,
                last4,
                mandate_reference,
                mandate_url,
            })
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

    impl ObjectDeser for SourceTypeSepaDebit {
        type Builder = SourceTypeSepaDebitBuilder;
    }

    impl FromValueOpt for SourceTypeSepaDebit {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SourceTypeSepaDebitBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "bank_code" => b.bank_code = FromValueOpt::from_value(v),
                    "branch_code" => b.branch_code = FromValueOpt::from_value(v),
                    "country" => b.country = FromValueOpt::from_value(v),
                    "fingerprint" => b.fingerprint = FromValueOpt::from_value(v),
                    "last4" => b.last4 = FromValueOpt::from_value(v),
                    "mandate_reference" => b.mandate_reference = FromValueOpt::from_value(v),
                    "mandate_url" => b.mandate_url = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
