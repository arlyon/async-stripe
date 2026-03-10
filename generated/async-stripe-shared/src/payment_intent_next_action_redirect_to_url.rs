#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentIntentNextActionRedirectToUrl {
    /// If the customer does not exit their browser while authenticating, they will be redirected to this specified URL after completion.
    pub return_url: Option<String>,
    /// The URL you must redirect your customer to in order to authenticate the payment.
    pub url: Option<String>,
}
#[doc(hidden)]
pub struct PaymentIntentNextActionRedirectToUrlBuilder {
    return_url: Option<Option<String>>,
    url: Option<Option<String>>,
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

    impl Deserialize for PaymentIntentNextActionRedirectToUrl {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentNextActionRedirectToUrl>,
        builder: PaymentIntentNextActionRedirectToUrlBuilder,
    }

    impl Visitor for Place<PaymentIntentNextActionRedirectToUrl> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentIntentNextActionRedirectToUrlBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentIntentNextActionRedirectToUrlBuilder {
        type Out = PaymentIntentNextActionRedirectToUrl;
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

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for PaymentIntentNextActionRedirectToUrl {
        type Builder = PaymentIntentNextActionRedirectToUrlBuilder;
    }

    impl FromValueOpt for PaymentIntentNextActionRedirectToUrl {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentIntentNextActionRedirectToUrlBuilder::deser_default();
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
