/// Customer details attached to this payment evaluation.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InsightsResourcesPaymentEvaluationCustomerDetails {
    /// The ID of the customer associated with the payment evaluation.
    pub customer: Option<String>,
    /// The ID of the Account representing the customer associated with the payment evaluation.
    pub customer_account: Option<String>,
    /// The customer's email address.
    pub email: Option<String>,
    /// The customer's full name or business name.
    pub name: Option<String>,
    /// The customer's phone number.
    pub phone: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InsightsResourcesPaymentEvaluationCustomerDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InsightsResourcesPaymentEvaluationCustomerDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct InsightsResourcesPaymentEvaluationCustomerDetailsBuilder {
    customer: Option<Option<String>>,
    customer_account: Option<Option<String>>,
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

    impl Deserialize for InsightsResourcesPaymentEvaluationCustomerDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InsightsResourcesPaymentEvaluationCustomerDetails>,
        builder: InsightsResourcesPaymentEvaluationCustomerDetailsBuilder,
    }

    impl Visitor for Place<InsightsResourcesPaymentEvaluationCustomerDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InsightsResourcesPaymentEvaluationCustomerDetailsBuilder {
                    customer: Deserialize::default(),
                    customer_account: Deserialize::default(),
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
                "customer_account" => Deserialize::begin(&mut self.builder.customer_account),
                "email" => Deserialize::begin(&mut self.builder.email),
                "name" => Deserialize::begin(&mut self.builder.name),
                "phone" => Deserialize::begin(&mut self.builder.phone),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(customer), Some(customer_account), Some(email), Some(name), Some(phone)) = (
                self.builder.customer.take(),
                self.builder.customer_account.take(),
                self.builder.email.take(),
                self.builder.name.take(),
                self.builder.phone.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(InsightsResourcesPaymentEvaluationCustomerDetails {
                customer,
                customer_account,
                email,
                name,
                phone,
            });
            Ok(())
        }
    }
};
