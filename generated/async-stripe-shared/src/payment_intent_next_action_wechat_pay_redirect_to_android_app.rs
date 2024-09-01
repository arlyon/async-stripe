#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentIntentNextActionWechatPayRedirectToAndroidApp {
    /// app_id is the APP ID registered on WeChat open platform
    pub app_id: String,
    /// nonce_str is a random string
    pub nonce_str: String,
    /// package is static value
    pub package: String,
    /// an unique merchant ID assigned by WeChat Pay
    pub partner_id: String,
    /// an unique trading ID assigned by WeChat Pay
    pub prepay_id: String,
    /// A signature
    pub sign: String,
    /// Specifies the current time in epoch format
    pub timestamp: String,
}
#[doc(hidden)]
pub struct PaymentIntentNextActionWechatPayRedirectToAndroidAppBuilder {
    app_id: Option<String>,
    nonce_str: Option<String>,
    package: Option<String>,
    partner_id: Option<String>,
    prepay_id: Option<String>,
    sign: Option<String>,
    timestamp: Option<String>,
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

    impl Deserialize for PaymentIntentNextActionWechatPayRedirectToAndroidApp {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentNextActionWechatPayRedirectToAndroidApp>,
        builder: PaymentIntentNextActionWechatPayRedirectToAndroidAppBuilder,
    }

    impl Visitor for Place<PaymentIntentNextActionWechatPayRedirectToAndroidApp> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentIntentNextActionWechatPayRedirectToAndroidAppBuilder::deser_default(
                ),
            }))
        }
    }

    impl MapBuilder for PaymentIntentNextActionWechatPayRedirectToAndroidAppBuilder {
        type Out = PaymentIntentNextActionWechatPayRedirectToAndroidApp;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "app_id" => Deserialize::begin(&mut self.app_id),
                "nonce_str" => Deserialize::begin(&mut self.nonce_str),
                "package" => Deserialize::begin(&mut self.package),
                "partner_id" => Deserialize::begin(&mut self.partner_id),
                "prepay_id" => Deserialize::begin(&mut self.prepay_id),
                "sign" => Deserialize::begin(&mut self.sign),
                "timestamp" => Deserialize::begin(&mut self.timestamp),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                app_id: Deserialize::default(),
                nonce_str: Deserialize::default(),
                package: Deserialize::default(),
                partner_id: Deserialize::default(),
                prepay_id: Deserialize::default(),
                sign: Deserialize::default(),
                timestamp: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(app_id),
                Some(nonce_str),
                Some(package),
                Some(partner_id),
                Some(prepay_id),
                Some(sign),
                Some(timestamp),
            ) = (
                self.app_id.take(),
                self.nonce_str.take(),
                self.package.take(),
                self.partner_id.take(),
                self.prepay_id.take(),
                self.sign.take(),
                self.timestamp.take(),
            )
            else {
                return None;
            };
            Some(Self::Out { app_id, nonce_str, package, partner_id, prepay_id, sign, timestamp })
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

    impl ObjectDeser for PaymentIntentNextActionWechatPayRedirectToAndroidApp {
        type Builder = PaymentIntentNextActionWechatPayRedirectToAndroidAppBuilder;
    }

    impl FromValueOpt for PaymentIntentNextActionWechatPayRedirectToAndroidApp {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                PaymentIntentNextActionWechatPayRedirectToAndroidAppBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "app_id" => b.app_id = FromValueOpt::from_value(v),
                    "nonce_str" => b.nonce_str = FromValueOpt::from_value(v),
                    "package" => b.package = FromValueOpt::from_value(v),
                    "partner_id" => b.partner_id = FromValueOpt::from_value(v),
                    "prepay_id" => b.prepay_id = FromValueOpt::from_value(v),
                    "sign" => b.sign = FromValueOpt::from_value(v),
                    "timestamp" => b.timestamp = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
