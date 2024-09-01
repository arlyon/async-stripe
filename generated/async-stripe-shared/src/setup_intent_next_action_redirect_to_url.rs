#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SetupIntentNextActionRedirectToUrl {
    /// If the customer does not exit their browser while authenticating, they will be redirected to this specified URL after completion.
    pub return_url: Option<String>,
    /// The URL you must redirect your customer to in order to authenticate.
    pub url: Option<String>,
}
#[doc(hidden)]
pub struct SetupIntentNextActionRedirectToUrlBuilder {
    return_url: Option<Option<String>>,
    url: Option<Option<String>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
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

    impl Deserialize for SetupIntentNextActionRedirectToUrl {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SetupIntentNextActionRedirectToUrl>,
        builder: SetupIntentNextActionRedirectToUrlBuilder,
    }

    impl Visitor for Place<SetupIntentNextActionRedirectToUrl> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SetupIntentNextActionRedirectToUrlBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SetupIntentNextActionRedirectToUrlBuilder {
        type Out = SetupIntentNextActionRedirectToUrl;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "return_url" => Deserialize::begin(&mut self.return_url),
                "url" => Deserialize::begin(&mut self.url),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { return_url: Deserialize::default(), url: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(return_url), Some(url)) = (self.return_url.take(), self.url.take()) else {
                return None;
            };
            Some(Self::Out { return_url, url })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for SetupIntentNextActionRedirectToUrl {
        type Builder = SetupIntentNextActionRedirectToUrlBuilder;
    }

    impl FromValueOpt for SetupIntentNextActionRedirectToUrl {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SetupIntentNextActionRedirectToUrlBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "return_url" => b.return_url = FromValueOpt::from_value(v),
                    "url" => b.url = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
