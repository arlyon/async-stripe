#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionCustomerDetails {
    /// The customer's address after a completed Checkout Session.
    /// Note: This property is populated only for sessions on or after March 30, 2022.
    pub address: Option<stripe_shared::Address>,
    /// The customer's business name after a completed Checkout Session.
    pub business_name: Option<String>,
    /// The email associated with the Customer, if one exists, on the Checkout Session after a completed Checkout Session or at time of session expiry.
    /// Otherwise, if the customer has consented to promotional content, this value is the most recent valid email provided by the customer on the Checkout form.
    pub email: Option<String>,
    /// The customer's individual name after a completed Checkout Session.
    pub individual_name: Option<String>,
    /// The customer's name after a completed Checkout Session.
    /// Note: This property is populated only for sessions on or after March 30, 2022.
    pub name: Option<String>,
    /// The customer's phone number after a completed Checkout Session.
    pub phone: Option<String>,
    /// The customer’s tax exempt status after a completed Checkout Session.
    pub tax_exempt: Option<PaymentPagesCheckoutSessionCustomerDetailsTaxExempt>,
    /// The customer’s tax IDs after a completed Checkout Session.
    pub tax_ids: Option<Vec<stripe_shared::PaymentPagesCheckoutSessionTaxId>>,
}
#[doc(hidden)]
pub struct PaymentPagesCheckoutSessionCustomerDetailsBuilder {
    address: Option<Option<stripe_shared::Address>>,
    business_name: Option<Option<String>>,
    email: Option<Option<String>>,
    individual_name: Option<Option<String>>,
    name: Option<Option<String>>,
    phone: Option<Option<String>>,
    tax_exempt: Option<Option<PaymentPagesCheckoutSessionCustomerDetailsTaxExempt>>,
    tax_ids: Option<Option<Vec<stripe_shared::PaymentPagesCheckoutSessionTaxId>>>,
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

    impl Deserialize for PaymentPagesCheckoutSessionCustomerDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentPagesCheckoutSessionCustomerDetails>,
        builder: PaymentPagesCheckoutSessionCustomerDetailsBuilder,
    }

    impl Visitor for Place<PaymentPagesCheckoutSessionCustomerDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentPagesCheckoutSessionCustomerDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentPagesCheckoutSessionCustomerDetailsBuilder {
        type Out = PaymentPagesCheckoutSessionCustomerDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "address" => Deserialize::begin(&mut self.address),
                "business_name" => Deserialize::begin(&mut self.business_name),
                "email" => Deserialize::begin(&mut self.email),
                "individual_name" => Deserialize::begin(&mut self.individual_name),
                "name" => Deserialize::begin(&mut self.name),
                "phone" => Deserialize::begin(&mut self.phone),
                "tax_exempt" => Deserialize::begin(&mut self.tax_exempt),
                "tax_ids" => Deserialize::begin(&mut self.tax_ids),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                address: Deserialize::default(),
                business_name: Deserialize::default(),
                email: Deserialize::default(),
                individual_name: Deserialize::default(),
                name: Deserialize::default(),
                phone: Deserialize::default(),
                tax_exempt: Deserialize::default(),
                tax_ids: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(address),
                Some(business_name),
                Some(email),
                Some(individual_name),
                Some(name),
                Some(phone),
                Some(tax_exempt),
                Some(tax_ids),
            ) = (
                self.address.take(),
                self.business_name.take(),
                self.email.take(),
                self.individual_name.take(),
                self.name.take(),
                self.phone.take(),
                self.tax_exempt.take(),
                self.tax_ids.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                address,
                business_name,
                email,
                individual_name,
                name,
                phone,
                tax_exempt,
                tax_ids,
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

    impl ObjectDeser for PaymentPagesCheckoutSessionCustomerDetails {
        type Builder = PaymentPagesCheckoutSessionCustomerDetailsBuilder;
    }

    impl FromValueOpt for PaymentPagesCheckoutSessionCustomerDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentPagesCheckoutSessionCustomerDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "address" => b.address = FromValueOpt::from_value(v),
                    "business_name" => b.business_name = FromValueOpt::from_value(v),
                    "email" => b.email = FromValueOpt::from_value(v),
                    "individual_name" => b.individual_name = FromValueOpt::from_value(v),
                    "name" => b.name = FromValueOpt::from_value(v),
                    "phone" => b.phone = FromValueOpt::from_value(v),
                    "tax_exempt" => b.tax_exempt = FromValueOpt::from_value(v),
                    "tax_ids" => b.tax_ids = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The customer’s tax exempt status after a completed Checkout Session.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentPagesCheckoutSessionCustomerDetailsTaxExempt {
    Exempt,
    None,
    Reverse,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentPagesCheckoutSessionCustomerDetailsTaxExempt {
    pub fn as_str(&self) -> &str {
        use PaymentPagesCheckoutSessionCustomerDetailsTaxExempt::*;
        match self {
            Exempt => "exempt",
            None => "none",
            Reverse => "reverse",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentPagesCheckoutSessionCustomerDetailsTaxExempt {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentPagesCheckoutSessionCustomerDetailsTaxExempt::*;
        match s {
            "exempt" => Ok(Exempt),
            "none" => Ok(None),
            "reverse" => Ok(Reverse),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentPagesCheckoutSessionCustomerDetailsTaxExempt"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentPagesCheckoutSessionCustomerDetailsTaxExempt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentPagesCheckoutSessionCustomerDetailsTaxExempt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentPagesCheckoutSessionCustomerDetailsTaxExempt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentPagesCheckoutSessionCustomerDetailsTaxExempt {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentPagesCheckoutSessionCustomerDetailsTaxExempt> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentPagesCheckoutSessionCustomerDetailsTaxExempt::from_str(s).expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentPagesCheckoutSessionCustomerDetailsTaxExempt);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentPagesCheckoutSessionCustomerDetailsTaxExempt {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
