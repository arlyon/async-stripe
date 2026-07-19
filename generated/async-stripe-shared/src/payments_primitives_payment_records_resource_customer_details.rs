/// Information about the customer for this payment.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentsPrimitivesPaymentRecordsResourceCustomerDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentsPrimitivesPaymentRecordsResourceCustomerDetails")
            .finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: PaymentsPrimitivesPaymentRecordsResourceCustomerDetailsBuilder {
                    customer: Deserialize::default(),
                    email: Deserialize::default(),
                    name: Deserialize::default(),
                    phone: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "customer" => Deserialize::begin(&mut self.builder.customer),
                "email" => Deserialize::begin(&mut self.builder.email),
                "name" => Deserialize::begin(&mut self.builder.name),
                "phone" => Deserialize::begin(&mut self.builder.phone),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(customer), Some(email), Some(name), Some(phone)) = (
                self.builder.customer.take(),
                self.builder.email.take(),
                self.builder.name.take(),
                self.builder.phone.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(PaymentsPrimitivesPaymentRecordsResourceCustomerDetails {
                customer,
                email,
                name,
                phone,
            });
            Ok(())
        }
    }
};
