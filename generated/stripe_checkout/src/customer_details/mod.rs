#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CustomerDetails {
    /// The customer's address after a completed Checkout Session.
    ///
    /// Note: This property is populated only for sessions on or after March 30, 2022.
    pub address: Option<stripe_types::address::Address>,
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
    pub tax_exempt: Option<CustomerDetailsTaxExempt>,
    /// The customer’s tax IDs after a completed Checkout Session.
    pub tax_ids: Option<Vec<stripe_checkout::tax_id::TaxId>>,
}
/// The customer’s tax exempt status after a completed Checkout Session.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CustomerDetailsTaxExempt {
    Exempt,
    None,
    Reverse,
}

impl CustomerDetailsTaxExempt {
    pub fn as_str(self) -> &'static str {
        use CustomerDetailsTaxExempt::*;
        match self {
            Exempt => "exempt",
            None => "none",
            Reverse => "reverse",
        }
    }
}

impl std::str::FromStr for CustomerDetailsTaxExempt {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CustomerDetailsTaxExempt::*;
        match s {
            "exempt" => Ok(Exempt),
            "none" => Ok(None),
            "reverse" => Ok(Reverse),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CustomerDetailsTaxExempt {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CustomerDetailsTaxExempt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CustomerDetailsTaxExempt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CustomerDetailsTaxExempt {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CustomerDetailsTaxExempt"))
    }
}
