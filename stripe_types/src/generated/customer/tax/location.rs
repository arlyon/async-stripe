#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Location {
    /// The customer's country as identified by Stripe Tax.
    pub country: String,
    /// The data source used to infer the customer's location.
    pub source: LocationSource,
    /// The customer's state, county, province, or region as identified by Stripe Tax.
    pub state: Option<String>,
}
/// The data source used to infer the customer's location.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum LocationSource {
    BillingAddress,
    IpAddress,
    PaymentMethod,
    ShippingDestination,
}

impl LocationSource {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BillingAddress => "billing_address",
            Self::IpAddress => "ip_address",
            Self::PaymentMethod => "payment_method",
            Self::ShippingDestination => "shipping_destination",
        }
    }
}

impl std::str::FromStr for LocationSource {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "billing_address" => Ok(Self::BillingAddress),
            "ip_address" => Ok(Self::IpAddress),
            "payment_method" => Ok(Self::PaymentMethod),
            "shipping_destination" => Ok(Self::ShippingDestination),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for LocationSource {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for LocationSource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for LocationSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LocationSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for LocationSource"))
    }
}
