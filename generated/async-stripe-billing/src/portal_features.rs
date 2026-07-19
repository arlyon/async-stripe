#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PortalFeatures {
    pub customer_update: stripe_billing::PortalCustomerUpdate,
    pub invoice_history: stripe_billing::PortalInvoiceList,
    pub payment_method_update: stripe_billing::PortalPaymentMethodUpdate,
    pub subscription_cancel: stripe_billing::PortalSubscriptionCancel,
    pub subscription_update: stripe_billing::PortalSubscriptionUpdate,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PortalFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PortalFeatures").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: PortalFeaturesBuilder {
                    customer_update: Deserialize::default(),
                    invoice_history: Deserialize::default(),
                    payment_method_update: Deserialize::default(),
                    subscription_cancel: Deserialize::default(),
                    subscription_update: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "customer_update" => Deserialize::begin(&mut self.builder.customer_update),
                "invoice_history" => Deserialize::begin(&mut self.builder.invoice_history),
                "payment_method_update" => {
                    Deserialize::begin(&mut self.builder.payment_method_update)
                }
                "subscription_cancel" => Deserialize::begin(&mut self.builder.subscription_cancel),
                "subscription_update" => Deserialize::begin(&mut self.builder.subscription_update),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(customer_update),
                Some(invoice_history),
                Some(payment_method_update),
                Some(subscription_cancel),
                Some(subscription_update),
            ) = (
                self.builder.customer_update.take(),
                self.builder.invoice_history,
                self.builder.payment_method_update.take(),
                self.builder.subscription_cancel.take(),
                self.builder.subscription_update.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(PortalFeatures {
                customer_update,
                invoice_history,
                payment_method_update,
                subscription_cancel,
                subscription_update,
            });
            Ok(())
        }
    }
};
