/// Customer details attached to this payment evaluation.
#[derive(Clone, Debug, Eq, PartialEq)]
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
                builder: InsightsResourcesPaymentEvaluationCustomerDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InsightsResourcesPaymentEvaluationCustomerDetailsBuilder {
        type Out = InsightsResourcesPaymentEvaluationCustomerDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "customer" => Deserialize::begin(&mut self.customer),
                "customer_account" => Deserialize::begin(&mut self.customer_account),
                "email" => Deserialize::begin(&mut self.email),
                "name" => Deserialize::begin(&mut self.name),
                "phone" => Deserialize::begin(&mut self.phone),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                customer: Deserialize::default(),
                customer_account: Deserialize::default(),
                email: Deserialize::default(),
                name: Deserialize::default(),
                phone: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(customer), Some(customer_account), Some(email), Some(name), Some(phone)) = (
                self.customer.take(),
                self.customer_account.take(),
                self.email.take(),
                self.name.take(),
                self.phone.take(),
            ) else {
                return None;
            };
            Some(Self::Out { customer, customer_account, email, name, phone })
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

    impl ObjectDeser for InsightsResourcesPaymentEvaluationCustomerDetails {
        type Builder = InsightsResourcesPaymentEvaluationCustomerDetailsBuilder;
    }

    impl FromValueOpt for InsightsResourcesPaymentEvaluationCustomerDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InsightsResourcesPaymentEvaluationCustomerDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "customer" => b.customer = FromValueOpt::from_value(v),
                    "customer_account" => b.customer_account = FromValueOpt::from_value(v),
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
