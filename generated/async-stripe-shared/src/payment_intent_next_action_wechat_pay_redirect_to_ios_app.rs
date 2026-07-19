#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentIntentNextActionWechatPayRedirectToIosApp {
    /// An universal link that redirect to WeChat Pay app
    pub native_url: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentIntentNextActionWechatPayRedirectToIosApp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentIntentNextActionWechatPayRedirectToIosApp").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentIntentNextActionWechatPayRedirectToIosAppBuilder {
    native_url: Option<String>,
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

    impl Deserialize for PaymentIntentNextActionWechatPayRedirectToIosApp {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentNextActionWechatPayRedirectToIosApp>,
        builder: PaymentIntentNextActionWechatPayRedirectToIosAppBuilder,
    }

    impl Visitor for Place<PaymentIntentNextActionWechatPayRedirectToIosApp> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentIntentNextActionWechatPayRedirectToIosAppBuilder {
                    native_url: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "native_url" => Deserialize::begin(&mut self.builder.native_url),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(native_url),) = (self.builder.native_url.take(),) else {
                return Ok(());
            };
            *self.out = Some(PaymentIntentNextActionWechatPayRedirectToIosApp { native_url });
            Ok(())
        }
    }
};
