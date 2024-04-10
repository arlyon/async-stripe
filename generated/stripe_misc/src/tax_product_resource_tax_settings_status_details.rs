#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxProductResourceTaxSettingsStatusDetails {
    pub active: Option<stripe_misc::TaxProductResourceTaxSettingsStatusDetailsResourceActive>,
    pub pending: Option<stripe_misc::TaxProductResourceTaxSettingsStatusDetailsResourcePending>,
}
#[doc(hidden)]
pub struct TaxProductResourceTaxSettingsStatusDetailsBuilder {
    active: Option<Option<stripe_misc::TaxProductResourceTaxSettingsStatusDetailsResourceActive>>,
    pending: Option<Option<stripe_misc::TaxProductResourceTaxSettingsStatusDetailsResourcePending>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TaxProductResourceTaxSettingsStatusDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductResourceTaxSettingsStatusDetails>,
        builder: TaxProductResourceTaxSettingsStatusDetailsBuilder,
    }

    impl Visitor for Place<TaxProductResourceTaxSettingsStatusDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TaxProductResourceTaxSettingsStatusDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TaxProductResourceTaxSettingsStatusDetailsBuilder {
        type Out = TaxProductResourceTaxSettingsStatusDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "active" => Deserialize::begin(&mut self.active),
                "pending" => Deserialize::begin(&mut self.pending),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { active: Deserialize::default(), pending: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out { active: self.active?, pending: self.pending.take()? })
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

    impl ObjectDeser for TaxProductResourceTaxSettingsStatusDetails {
        type Builder = TaxProductResourceTaxSettingsStatusDetailsBuilder;
    }

    impl FromValueOpt for TaxProductResourceTaxSettingsStatusDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TaxProductResourceTaxSettingsStatusDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "active" => b.active = Some(FromValueOpt::from_value(v)?),
                    "pending" => b.pending = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
