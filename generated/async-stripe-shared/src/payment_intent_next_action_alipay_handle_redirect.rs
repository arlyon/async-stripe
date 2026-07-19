#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentIntentNextActionAlipayHandleRedirect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentIntentNextActionAlipayHandleRedirect").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: PaymentIntentNextActionAlipayHandleRedirectBuilder {
                    native_data: Deserialize::default(),
                    native_url: Deserialize::default(),
                    return_url: Deserialize::default(),
                    url: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "native_data" => Deserialize::begin(&mut self.builder.native_data),
                "native_url" => Deserialize::begin(&mut self.builder.native_url),
                "return_url" => Deserialize::begin(&mut self.builder.return_url),
                "url" => Deserialize::begin(&mut self.builder.url),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(native_data), Some(native_url), Some(return_url), Some(url)) = (
                self.builder.native_data.take(),
                self.builder.native_url.take(),
                self.builder.return_url.take(),
                self.builder.url.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(PaymentIntentNextActionAlipayHandleRedirect {
                native_data,
                native_url,
                return_url,
                url,
            });
            Ok(())
        }
    }
};
