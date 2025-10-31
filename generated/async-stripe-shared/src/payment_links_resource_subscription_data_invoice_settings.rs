#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentLinksResourceSubscriptionDataInvoiceSettings {
    pub issuer: stripe_shared::ConnectAccountReference,
}
#[doc(hidden)]
pub struct PaymentLinksResourceSubscriptionDataInvoiceSettingsBuilder {
    issuer: Option<stripe_shared::ConnectAccountReference>,
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
                builder: PaymentLinksResourceSubscriptionDataInvoiceSettingsBuilder::deser_default(
                ),
            }))
        }
    }

    impl MapBuilder for PaymentLinksResourceSubscriptionDataInvoiceSettingsBuilder {
        type Out = PaymentLinksResourceSubscriptionDataInvoiceSettings;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "issuer" => Deserialize::begin(&mut self.issuer),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { issuer: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(issuer),) = (self.issuer.take(),) else {
                return None;
            };
            Some(Self::Out { issuer })
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

    impl ObjectDeser for PaymentLinksResourceSubscriptionDataInvoiceSettings {
        type Builder = PaymentLinksResourceSubscriptionDataInvoiceSettingsBuilder;
    }

    impl FromValueOpt for PaymentLinksResourceSubscriptionDataInvoiceSettings {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentLinksResourceSubscriptionDataInvoiceSettingsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "issuer" => b.issuer = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
