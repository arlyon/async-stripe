#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalConfigurationConfigurationResourceRebootWindow {
    /// Integer between 0 to 23 that represents the end hour of the reboot time window.
    /// The value must be different than the start_hour.
    pub end_hour: i64,
    /// Integer between 0 to 23 that represents the start hour of the reboot time window.
    pub start_hour: i64,
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
            Self { end_hour: Deserialize::default(), start_hour: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(end_hour), Some(start_hour)) = (self.end_hour, self.start_hour) else {
                return None;
            };
            Some(Self::Out { end_hour, start_hour })
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

    impl ObjectDeser for TerminalConfigurationConfigurationResourceRebootWindow {
        type Builder = TerminalConfigurationConfigurationResourceRebootWindowBuilder;
    }

    impl FromValueOpt for TerminalConfigurationConfigurationResourceRebootWindow {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                TerminalConfigurationConfigurationResourceRebootWindowBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "end_hour" => b.end_hour = FromValueOpt::from_value(v),
                    "start_hour" => b.start_hour = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
