#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Location {
    /// The customer's country as identified by Stripe Tax.
    pub country: String,
    /// The data source used to infer the customer's location.
    pub source: LocationSource,
    /// The customer's state, county, province, or region as identified by Stripe Tax.
    pub state: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Location {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The data source used to infer the customer's location.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
