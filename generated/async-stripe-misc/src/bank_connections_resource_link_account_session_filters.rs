#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BankConnectionsResourceLinkAccountSessionFilters {
    /// List of countries from which to filter accounts.
    pub countries: Option<Vec<String>>,
}
#[doc(hidden)]
pub struct BankConnectionsResourceLinkAccountSessionFiltersBuilder {
    countries: Option<Option<Vec<String>>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for BankConnectionsResourceLinkAccountSessionFilters {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BankConnectionsResourceLinkAccountSessionFilters>,
        builder: BankConnectionsResourceLinkAccountSessionFiltersBuilder,
    }

    impl Visitor for Place<BankConnectionsResourceLinkAccountSessionFilters> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BankConnectionsResourceLinkAccountSessionFiltersBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BankConnectionsResourceLinkAccountSessionFiltersBuilder {
        type Out = BankConnectionsResourceLinkAccountSessionFilters;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "countries" => Deserialize::begin(&mut self.countries),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { countries: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out { countries: self.countries.take()? })
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

    impl ObjectDeser for BankConnectionsResourceLinkAccountSessionFilters {
        type Builder = BankConnectionsResourceLinkAccountSessionFiltersBuilder;
    }

    impl FromValueOpt for BankConnectionsResourceLinkAccountSessionFilters {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BankConnectionsResourceLinkAccountSessionFiltersBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "countries" => b.countries = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
