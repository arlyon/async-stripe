#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PortalBusinessProfile {
    /// The messaging shown to customers in the portal.
    pub headline: Option<String>,
    /// A link to the business’s publicly available privacy policy.
    pub privacy_policy_url: Option<String>,
    /// A link to the business’s publicly available terms of service.
    pub terms_of_service_url: Option<String>,
}
#[doc(hidden)]
pub struct PortalBusinessProfileBuilder {
    headline: Option<Option<String>>,
    privacy_policy_url: Option<Option<String>>,
    terms_of_service_url: Option<Option<String>>,
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PortalBusinessProfile {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PortalBusinessProfile>,
        builder: PortalBusinessProfileBuilder,
    }

    impl Visitor for Place<PortalBusinessProfile> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PortalBusinessProfileBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PortalBusinessProfileBuilder {
        type Out = PortalBusinessProfile;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "headline" => Deserialize::begin(&mut self.headline),
                "privacy_policy_url" => Deserialize::begin(&mut self.privacy_policy_url),
                "terms_of_service_url" => Deserialize::begin(&mut self.terms_of_service_url),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                headline: Deserialize::default(),
                privacy_policy_url: Deserialize::default(),
                terms_of_service_url: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(headline), Some(privacy_policy_url), Some(terms_of_service_url)) = (
                self.headline.take(),
                self.privacy_policy_url.take(),
                self.terms_of_service_url.take(),
            ) else {
                return None;
            };
            Some(Self::Out { headline, privacy_policy_url, terms_of_service_url })
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

    impl ObjectDeser for PortalBusinessProfile {
        type Builder = PortalBusinessProfileBuilder;
    }

    impl FromValueOpt for PortalBusinessProfile {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PortalBusinessProfileBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "headline" => b.headline = FromValueOpt::from_value(v),
                    "privacy_policy_url" => b.privacy_policy_url = FromValueOpt::from_value(v),
                    "terms_of_service_url" => b.terms_of_service_url = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
