#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PersonAdditionalTosAcceptance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PersonAdditionalTosAcceptance").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PersonAdditionalTosAcceptanceBuilder {
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
            Self { date: Some(None), ip: Some(None), user_agent: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self { date: Some(date), ip: Some(ip), user_agent: Some(user_agent) } => {
                    Some(Self::Out { date, ip, user_agent })
                }
                builder => {
                    *self = builder;
                    None
                }
            }
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for PersonAdditionalTosAcceptance {
        type Builder = PersonAdditionalTosAcceptanceBuilder;
    }

    impl FromValueOpt for PersonAdditionalTosAcceptance {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
