#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PortalBusinessProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PortalBusinessProfile").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: PortalBusinessProfileBuilder {
                    headline: Deserialize::default(),
                    privacy_policy_url: Deserialize::default(),
                    terms_of_service_url: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "headline" => Deserialize::begin(&mut self.builder.headline),
                "privacy_policy_url" => Deserialize::begin(&mut self.builder.privacy_policy_url),
                "terms_of_service_url" => {
                    Deserialize::begin(&mut self.builder.terms_of_service_url)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(headline), Some(privacy_policy_url), Some(terms_of_service_url)) = (
                self.builder.headline.take(),
                self.builder.privacy_policy_url.take(),
                self.builder.terms_of_service_url.take(),
            ) else {
                return Ok(());
            };
            *self.out =
                Some(PortalBusinessProfile { headline, privacy_policy_url, terms_of_service_url });
            Ok(())
        }
    }
};
