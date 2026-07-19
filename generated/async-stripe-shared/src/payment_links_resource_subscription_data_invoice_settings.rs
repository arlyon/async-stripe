#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentLinksResourceSubscriptionDataInvoiceSettings {
    pub issuer: stripe_shared::ConnectAccountReference,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentLinksResourceSubscriptionDataInvoiceSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentLinksResourceSubscriptionDataInvoiceSettings")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentLinksResourceSubscriptionDataInvoiceSettingsBuilder {
    issuer: Option<stripe_shared::ConnectAccountReference>,
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

    impl Deserialize for PaymentLinksResourceSubscriptionDataInvoiceSettings {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentLinksResourceSubscriptionDataInvoiceSettings>,
        builder: PaymentLinksResourceSubscriptionDataInvoiceSettingsBuilder,
    }

    impl Visitor for Place<PaymentLinksResourceSubscriptionDataInvoiceSettings> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentLinksResourceSubscriptionDataInvoiceSettingsBuilder {
                    issuer: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "issuer" => Deserialize::begin(&mut self.builder.issuer),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(issuer),) = (self.builder.issuer.take(),) else {
                return Ok(());
            };
            *self.out = Some(PaymentLinksResourceSubscriptionDataInvoiceSettings { issuer });
            Ok(())
        }
    }
};
