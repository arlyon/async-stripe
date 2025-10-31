#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PortalFeatures {
    pub customer_update: stripe_billing::PortalCustomerUpdate,
    pub invoice_history: stripe_billing::PortalInvoiceList,
    pub payment_method_update: stripe_billing::PortalPaymentMethodUpdate,
    pub subscription_cancel: stripe_billing::PortalSubscriptionCancel,
    pub subscription_update: stripe_billing::PortalSubscriptionUpdate,
}
#[doc(hidden)]
pub struct PortalFeaturesBuilder {
    customer_update: Option<stripe_billing::PortalCustomerUpdate>,
    invoice_history: Option<stripe_billing::PortalInvoiceList>,
    payment_method_update: Option<stripe_billing::PortalPaymentMethodUpdate>,
    subscription_cancel: Option<stripe_billing::PortalSubscriptionCancel>,
    subscription_update: Option<stripe_billing::PortalSubscriptionUpdate>,
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

    impl Deserialize for PortalFeatures {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PortalFeatures>,
        builder: PortalFeaturesBuilder,
    }

    impl Visitor for Place<PortalFeatures> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PortalFeaturesBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PortalFeaturesBuilder {
        type Out = PortalFeatures;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "customer_update" => Deserialize::begin(&mut self.customer_update),
                "invoice_history" => Deserialize::begin(&mut self.invoice_history),
                "payment_method_update" => Deserialize::begin(&mut self.payment_method_update),
                "subscription_cancel" => Deserialize::begin(&mut self.subscription_cancel),
                "subscription_update" => Deserialize::begin(&mut self.subscription_update),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                customer_update: Deserialize::default(),
                invoice_history: Deserialize::default(),
                payment_method_update: Deserialize::default(),
                subscription_cancel: Deserialize::default(),
                subscription_update: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(customer_update),
                Some(invoice_history),
                Some(payment_method_update),
                Some(subscription_cancel),
                Some(subscription_update),
            ) = (
                self.customer_update.take(),
                self.invoice_history,
                self.payment_method_update,
                self.subscription_cancel.take(),
                self.subscription_update.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                customer_update,
                invoice_history,
                payment_method_update,
                subscription_cancel,
                subscription_update,
            })
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

    impl ObjectDeser for PortalFeatures {
        type Builder = PortalFeaturesBuilder;
    }

    impl FromValueOpt for PortalFeatures {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PortalFeaturesBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "customer_update" => b.customer_update = FromValueOpt::from_value(v),
                    "invoice_history" => b.invoice_history = FromValueOpt::from_value(v),
                    "payment_method_update" => {
                        b.payment_method_update = FromValueOpt::from_value(v)
                    }
                    "subscription_cancel" => b.subscription_cancel = FromValueOpt::from_value(v),
                    "subscription_update" => b.subscription_update = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
