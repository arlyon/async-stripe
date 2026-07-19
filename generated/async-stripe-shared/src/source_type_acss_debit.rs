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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: SourceTypeAcssDebitBuilder {
                    bank_address_city: Deserialize::default(),
                    bank_address_line_1: Deserialize::default(),
                    bank_address_line_2: Deserialize::default(),
                    bank_address_postal_code: Deserialize::default(),
                    bank_name: Deserialize::default(),
                    category: Deserialize::default(),
                    country: Deserialize::default(),
                    fingerprint: Deserialize::default(),
                    last4: Deserialize::default(),
                    routing_number: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bank_address_city" => Deserialize::begin(&mut self.builder.bank_address_city),
                "bank_address_line_1" => Deserialize::begin(&mut self.builder.bank_address_line_1),
                "bank_address_line_2" => Deserialize::begin(&mut self.builder.bank_address_line_2),
                "bank_address_postal_code" => {
                    Deserialize::begin(&mut self.builder.bank_address_postal_code)
                }
                "bank_name" => Deserialize::begin(&mut self.builder.bank_name),
                "category" => Deserialize::begin(&mut self.builder.category),
                "country" => Deserialize::begin(&mut self.builder.country),
                "fingerprint" => Deserialize::begin(&mut self.builder.fingerprint),
                "last4" => Deserialize::begin(&mut self.builder.last4),
                "routing_number" => Deserialize::begin(&mut self.builder.routing_number),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(bank_address_city),
                Some(bank_address_line_1),
                Some(bank_address_line_2),
                Some(bank_address_postal_code),
                Some(bank_name),
                Some(category),
                Some(country),
                Some(fingerprint),
                Some(last4),
                Some(routing_number),
            ) = (
                self.builder.bank_address_city.take(),
                self.builder.bank_address_line_1.take(),
                self.builder.bank_address_line_2.take(),
                self.builder.bank_address_postal_code.take(),
                self.builder.bank_name.take(),
                self.builder.category.take(),
                self.builder.country.take(),
                self.builder.fingerprint.take(),
                self.builder.last4.take(),
                self.builder.routing_number.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(SourceTypeAcssDebit {
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
            });
            Ok(())
        }
    }
};
