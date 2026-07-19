#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentLinksResourceCompletionBehaviorRedirect {
    /// The URL the customer will be redirected to after the purchase is complete.
    pub url: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentLinksResourceCompletionBehaviorRedirect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentLinksResourceCompletionBehaviorRedirect").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentLinksResourceCompletionBehaviorRedirectBuilder {
    url: Option<String>,
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
                builder: PaymentLinksResourceCompletionBehaviorRedirectBuilder {
                    url: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "url" => Deserialize::begin(&mut self.builder.url),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(url),) = (self.builder.url.take(),) else {
                return Ok(());
            };
            *self.out = Some(PaymentLinksResourceCompletionBehaviorRedirect { url });
            Ok(())
        }
    }
};
