#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentIntentNextActionWechatPayRedirectToIosApp {
    /// An universal link that redirect to WeChat Pay app
    pub native_url: String,
}
#[doc(hidden)]
pub struct PaymentIntentNextActionWechatPayRedirectToIosAppBuilder {
    native_url: Option<String>,
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
                builder: PaymentIntentNextActionWechatPayRedirectToIosAppBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentIntentNextActionWechatPayRedirectToIosAppBuilder {
        type Out = PaymentIntentNextActionWechatPayRedirectToIosApp;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "native_url" => Deserialize::begin(&mut self.native_url),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { native_url: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(native_url),) = (self.native_url.take(),) else {
                return None;
            };
            Some(Self::Out { native_url })
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

    impl ObjectDeser for PaymentIntentNextActionWechatPayRedirectToIosApp {
        type Builder = PaymentIntentNextActionWechatPayRedirectToIosAppBuilder;
    }

    impl FromValueOpt for PaymentIntentNextActionWechatPayRedirectToIosApp {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentIntentNextActionWechatPayRedirectToIosAppBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "native_url" => b.native_url = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
