#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentFlowsPaymentDetails {
    pub car_rental_data:
        Option<Vec<stripe_shared::PaymentFlowsIndustryMetadataCarRentalDataResourceCarRental>>,
    /// A unique value to identify the customer. This field is available only for card payments.
    ///
    /// This field is truncated to 25 alphanumeric characters, excluding spaces, before being sent to card networks.
    pub customer_reference: Option<String>,
    pub lodging_data:
        Option<Vec<stripe_shared::PaymentFlowsIndustryMetadataLodgingDataResourceLodging>>,
    /// A unique value assigned by the business to identify the transaction. Required for L2 and L3 rates.
    ///
    /// For Cards, this field is truncated to 25 alphanumeric characters, excluding spaces, before being sent to card networks.
    /// For Klarna, this field is truncated to 255 characters and is visible to customers when they view the order in the Klarna app.
    pub order_reference: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentFlowsPaymentDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentFlowsPaymentDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentFlowsPaymentDetailsBuilder {
    car_rental_data: Option<
        Option<Vec<stripe_shared::PaymentFlowsIndustryMetadataCarRentalDataResourceCarRental>>,
    >,
    customer_reference: Option<Option<String>>,
    lodging_data:
        Option<Option<Vec<stripe_shared::PaymentFlowsIndustryMetadataLodgingDataResourceLodging>>>,
    order_reference: Option<Option<String>>,
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

    impl Deserialize for PaymentFlowsPaymentDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentFlowsPaymentDetails>,
        builder: PaymentFlowsPaymentDetailsBuilder,
    }

    impl Visitor for Place<PaymentFlowsPaymentDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentFlowsPaymentDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentFlowsPaymentDetailsBuilder {
        type Out = PaymentFlowsPaymentDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "car_rental_data" => Deserialize::begin(&mut self.car_rental_data),
                "customer_reference" => Deserialize::begin(&mut self.customer_reference),
                "lodging_data" => Deserialize::begin(&mut self.lodging_data),
                "order_reference" => Deserialize::begin(&mut self.order_reference),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                car_rental_data: Some(None),
                customer_reference: Some(None),
                lodging_data: Some(None),
                order_reference: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(car_rental_data),
                Some(customer_reference),
                Some(lodging_data),
                Some(order_reference),
            ) = (
                self.car_rental_data.take(),
                self.customer_reference.take(),
                self.lodging_data.take(),
                self.order_reference.take(),
            )
            else {
                return None;
            };
            Some(Self::Out { car_rental_data, customer_reference, lodging_data, order_reference })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for PaymentFlowsPaymentDetails {
        type Builder = PaymentFlowsPaymentDetailsBuilder;
    }

    impl FromValueOpt for PaymentFlowsPaymentDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentFlowsPaymentDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "car_rental_data" => b.car_rental_data = FromValueOpt::from_value(v),
                    "customer_reference" => b.customer_reference = FromValueOpt::from_value(v),
                    "lodging_data" => b.lodging_data = FromValueOpt::from_value(v),
                    "order_reference" => b.order_reference = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
