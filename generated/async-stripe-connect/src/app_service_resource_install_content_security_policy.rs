#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AppServiceResourceInstallContentSecurityPolicy {
    /// The URLs that the app can make network requests to.
    pub connect_src: Vec<String>,
    /// The URLs that the app can load images from.
    pub image_src: Vec<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AppServiceResourceInstallContentSecurityPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AppServiceResourceInstallContentSecurityPolicy").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct AppServiceResourceInstallContentSecurityPolicyBuilder {
    connect_src: Option<Vec<String>>,
    image_src: Option<Vec<String>>,
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

    impl Deserialize for AppServiceResourceInstallContentSecurityPolicy {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AppServiceResourceInstallContentSecurityPolicy>,
        builder: AppServiceResourceInstallContentSecurityPolicyBuilder,
    }

    impl Visitor for Place<AppServiceResourceInstallContentSecurityPolicy> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: AppServiceResourceInstallContentSecurityPolicyBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for AppServiceResourceInstallContentSecurityPolicyBuilder {
        type Out = AppServiceResourceInstallContentSecurityPolicy;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "connect_src" => Deserialize::begin(&mut self.connect_src),
                "image_src" => Deserialize::begin(&mut self.image_src),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { connect_src: None, image_src: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(connect_src), Some(image_src)) =
                (self.connect_src.take(), self.image_src.take())
            else {
                return None;
            };
            Some(Self::Out { connect_src, image_src })
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

    impl ObjectDeser for AppServiceResourceInstallContentSecurityPolicy {
        type Builder = AppServiceResourceInstallContentSecurityPolicyBuilder;
    }

    impl FromValueOpt for AppServiceResourceInstallContentSecurityPolicy {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = AppServiceResourceInstallContentSecurityPolicyBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "connect_src" => b.connect_src = FromValueOpt::from_value(v),
                    "image_src" => b.image_src = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
