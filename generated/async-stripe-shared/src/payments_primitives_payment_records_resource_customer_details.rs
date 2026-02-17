/// Information about the customer for this payment.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentsPrimitivesPaymentRecordsResourceCustomerDetails {
    /// ID of the Stripe Customer associated with this payment.
    pub customer: Option<String>,
    /// The customer's email address.
    pub email: Option<String>,
    /// The customer's name.
    pub name: Option<String>,
    /// The customer's phone number.
    pub phone: Option<String>,
}
#[doc(hidden)]
pub struct PaymentsPrimitivesPaymentRecordsResourceCustomerDetailsBuilder {
    customer: Option<Option<String>>,
    email: Option<Option<String>>,
    name: Option<Option<String>>,
    phone: Option<Option<String>>,
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

    impl Deserialize for PaymentsPrimitivesPaymentRecordsResourceCustomerDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentsPrimitivesPaymentRecordsResourceCustomerDetails>,
        builder: PaymentsPrimitivesPaymentRecordsResourceCustomerDetailsBuilder,
    }

    impl Visitor for Place<PaymentsPrimitivesPaymentRecordsResourceCustomerDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder:
                    PaymentsPrimitivesPaymentRecordsResourceCustomerDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentsPrimitivesPaymentRecordsResourceCustomerDetailsBuilder {
        type Out = PaymentsPrimitivesPaymentRecordsResourceCustomerDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "customer" => Deserialize::begin(&mut self.customer),
                "email" => Deserialize::begin(&mut self.email),
                "name" => Deserialize::begin(&mut self.name),
                "phone" => Deserialize::begin(&mut self.phone),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                customer: Deserialize::default(),
                email: Deserialize::default(),
                name: Deserialize::default(),
                phone: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(customer), Some(email), Some(name), Some(phone)) =
                (self.customer.take(), self.email.take(), self.name.take(), self.phone.take())
            else {
                return None;
            };
            Some(Self::Out { customer, email, name, phone })
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

    impl ObjectDeser for PaymentsPrimitivesPaymentRecordsResourceCustomerDetails {
        type Builder = PaymentsPrimitivesPaymentRecordsResourceCustomerDetailsBuilder;
    }

    impl FromValueOpt for PaymentsPrimitivesPaymentRecordsResourceCustomerDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                PaymentsPrimitivesPaymentRecordsResourceCustomerDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "customer" => b.customer = FromValueOpt::from_value(v),
                    "email" => b.email = FromValueOpt::from_value(v),
                    "name" => b.name = FromValueOpt::from_value(v),
                    "phone" => b.phone = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
