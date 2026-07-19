/// Options to configure Radar.
/// See [Radar Session](https://docs.stripe.com/radar/radar-session) for more information.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct RadarRadarOptions {
    /// A [Radar Session](https://docs.stripe.com/radar/radar-session) is a snapshot of the browser metadata and device details that help Radar make more accurate predictions on your payments.
    pub session: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RadarRadarOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RadarRadarOptions").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct RadarRadarOptionsBuilder {
    session: Option<Option<String>>,
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

    impl Deserialize for RadarRadarOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<RadarRadarOptions>,
        builder: RadarRadarOptionsBuilder,
    }

    impl Visitor for Place<RadarRadarOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: RadarRadarOptionsBuilder { session: Deserialize::default() },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "session" => Deserialize::begin(&mut self.builder.session),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(session),) = (self.builder.session.take(),) else {
                return Ok(());
            };
            *self.out = Some(RadarRadarOptions { session });
            Ok(())
        }
    }
};
