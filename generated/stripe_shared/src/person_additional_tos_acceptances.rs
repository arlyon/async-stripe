#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PersonAdditionalTosAcceptances {
    pub account: stripe_shared::PersonAdditionalTosAcceptance,
}
#[doc(hidden)]
pub struct PersonAdditionalTosAcceptancesBuilder {
    account: Option<stripe_shared::PersonAdditionalTosAcceptance>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PersonAdditionalTosAcceptances {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PersonAdditionalTosAcceptances>,
        builder: PersonAdditionalTosAcceptancesBuilder,
    }

    impl Visitor for Place<PersonAdditionalTosAcceptances> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PersonAdditionalTosAcceptancesBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PersonAdditionalTosAcceptancesBuilder {
        type Out = PersonAdditionalTosAcceptances;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account" => Deserialize::begin(&mut self.account),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { account: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out { account: self.account.take()? })
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

    impl ObjectDeser for PersonAdditionalTosAcceptances {
        type Builder = PersonAdditionalTosAcceptancesBuilder;
    }

    impl FromValueOpt for PersonAdditionalTosAcceptances {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PersonAdditionalTosAcceptancesBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "account" => b.account = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
