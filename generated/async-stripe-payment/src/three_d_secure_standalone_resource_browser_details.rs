/// Contains details on the browser for a standalone 3DS Authentication.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ThreeDSecureStandaloneResourceBrowserDetails {
    /// The HTTP accept headers from the cardholder's browser.
    pub accept_header: String,
    /// The color depth of the cardholder’s screen.
    pub color_depth: Option<i64>,
    /// The IP address of the browser.
    pub ip_address: String,
    /// The cardholder browser’s ability to execute Java.
    pub java_enabled: Option<bool>,
    /// The cardholder browser’s ability to execute JavaScript.
    pub javascript_enabled: bool,
    /// An IETF BCP 47 language tag representing the browser language.
    pub language: String,
    /// The total height of the cardholder’s screen in pixels.
    pub screen_height: Option<i64>,
    /// The total width of the cardholder’s screen in pixels.
    pub screen_width: Option<i64>,
    /// The time difference between UTC time and the local time of the cardholder’s browser, in minutes.
    pub timezone_offset: Option<i64>,
    /// The browser user agent.
    pub user_agent: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ThreeDSecureStandaloneResourceBrowserDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ThreeDSecureStandaloneResourceBrowserDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ThreeDSecureStandaloneResourceBrowserDetailsBuilder {
    accept_header: Option<String>,
    color_depth: Option<Option<i64>>,
    ip_address: Option<String>,
    java_enabled: Option<Option<bool>>,
    javascript_enabled: Option<bool>,
    language: Option<String>,
    screen_height: Option<Option<i64>>,
    screen_width: Option<Option<i64>>,
    timezone_offset: Option<Option<i64>>,
    user_agent: Option<String>,
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

    impl Deserialize for ThreeDSecureStandaloneResourceBrowserDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ThreeDSecureStandaloneResourceBrowserDetails>,
        builder: ThreeDSecureStandaloneResourceBrowserDetailsBuilder,
    }

    impl Visitor for Place<ThreeDSecureStandaloneResourceBrowserDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ThreeDSecureStandaloneResourceBrowserDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ThreeDSecureStandaloneResourceBrowserDetailsBuilder {
        type Out = ThreeDSecureStandaloneResourceBrowserDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "accept_header" => Deserialize::begin(&mut self.accept_header),
                "color_depth" => Deserialize::begin(&mut self.color_depth),
                "ip_address" => Deserialize::begin(&mut self.ip_address),
                "java_enabled" => Deserialize::begin(&mut self.java_enabled),
                "javascript_enabled" => Deserialize::begin(&mut self.javascript_enabled),
                "language" => Deserialize::begin(&mut self.language),
                "screen_height" => Deserialize::begin(&mut self.screen_height),
                "screen_width" => Deserialize::begin(&mut self.screen_width),
                "timezone_offset" => Deserialize::begin(&mut self.timezone_offset),
                "user_agent" => Deserialize::begin(&mut self.user_agent),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                accept_header: None,
                color_depth: Some(None),
                ip_address: None,
                java_enabled: Some(None),
                javascript_enabled: None,
                language: None,
                screen_height: Some(None),
                screen_width: Some(None),
                timezone_offset: Some(None),
                user_agent: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(accept_header),
                Some(color_depth),
                Some(ip_address),
                Some(java_enabled),
                Some(javascript_enabled),
                Some(language),
                Some(screen_height),
                Some(screen_width),
                Some(timezone_offset),
                Some(user_agent),
            ) = (
                self.accept_header.take(),
                self.color_depth,
                self.ip_address.take(),
                self.java_enabled,
                self.javascript_enabled,
                self.language.take(),
                self.screen_height,
                self.screen_width,
                self.timezone_offset,
                self.user_agent.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                accept_header,
                color_depth,
                ip_address,
                java_enabled,
                javascript_enabled,
                language,
                screen_height,
                screen_width,
                timezone_offset,
                user_agent,
            })
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

    impl ObjectDeser for ThreeDSecureStandaloneResourceBrowserDetails {
        type Builder = ThreeDSecureStandaloneResourceBrowserDetailsBuilder;
    }

    impl FromValueOpt for ThreeDSecureStandaloneResourceBrowserDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ThreeDSecureStandaloneResourceBrowserDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "accept_header" => b.accept_header = FromValueOpt::from_value(v),
                    "color_depth" => b.color_depth = FromValueOpt::from_value(v),
                    "ip_address" => b.ip_address = FromValueOpt::from_value(v),
                    "java_enabled" => b.java_enabled = FromValueOpt::from_value(v),
                    "javascript_enabled" => b.javascript_enabled = FromValueOpt::from_value(v),
                    "language" => b.language = FromValueOpt::from_value(v),
                    "screen_height" => b.screen_height = FromValueOpt::from_value(v),
                    "screen_width" => b.screen_width = FromValueOpt::from_value(v),
                    "timezone_offset" => b.timezone_offset = FromValueOpt::from_value(v),
                    "user_agent" => b.user_agent = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
