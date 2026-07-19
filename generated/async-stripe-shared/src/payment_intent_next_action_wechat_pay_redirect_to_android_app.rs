#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentIntentNextActionWechatPayRedirectToAndroidApp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentIntentNextActionWechatPayRedirectToAndroidApp")
            .finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: PaymentIntentNextActionWechatPayRedirectToAndroidAppBuilder {
                    app_id: Deserialize::default(),
                    nonce_str: Deserialize::default(),
                    package: Deserialize::default(),
                    partner_id: Deserialize::default(),
                    prepay_id: Deserialize::default(),
                    sign: Deserialize::default(),
                    timestamp: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "app_id" => Deserialize::begin(&mut self.builder.app_id),
                "nonce_str" => Deserialize::begin(&mut self.builder.nonce_str),
                "package" => Deserialize::begin(&mut self.builder.package),
                "partner_id" => Deserialize::begin(&mut self.builder.partner_id),
                "prepay_id" => Deserialize::begin(&mut self.builder.prepay_id),
                "sign" => Deserialize::begin(&mut self.builder.sign),
                "timestamp" => Deserialize::begin(&mut self.builder.timestamp),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(app_id),
                Some(nonce_str),
                Some(package),
                Some(partner_id),
                Some(prepay_id),
                Some(sign),
                Some(timestamp),
            ) = (
                self.builder.app_id.take(),
                self.builder.nonce_str.take(),
                self.builder.package.take(),
                self.builder.partner_id.take(),
                self.builder.prepay_id.take(),
                self.builder.sign.take(),
                self.builder.timestamp.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(PaymentIntentNextActionWechatPayRedirectToAndroidApp {
                app_id,
                nonce_str,
                package,
                partner_id,
                prepay_id,
                sign,
                timestamp,
            });
            Ok(())
        }
    }
};
