#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalConfigurationConfigurationResourceRebootWindow {
    /// Integer between 0 to 23 that represents the end hour of the reboot time window.
    /// The value must be different than the start_hour.
    pub end_hour: i64,
    /// Integer between 0 to 23 that represents the start hour of the reboot time window.
    pub start_hour: i64,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TerminalConfigurationConfigurationResourceRebootWindow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TerminalConfigurationConfigurationResourceRebootWindow")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TerminalConfigurationConfigurationResourceRebootWindowBuilder {
    end_hour: Option<i64>,
    start_hour: Option<i64>,
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

    impl Deserialize for TerminalConfigurationConfigurationResourceRebootWindow {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TerminalConfigurationConfigurationResourceRebootWindow>,
        builder: TerminalConfigurationConfigurationResourceRebootWindowBuilder,
    }

    impl Visitor for Place<TerminalConfigurationConfigurationResourceRebootWindow> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder:
                    TerminalConfigurationConfigurationResourceRebootWindowBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TerminalConfigurationConfigurationResourceRebootWindowBuilder {
        type Out = TerminalConfigurationConfigurationResourceRebootWindow;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "end_hour" => Deserialize::begin(&mut self.end_hour),
                "start_hour" => Deserialize::begin(&mut self.start_hour),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { end_hour: None, start_hour: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self { end_hour: Some(end_hour), start_hour: Some(start_hour) } => {
                    Some(Self::Out { end_hour, start_hour })
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

    impl ObjectDeser for TerminalConfigurationConfigurationResourceRebootWindow {
        type Builder = TerminalConfigurationConfigurationResourceRebootWindowBuilder;
    }

    impl FromValueOpt for TerminalConfigurationConfigurationResourceRebootWindow {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
