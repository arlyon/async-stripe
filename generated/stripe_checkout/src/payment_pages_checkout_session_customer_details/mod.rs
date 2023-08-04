#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentPagesCheckoutSessionCustomerDetails {
    /// The customer's address after a completed Checkout Session.
    ///
    /// Note: This property is populated only for sessions on or after March 30, 2022.
    pub address: Option<stripe_types::Address>,
    /// The email associated with the Customer, if one exists, on the Checkout Session after a completed Checkout Session or at time of session expiry.
    /// Otherwise, if the customer has consented to promotional content, this value is the most recent valid email provided by the customer on the Checkout form.
    pub email: Option<String>,
    /// The customer's name after a completed Checkout Session.
    ///
    /// Note: This property is populated only for sessions on or after March 30, 2022.
    pub name: Option<String>,
    /// The customer's phone number after a completed Checkout Session.
    pub phone: Option<String>,
    /// The customer’s tax exempt status after a completed Checkout Session.
    pub tax_exempt: Option<PaymentPagesCheckoutSessionCustomerDetailsTaxExempt>,
    /// The customer’s tax IDs after a completed Checkout Session.
    pub tax_ids: Option<Vec<stripe_checkout::PaymentPagesCheckoutSessionTaxId>>,
}
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentPagesCheckoutSessionCustomerDetailsTaxExempt::*;
        match s {
            "exempt" => Ok(Exempt),
            "none" => Ok(None),
            "reverse" => Ok(Reverse),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentPagesCheckoutSessionCustomerDetailsTaxExempt {
    fn as_ref(&self) -> &str {
        self.as_str()
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
impl serde::Serialize for PaymentPagesCheckoutSessionCustomerDetailsTaxExempt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentPagesCheckoutSessionCustomerDetailsTaxExempt {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentPagesCheckoutSessionCustomerDetailsTaxExempt",
            )
        })
    }
}
