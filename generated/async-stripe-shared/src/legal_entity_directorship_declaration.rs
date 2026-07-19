#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for LegalEntityDirectorshipDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("LegalEntityDirectorshipDeclaration").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: LegalEntityDirectorshipDeclarationBuilder {
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
            *self.out = Some(LegalEntityDirectorshipDeclaration { date, ip, user_agent });
            Ok(())
        }
    }
};
