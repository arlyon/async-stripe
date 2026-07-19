#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct LegalEntityUboDeclaration {
    /// The Unix timestamp marking when the beneficial owner attestation was made.
    pub date: Option<stripe_types::Timestamp>,
    /// The IP address from which the beneficial owner attestation was made.
    pub ip: Option<String>,
    /// The user-agent string from the browser where the beneficial owner attestation was made.
    pub user_agent: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for LegalEntityUboDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("LegalEntityUboDeclaration").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct LegalEntityUboDeclarationBuilder {
    date: Option<Option<stripe_types::Timestamp>>,
    ip: Option<Option<String>>,
    user_agent: Option<Option<String>>,
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

    impl Deserialize for LegalEntityUboDeclaration {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<LegalEntityUboDeclaration>,
        builder: LegalEntityUboDeclarationBuilder,
    }

    impl Visitor for Place<LegalEntityUboDeclaration> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: LegalEntityUboDeclarationBuilder {
                    date: Deserialize::default(),
                    ip: Deserialize::default(),
                    user_agent: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "date" => Deserialize::begin(&mut self.builder.date),
                "ip" => Deserialize::begin(&mut self.builder.ip),
                "user_agent" => Deserialize::begin(&mut self.builder.user_agent),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(date), Some(ip), Some(user_agent)) =
                (self.builder.date, self.builder.ip.take(), self.builder.user_agent.take())
            else {
                return Ok(());
            };
            *self.out = Some(LegalEntityUboDeclaration { date, ip, user_agent });
            Ok(())
        }
    }
};
