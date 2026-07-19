#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct RadarReviewResourceSession {
    /// The browser used in this browser session (e.g., `Chrome`).
    pub browser: Option<String>,
    /// Information about the device used for the browser session (e.g., `Samsung SM-G930T`).
    pub device: Option<String>,
    /// The platform for the browser session (e.g., `Macintosh`).
    pub platform: Option<String>,
    /// The version for the browser session (e.g., `61.0.3163.100`).
    pub version: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RadarReviewResourceSession {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RadarReviewResourceSession").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct RadarReviewResourceSessionBuilder {
    browser: Option<Option<String>>,
    device: Option<Option<String>>,
    platform: Option<Option<String>>,
    version: Option<Option<String>>,
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

    impl Deserialize for RadarReviewResourceSession {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<RadarReviewResourceSession>,
        builder: RadarReviewResourceSessionBuilder,
    }

    impl Visitor for Place<RadarReviewResourceSession> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: RadarReviewResourceSessionBuilder {
                    browser: Deserialize::default(),
                    device: Deserialize::default(),
                    platform: Deserialize::default(),
                    version: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "browser" => Deserialize::begin(&mut self.builder.browser),
                "device" => Deserialize::begin(&mut self.builder.device),
                "platform" => Deserialize::begin(&mut self.builder.platform),
                "version" => Deserialize::begin(&mut self.builder.version),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(browser), Some(device), Some(platform), Some(version)) = (
                self.builder.browser.take(),
                self.builder.device.take(),
                self.builder.platform.take(),
                self.builder.version.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(RadarReviewResourceSession { browser, device, platform, version });
            Ok(())
        }
    }
};
