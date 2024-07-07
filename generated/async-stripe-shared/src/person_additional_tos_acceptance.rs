#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PersonAdditionalTosAcceptance {
    /// The Unix timestamp marking when the legal guardian accepted the service agreement.
    pub date: Option<stripe_types::Timestamp>,
    /// The IP address from which the legal guardian accepted the service agreement.
    pub ip: Option<String>,
    /// The user agent of the browser from which the legal guardian accepted the service agreement.
    pub user_agent: Option<String>,
}
#[doc(hidden)]
pub struct PersonAdditionalTosAcceptanceBuilder {
    date: Option<Option<stripe_types::Timestamp>>,
    ip: Option<Option<String>>,
    user_agent: Option<Option<String>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PersonAdditionalTosAcceptance {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PersonAdditionalTosAcceptance>,
        builder: PersonAdditionalTosAcceptanceBuilder,
    }

    impl Visitor for Place<PersonAdditionalTosAcceptance> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PersonAdditionalTosAcceptanceBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PersonAdditionalTosAcceptanceBuilder {
        type Out = PersonAdditionalTosAcceptance;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "date" => Deserialize::begin(&mut self.date),
                "ip" => Deserialize::begin(&mut self.ip),
                "user_agent" => Deserialize::begin(&mut self.user_agent),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                date: Deserialize::default(),
                ip: Deserialize::default(),
                user_agent: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                date: self.date?,
                ip: self.ip.take()?,
                user_agent: self.user_agent.take()?,
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

    impl ObjectDeser for PersonAdditionalTosAcceptance {
        type Builder = PersonAdditionalTosAcceptanceBuilder;
    }

    impl FromValueOpt for PersonAdditionalTosAcceptance {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PersonAdditionalTosAcceptanceBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "date" => b.date = Some(FromValueOpt::from_value(v)?),
                    "ip" => b.ip = Some(FromValueOpt::from_value(v)?),
                    "user_agent" => b.user_agent = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
