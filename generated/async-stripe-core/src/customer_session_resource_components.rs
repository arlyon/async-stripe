/// Configuration for the components supported by this Customer Session.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CustomerSessionResourceComponents {
    pub buy_button: stripe_core::CustomerSessionResourceComponentsResourceBuyButton,
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
    buy_button: Option<stripe_core::CustomerSessionResourceComponentsResourceBuyButton>,
    customer_sheet: Option<stripe_core::CustomerSessionResourceComponentsResourceCustomerSheet>,
    mobile_payment_element:
        Option<stripe_core::CustomerSessionResourceComponentsResourceMobilePaymentElement>,
    payment_element: Option<stripe_core::CustomerSessionResourceComponentsResourcePaymentElement>,
    pricing_table: Option<stripe_core::CustomerSessionResourceComponentsResourcePricingTable>,
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
                builder: CustomerSessionResourceComponentsBuilder {
                    buy_button: Deserialize::default(),
                    customer_sheet: Deserialize::default(),
                    mobile_payment_element: Deserialize::default(),
                    payment_element: Deserialize::default(),
                    pricing_table: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "buy_button" => Deserialize::begin(&mut self.builder.buy_button),
                "customer_sheet" => Deserialize::begin(&mut self.builder.customer_sheet),
                "mobile_payment_element" => {
                    Deserialize::begin(&mut self.builder.mobile_payment_element)
                }
                "payment_element" => Deserialize::begin(&mut self.builder.payment_element),
                "pricing_table" => Deserialize::begin(&mut self.builder.pricing_table),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(buy_button),
                Some(customer_sheet),
                Some(mobile_payment_element),
                Some(payment_element),
                Some(pricing_table),
            ) = (
                self.builder.buy_button,
                self.builder.customer_sheet.take(),
                self.builder.mobile_payment_element.take(),
                self.builder.payment_element.take(),
                self.builder.pricing_table,
            )
            else {
                return Ok(());
            };
            *self.out = Some(CustomerSessionResourceComponents {
                buy_button,
                customer_sheet,
                mobile_payment_element,
                payment_element,
                pricing_table,
            });
            Ok(())
        }
    }
};
