#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxProductResourcePostalAddress {
    /// City, district, suburb, town, or village.
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: String,
    /// Address line 1 (e.g., street, PO Box, or company name).
    pub line1: Option<String>,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    pub line2: Option<String>,
    /// ZIP or postal code.
    pub postal_code: Option<String>,
    /// State/province as an [ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2) subdivision code, without country prefix.
    /// Example: "NY" or "TX".
    pub state: Option<String>,
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

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: TaxProductResourcePostalAddressBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TaxProductResourcePostalAddressBuilder {
        type Out = TaxProductResourcePostalAddress;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "city" => Deserialize::begin(&mut self.city),
                "country" => Deserialize::begin(&mut self.country),
                "line1" => Deserialize::begin(&mut self.line1),
                "line2" => Deserialize::begin(&mut self.line2),
                "postal_code" => Deserialize::begin(&mut self.postal_code),
                "state" => Deserialize::begin(&mut self.state),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                city: Deserialize::default(),
                country: Deserialize::default(),
                line1: Deserialize::default(),
                line2: Deserialize::default(),
                postal_code: Deserialize::default(),
                state: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                city: self.city.take()?,
                country: self.country.take()?,
                line1: self.line1.take()?,
                line2: self.line2.take()?,
                postal_code: self.postal_code.take()?,
                state: self.state.take()?,
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

    impl ObjectDeser for TaxProductResourcePostalAddress {
        type Builder = TaxProductResourcePostalAddressBuilder;
    }

    impl FromValueOpt for TaxProductResourcePostalAddress {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TaxProductResourcePostalAddressBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "city" => b.city = Some(FromValueOpt::from_value(v)?),
                    "country" => b.country = Some(FromValueOpt::from_value(v)?),
                    "line1" => b.line1 = Some(FromValueOpt::from_value(v)?),
                    "line2" => b.line2 = Some(FromValueOpt::from_value(v)?),
                    "postal_code" => b.postal_code = Some(FromValueOpt::from_value(v)?),
                    "state" => b.state = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
