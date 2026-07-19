#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentPagesCheckoutSessionCustomerDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentPagesCheckoutSessionCustomerDetails").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: PaymentPagesCheckoutSessionCustomerDetailsBuilder {
                    address: Deserialize::default(),
                    business_name: Deserialize::default(),
                    email: Deserialize::default(),
                    individual_name: Deserialize::default(),
                    name: Deserialize::default(),
                    phone: Deserialize::default(),
                    tax_exempt: Deserialize::default(),
                    tax_ids: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "address" => Deserialize::begin(&mut self.builder.address),
                "business_name" => Deserialize::begin(&mut self.builder.business_name),
                "email" => Deserialize::begin(&mut self.builder.email),
                "individual_name" => Deserialize::begin(&mut self.builder.individual_name),
                "name" => Deserialize::begin(&mut self.builder.name),
                "phone" => Deserialize::begin(&mut self.builder.phone),
                "tax_exempt" => Deserialize::begin(&mut self.builder.tax_exempt),
                "tax_ids" => Deserialize::begin(&mut self.builder.tax_ids),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
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
                self.builder.address.take(),
                self.builder.business_name.take(),
                self.builder.email.take(),
                self.builder.individual_name.take(),
                self.builder.name.take(),
                self.builder.phone.take(),
                self.builder.tax_exempt.take(),
                self.builder.tax_ids.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(PaymentPagesCheckoutSessionCustomerDetails {
                address,
                business_name,
                email,
                individual_name,
                name,
                phone,
                tax_exempt,
                tax_ids,
            });
            Ok(())
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

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PaymentPagesCheckoutSessionCustomerDetailsTaxExempt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentPagesCheckoutSessionCustomerDetailsTaxExempt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PaymentPagesCheckoutSessionCustomerDetailsTaxExempt))
            .finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for PaymentPagesCheckoutSessionCustomerDetailsTaxExempt {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<PaymentPagesCheckoutSessionCustomerDetailsTaxExempt> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentPagesCheckoutSessionCustomerDetailsTaxExempt::from_str(s).expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentPagesCheckoutSessionCustomerDetailsTaxExempt {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
