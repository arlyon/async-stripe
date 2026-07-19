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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: TerminalConfigurationConfigurationResourceRebootWindowBuilder {
                    end_hour: Deserialize::default(),
                    start_hour: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "end_hour" => Deserialize::begin(&mut self.builder.end_hour),
                "start_hour" => Deserialize::begin(&mut self.builder.start_hour),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(end_hour), Some(start_hour)) =
                (self.builder.end_hour, self.builder.start_hour)
            else {
                return Ok(());
            };
            *self.out = Some(TerminalConfigurationConfigurationResourceRebootWindow {
                end_hour,
                start_hour,
            });
            Ok(())
        }
    }
};
