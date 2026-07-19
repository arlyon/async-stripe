#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxProductResourcePostalAddress {
    /// City, district, suburb, town, or village.
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: String,
    /// Address line 1, such as the street, PO Box, or company name.
    pub line1: Option<String>,
    /// Address line 2, such as the apartment, suite, unit, or building.
    pub line2: Option<String>,
    /// ZIP or postal code.
    pub postal_code: Option<String>,
    /// State/province as an [ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2) subdivision code, without country prefix, such as "NY" or "TX".
    pub state: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxProductResourcePostalAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TaxProductResourcePostalAddress").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TaxProductResourcePostalAddressBuilder {
    city: Option<Option<String>>,
    country: Option<String>,
    line1: Option<Option<String>>,
    line2: Option<Option<String>>,
    postal_code: Option<Option<String>>,
    state: Option<Option<String>>,
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

    impl Deserialize for TaxProductResourcePostalAddress {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductResourcePostalAddress>,
        builder: TaxProductResourcePostalAddressBuilder,
    }

    impl Visitor for Place<TaxProductResourcePostalAddress> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TaxProductResourcePostalAddressBuilder {
                    city: Deserialize::default(),
                    country: Deserialize::default(),
                    line1: Deserialize::default(),
                    line2: Deserialize::default(),
                    postal_code: Deserialize::default(),
                    state: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "city" => Deserialize::begin(&mut self.builder.city),
                "country" => Deserialize::begin(&mut self.builder.country),
                "line1" => Deserialize::begin(&mut self.builder.line1),
                "line2" => Deserialize::begin(&mut self.builder.line2),
                "postal_code" => Deserialize::begin(&mut self.builder.postal_code),
                "state" => Deserialize::begin(&mut self.builder.state),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(city),
                Some(country),
                Some(line1),
                Some(line2),
                Some(postal_code),
                Some(state),
            ) = (
                self.builder.city.take(),
                self.builder.country.take(),
                self.builder.line1.take(),
                self.builder.line2.take(),
                self.builder.postal_code.take(),
                self.builder.state.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(TaxProductResourcePostalAddress {
                city,
                country,
                line1,
                line2,
                postal_code,
                state,
            });
            Ok(())
        }
    }
};
