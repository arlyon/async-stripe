/// Configuration for the components supported by this Customer Session.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CustomerSessionResourceComponents {
    pub active_entitlements:
        stripe_core::CustomerSessionResourceComponentsResourceActiveEntitlements,
    pub buy_button: stripe_core::CustomerSessionResourceComponentsResourceBuyButton,
    pub customer_portal: stripe_core::CustomerSessionResourceComponentsResourceCustomerPortal,
    pub customer_sheet: stripe_core::CustomerSessionResourceComponentsResourceCustomerSheet,
    pub mobile_payment_element:
        stripe_core::CustomerSessionResourceComponentsResourceMobilePaymentElement,
    pub payment_element: stripe_core::CustomerSessionResourceComponentsResourcePaymentElement,
    pub pricing_table: stripe_core::CustomerSessionResourceComponentsResourcePricingTable,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CustomerSessionResourceComponents {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CustomerSessionResourceComponents").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct CustomerSessionResourceComponentsBuilder {
    active_entitlements:
        Option<stripe_core::CustomerSessionResourceComponentsResourceActiveEntitlements>,
    buy_button: Option<stripe_core::CustomerSessionResourceComponentsResourceBuyButton>,
    customer_portal: Option<stripe_core::CustomerSessionResourceComponentsResourceCustomerPortal>,
    customer_sheet: Option<stripe_core::CustomerSessionResourceComponentsResourceCustomerSheet>,
    mobile_payment_element:
        Option<stripe_core::CustomerSessionResourceComponentsResourceMobilePaymentElement>,
    payment_element: Option<stripe_core::CustomerSessionResourceComponentsResourcePaymentElement>,
    pricing_table: Option<stripe_core::CustomerSessionResourceComponentsResourcePricingTable>,
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

    impl Deserialize for CustomerSessionResourceComponents {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CustomerSessionResourceComponents>,
        builder: CustomerSessionResourceComponentsBuilder,
    }

    impl Visitor for Place<CustomerSessionResourceComponents> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CustomerSessionResourceComponentsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for CustomerSessionResourceComponentsBuilder {
        type Out = CustomerSessionResourceComponents;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "active_entitlements" => Deserialize::begin(&mut self.active_entitlements),
                "buy_button" => Deserialize::begin(&mut self.buy_button),
                "customer_portal" => Deserialize::begin(&mut self.customer_portal),
                "customer_sheet" => Deserialize::begin(&mut self.customer_sheet),
                "mobile_payment_element" => Deserialize::begin(&mut self.mobile_payment_element),
                "payment_element" => Deserialize::begin(&mut self.payment_element),
                "pricing_table" => Deserialize::begin(&mut self.pricing_table),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                active_entitlements: None,
                buy_button: None,
                customer_portal: None,
                customer_sheet: None,
                mobile_payment_element: None,
                payment_element: None,
                pricing_table: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(active_entitlements),
                Some(buy_button),
                Some(customer_portal),
                Some(customer_sheet),
                Some(mobile_payment_element),
                Some(payment_element),
                Some(pricing_table),
            ) = (
                self.active_entitlements,
                self.buy_button,
                self.customer_portal,
                self.customer_sheet.take(),
                self.mobile_payment_element.take(),
                self.payment_element.take(),
                self.pricing_table,
            )
            else {
                return None;
            };
            Some(Self::Out {
                active_entitlements,
                buy_button,
                customer_portal,
                customer_sheet,
                mobile_payment_element,
                payment_element,
                pricing_table,
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

    impl ObjectDeser for CustomerSessionResourceComponents {
        type Builder = CustomerSessionResourceComponentsBuilder;
    }

    impl FromValueOpt for CustomerSessionResourceComponents {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = CustomerSessionResourceComponentsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "active_entitlements" => b.active_entitlements = FromValueOpt::from_value(v),
                    "buy_button" => b.buy_button = FromValueOpt::from_value(v),
                    "customer_portal" => b.customer_portal = FromValueOpt::from_value(v),
                    "customer_sheet" => b.customer_sheet = FromValueOpt::from_value(v),
                    "mobile_payment_element" => {
                        b.mobile_payment_element = FromValueOpt::from_value(v)
                    }
                    "payment_element" => b.payment_element = FromValueOpt::from_value(v),
                    "pricing_table" => b.pricing_table = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
