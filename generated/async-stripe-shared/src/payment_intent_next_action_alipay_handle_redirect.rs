#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentIntentNextActionAlipayHandleRedirect {
    /// The native data to be used with Alipay SDK you must redirect your customer to in order to authenticate the payment in an Android App.
    pub native_data: Option<String>,
    /// The native URL you must redirect your customer to in order to authenticate the payment in an iOS App.
    pub native_url: Option<String>,
    /// If the customer does not exit their browser while authenticating, they will be redirected to this specified URL after completion.
    pub return_url: Option<String>,
    /// The URL you must redirect your customer to in order to authenticate the payment.
    pub url: Option<String>,
}
#[doc(hidden)]
pub struct PaymentIntentNextActionAlipayHandleRedirectBuilder {
    native_data: Option<Option<String>>,
    native_url: Option<Option<String>>,
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

    impl Deserialize for PaymentIntentNextActionAlipayHandleRedirect {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentNextActionAlipayHandleRedirect>,
        builder: PaymentIntentNextActionAlipayHandleRedirectBuilder,
    }

    impl Visitor for Place<PaymentIntentNextActionAlipayHandleRedirect> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentIntentNextActionAlipayHandleRedirectBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentIntentNextActionAlipayHandleRedirectBuilder {
        type Out = PaymentIntentNextActionAlipayHandleRedirect;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "native_data" => Deserialize::begin(&mut self.native_data),
                "native_url" => Deserialize::begin(&mut self.native_url),
                "return_url" => Deserialize::begin(&mut self.return_url),
                "url" => Deserialize::begin(&mut self.url),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                native_data: Deserialize::default(),
                native_url: Deserialize::default(),
                return_url: Deserialize::default(),
                url: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(native_data), Some(native_url), Some(return_url), Some(url)) = (
                self.native_data.take(),
                self.native_url.take(),
                self.return_url.take(),
                self.url.take(),
            ) else {
                return None;
            };
            Some(Self::Out { native_data, native_url, return_url, url })
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

    impl ObjectDeser for PaymentIntentNextActionAlipayHandleRedirect {
        type Builder = PaymentIntentNextActionAlipayHandleRedirectBuilder;
    }

    impl FromValueOpt for PaymentIntentNextActionAlipayHandleRedirect {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentIntentNextActionAlipayHandleRedirectBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "native_data" => b.native_data = FromValueOpt::from_value(v),
                    "native_url" => b.native_url = FromValueOpt::from_value(v),
                    "return_url" => b.return_url = FromValueOpt::from_value(v),
                    "url" => b.url = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
