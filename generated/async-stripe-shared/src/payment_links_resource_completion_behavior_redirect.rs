#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentLinksResourceCompletionBehaviorRedirect {
    /// The URL the customer will be redirected to after the purchase is complete.
    pub url: String,
}
#[doc(hidden)]
pub struct PaymentLinksResourceCompletionBehaviorRedirectBuilder {
    url: Option<String>,
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

    impl Deserialize for PaymentLinksResourceCompletionBehaviorRedirect {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentLinksResourceCompletionBehaviorRedirect>,
        builder: PaymentLinksResourceCompletionBehaviorRedirectBuilder,
    }

    impl Visitor for Place<PaymentLinksResourceCompletionBehaviorRedirect> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentLinksResourceCompletionBehaviorRedirectBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentLinksResourceCompletionBehaviorRedirectBuilder {
        type Out = PaymentLinksResourceCompletionBehaviorRedirect;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "url" => Deserialize::begin(&mut self.url),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { url: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(url),) = (self.url.take(),) else {
                return None;
            };
            Some(Self::Out { url })
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

    impl ObjectDeser for PaymentLinksResourceCompletionBehaviorRedirect {
        type Builder = PaymentLinksResourceCompletionBehaviorRedirectBuilder;
    }

    impl FromValueOpt for PaymentLinksResourceCompletionBehaviorRedirect {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentLinksResourceCompletionBehaviorRedirectBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "url" => b.url = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
