#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct LegalEntityDirectorshipDeclaration {
    /// The Unix timestamp marking when the directorship declaration attestation was made.
    pub date: Option<stripe_types::Timestamp>,
    /// The IP address from which the directorship declaration attestation was made.
    pub ip: Option<String>,
    /// The user-agent string from the browser where the directorship declaration attestation was made.
    pub user_agent: Option<String>,
}
#[doc(hidden)]
pub struct LegalEntityDirectorshipDeclarationBuilder {
    date: Option<Option<stripe_types::Timestamp>>,
    ip: Option<Option<String>>,
    user_agent: Option<Option<String>>,
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

    impl Deserialize for LegalEntityDirectorshipDeclaration {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<LegalEntityDirectorshipDeclaration>,
        builder: LegalEntityDirectorshipDeclarationBuilder,
    }

    impl Visitor for Place<LegalEntityDirectorshipDeclaration> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: LegalEntityDirectorshipDeclarationBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for LegalEntityDirectorshipDeclarationBuilder {
        type Out = LegalEntityDirectorshipDeclaration;
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
            let (Some(date), Some(ip), Some(user_agent)) =
                (self.date, self.ip.take(), self.user_agent.take())
            else {
                return None;
            };
            Some(Self::Out { date, ip, user_agent })
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

    impl ObjectDeser for LegalEntityDirectorshipDeclaration {
        type Builder = LegalEntityDirectorshipDeclarationBuilder;
    }

    impl FromValueOpt for LegalEntityDirectorshipDeclaration {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = LegalEntityDirectorshipDeclarationBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "date" => b.date = FromValueOpt::from_value(v),
                    "ip" => b.ip = FromValueOpt::from_value(v),
                    "user_agent" => b.user_agent = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
