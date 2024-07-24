#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ClimateRemovalsBeneficiary {
    /// Publicly displayable name for the end beneficiary of carbon removal.
    pub public_name: String,
}
#[doc(hidden)]
pub struct ClimateRemovalsBeneficiaryBuilder {
    public_name: Option<String>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for ClimateRemovalsBeneficiary {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ClimateRemovalsBeneficiary>,
        builder: ClimateRemovalsBeneficiaryBuilder,
    }

    impl Visitor for Place<ClimateRemovalsBeneficiary> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ClimateRemovalsBeneficiaryBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ClimateRemovalsBeneficiaryBuilder {
        type Out = ClimateRemovalsBeneficiary;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "public_name" => Deserialize::begin(&mut self.public_name),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { public_name: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out { public_name: self.public_name.take()? })
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

    impl ObjectDeser for ClimateRemovalsBeneficiary {
        type Builder = ClimateRemovalsBeneficiaryBuilder;
    }

    impl FromValueOpt for ClimateRemovalsBeneficiary {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ClimateRemovalsBeneficiaryBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "public_name" => b.public_name = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
