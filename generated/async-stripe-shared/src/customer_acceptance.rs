#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CustomerAcceptance {
    /// The time that the customer accepts the mandate.
    pub accepted_at: Option<stripe_types::Timestamp>,
    pub offline: Option<stripe_shared::OfflineAcceptance>,
    pub online: Option<stripe_shared::OnlineAcceptance>,
    /// The mandate includes the type of customer acceptance information, such as: `online` or `offline`.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: stripe_shared::TerminalReaderStatus,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CustomerAcceptance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CustomerAcceptance").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct CustomerAcceptanceBuilder {
    accepted_at: Option<Option<stripe_types::Timestamp>>,
    offline: Option<Option<stripe_shared::OfflineAcceptance>>,
    online: Option<Option<stripe_shared::OnlineAcceptance>>,
    type_: Option<stripe_shared::TerminalReaderStatus>,
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

    impl Deserialize for CustomerAcceptance {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CustomerAcceptance>,
        builder: CustomerAcceptanceBuilder,
    }

    impl Visitor for Place<CustomerAcceptance> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CustomerAcceptanceBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for CustomerAcceptanceBuilder {
        type Out = CustomerAcceptance;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "accepted_at" => Deserialize::begin(&mut self.accepted_at),
                "offline" => Deserialize::begin(&mut self.offline),
                "online" => Deserialize::begin(&mut self.online),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { accepted_at: Some(None), offline: Some(None), online: Some(None), type_: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(accepted_at), Some(offline), Some(online), Some(type_)) =
                (self.accepted_at, self.offline, self.online.take(), self.type_.take())
            else {
                return None;
            };
            Some(Self::Out { accepted_at, offline, online, type_ })
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

    impl ObjectDeser for CustomerAcceptance {
        type Builder = CustomerAcceptanceBuilder;
    }

    impl FromValueOpt for CustomerAcceptance {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = CustomerAcceptanceBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "accepted_at" => b.accepted_at = FromValueOpt::from_value(v),
                    "offline" => b.offline = FromValueOpt::from_value(v),
                    "online" => b.online = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The mandate includes the type of customer acceptance information, such as: `online` or `offline`.
pub type CustomerAcceptanceType = stripe_shared::TerminalReaderStatus;
