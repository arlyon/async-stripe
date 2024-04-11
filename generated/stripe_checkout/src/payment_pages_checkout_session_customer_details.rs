#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionCustomerDetails {
    /// The customer's address after a completed Checkout Session.
    /// Note: This property is populated only for sessions on or after March 30, 2022.
    pub address: Option<stripe_shared::Address>,
    /// The email associated with the Customer, if one exists, on the Checkout Session after a completed Checkout Session or at time of session expiry.
    /// Otherwise, if the customer has consented to promotional content, this value is the most recent valid email provided by the customer on the Checkout form.
    pub email: Option<String>,
    /// The customer's name after a completed Checkout Session.
    /// Note: This property is populated only for sessions on or after March 30, 2022.
    pub name: Option<String>,
    /// The customer's phone number after a completed Checkout Session.
    pub phone: Option<String>,
    /// The customer’s tax exempt status after a completed Checkout Session.
    pub tax_exempt: Option<PaymentPagesCheckoutSessionCustomerDetailsTaxExempt>,
    /// The customer’s tax IDs after a completed Checkout Session.
    pub tax_ids: Option<Vec<stripe_checkout::PaymentPagesCheckoutSessionTaxId>>,
}
#[doc(hidden)]
pub struct PaymentPagesCheckoutSessionCustomerDetailsBuilder {
    address: Option<Option<stripe_shared::Address>>,
    email: Option<Option<String>>,
    name: Option<Option<String>>,
    phone: Option<Option<String>>,
    tax_exempt: Option<Option<PaymentPagesCheckoutSessionCustomerDetailsTaxExempt>>,
    tax_ids: Option<Option<Vec<stripe_checkout::PaymentPagesCheckoutSessionTaxId>>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
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
                "email" => Deserialize::begin(&mut self.email),
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
                email: Deserialize::default(),
                name: Deserialize::default(),
                phone: Deserialize::default(),
                tax_exempt: Deserialize::default(),
                tax_ids: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                address: self.address.take()?,
                email: self.email.take()?,
                name: self.name.take()?,
                phone: self.phone.take()?,
                tax_exempt: self.tax_exempt?,
                tax_ids: self.tax_ids.take()?,
            })
        }
    }

    impl<'a> Map for Builder<'a> {
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
                    "address" => b.address = Some(FromValueOpt::from_value(v)?),
                    "email" => b.email = Some(FromValueOpt::from_value(v)?),
                    "name" => b.name = Some(FromValueOpt::from_value(v)?),
                    "phone" => b.phone = Some(FromValueOpt::from_value(v)?),
                    "tax_exempt" => b.tax_exempt = Some(FromValueOpt::from_value(v)?),
                    "tax_ids" => b.tax_ids = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The customer’s tax exempt status after a completed Checkout Session.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentPagesCheckoutSessionCustomerDetailsTaxExempt {
    Exempt,
    None,
    Reverse,
}
impl PaymentPagesCheckoutSessionCustomerDetailsTaxExempt {
    pub fn as_str(self) -> &'static str {
        use PaymentPagesCheckoutSessionCustomerDetailsTaxExempt::*;
        match self {
            Exempt => "exempt",
            None => "none",
            Reverse => "reverse",
        }
    }
}

impl std::str::FromStr for PaymentPagesCheckoutSessionCustomerDetailsTaxExempt {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentPagesCheckoutSessionCustomerDetailsTaxExempt::*;
        match s {
            "exempt" => Ok(Exempt),
            "none" => Ok(None),
            "reverse" => Ok(Reverse),
            _ => Err(stripe_types::StripeParseError),
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
            PaymentPagesCheckoutSessionCustomerDetailsTaxExempt::from_str(s)
                .map_err(|_| miniserde::Error)?,
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
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentPagesCheckoutSessionCustomerDetailsTaxExempt",
            )
        })
    }
}
