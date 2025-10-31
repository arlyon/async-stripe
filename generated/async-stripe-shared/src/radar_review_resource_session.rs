#[derive(Clone, Debug)]
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
                builder: RadarReviewResourceSessionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for RadarReviewResourceSessionBuilder {
        type Out = RadarReviewResourceSession;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "browser" => Deserialize::begin(&mut self.browser),
                "device" => Deserialize::begin(&mut self.device),
                "platform" => Deserialize::begin(&mut self.platform),
                "version" => Deserialize::begin(&mut self.version),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                browser: Deserialize::default(),
                device: Deserialize::default(),
                platform: Deserialize::default(),
                version: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(browser), Some(device), Some(platform), Some(version)) = (
                self.browser.take(),
                self.device.take(),
                self.platform.take(),
                self.version.take(),
            ) else {
                return None;
            };
            Some(Self::Out { browser, device, platform, version })
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

    impl ObjectDeser for RadarReviewResourceSession {
        type Builder = RadarReviewResourceSessionBuilder;
    }

    impl FromValueOpt for RadarReviewResourceSession {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = RadarReviewResourceSessionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "browser" => b.browser = FromValueOpt::from_value(v),
                    "device" => b.device = FromValueOpt::from_value(v),
                    "platform" => b.platform = FromValueOpt::from_value(v),
                    "version" => b.version = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
