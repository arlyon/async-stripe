#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SourceTypeAcssDebit {
    pub bank_address_city: Option<String>,
    pub bank_address_line_1: Option<String>,
    pub bank_address_line_2: Option<String>,
    pub bank_address_postal_code: Option<String>,
    pub bank_name: Option<String>,
    pub category: Option<String>,
    pub country: Option<String>,
    pub fingerprint: Option<String>,
    pub last4: Option<String>,
    pub routing_number: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SourceTypeAcssDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SourceTypeAcssDebit").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SourceTypeAcssDebitBuilder {
    bank_address_city: Option<Option<String>>,
    bank_address_line_1: Option<Option<String>>,
    bank_address_line_2: Option<Option<String>>,
    bank_address_postal_code: Option<Option<String>>,
    bank_name: Option<Option<String>>,
    category: Option<Option<String>>,
    country: Option<Option<String>>,
    fingerprint: Option<Option<String>>,
    last4: Option<Option<String>>,
    routing_number: Option<Option<String>>,
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

    impl Deserialize for SourceTypeAcssDebit {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceTypeAcssDebit>,
        builder: SourceTypeAcssDebitBuilder,
    }

    impl Visitor for Place<SourceTypeAcssDebit> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SourceTypeAcssDebitBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SourceTypeAcssDebitBuilder {
        type Out = SourceTypeAcssDebit;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bank_address_city" => Deserialize::begin(&mut self.bank_address_city),
                "bank_address_line_1" => Deserialize::begin(&mut self.bank_address_line_1),
                "bank_address_line_2" => Deserialize::begin(&mut self.bank_address_line_2),
                "bank_address_postal_code" => {
                    Deserialize::begin(&mut self.bank_address_postal_code)
                }
                "bank_name" => Deserialize::begin(&mut self.bank_name),
                "category" => Deserialize::begin(&mut self.category),
                "country" => Deserialize::begin(&mut self.country),
                "fingerprint" => Deserialize::begin(&mut self.fingerprint),
                "last4" => Deserialize::begin(&mut self.last4),
                "routing_number" => Deserialize::begin(&mut self.routing_number),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                bank_address_city: Some(None),
                bank_address_line_1: Some(None),
                bank_address_line_2: Some(None),
                bank_address_postal_code: Some(None),
                bank_name: Some(None),
                category: Some(None),
                country: Some(None),
                fingerprint: Some(None),
                last4: Some(None),
                routing_number: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self {
                    bank_address_city: Some(bank_address_city),
                    bank_address_line_1: Some(bank_address_line_1),
                    bank_address_line_2: Some(bank_address_line_2),
                    bank_address_postal_code: Some(bank_address_postal_code),
                    bank_name: Some(bank_name),
                    category: Some(category),
                    country: Some(country),
                    fingerprint: Some(fingerprint),
                    last4: Some(last4),
                    routing_number: Some(routing_number),
                } => Some(Self::Out {
                    bank_address_city,
                    bank_address_line_1,
                    bank_address_line_2,
                    bank_address_postal_code,
                    bank_name,
                    category,
                    country,
                    fingerprint,
                    last4,
                    routing_number,
                }),
                builder => {
                    *self = builder;
                    None
                }
            }
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for SourceTypeAcssDebit {
        type Builder = SourceTypeAcssDebitBuilder;
    }

    impl FromValueOpt for SourceTypeAcssDebit {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
