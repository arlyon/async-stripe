#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CustomerDetails {
    /// The customer's postal address (for example, home or business location).
    pub address:
        Option<stripe_misc::tax::calculation::customer_details::postal_address::PostalAddress>,
    /// The type of customer address provided.
    pub address_source: Option<CustomerDetailsAddressSource>,
    /// The customer's IP address (IPv4 or IPv6).
    pub ip_address: Option<String>,
    /// The customer's tax IDs (for example, EU VAT numbers).
    pub tax_ids: Vec<stripe_misc::tax::calculation::customer_details::tax_id::TaxId>,
    /// The taxability override used for taxation.
    pub taxability_override: CustomerDetailsTaxabilityOverride,
}
/// The type of customer address provided.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CustomerDetailsAddressSource {
    Billing,
    Shipping,
}

impl CustomerDetailsAddressSource {
    pub fn as_str(self) -> &'static str {
        use CustomerDetailsAddressSource::*;
        match self {
            Billing => "billing",
            Shipping => "shipping",
        }
    }
}

impl std::str::FromStr for CustomerDetailsAddressSource {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CustomerDetailsAddressSource::*;
        match s {
            "billing" => Ok(Billing),
            "shipping" => Ok(Shipping),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CustomerDetailsAddressSource {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CustomerDetailsAddressSource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CustomerDetailsAddressSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CustomerDetailsAddressSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CustomerDetailsAddressSource"))
    }
}
/// The taxability override used for taxation.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CustomerDetailsTaxabilityOverride {
    CustomerExempt,
    None,
    ReverseCharge,
}

impl CustomerDetailsTaxabilityOverride {
    pub fn as_str(self) -> &'static str {
        use CustomerDetailsTaxabilityOverride::*;
        match self {
            CustomerExempt => "customer_exempt",
            None => "none",
            ReverseCharge => "reverse_charge",
        }
    }
}

impl std::str::FromStr for CustomerDetailsTaxabilityOverride {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CustomerDetailsTaxabilityOverride::*;
        match s {
            "customer_exempt" => Ok(CustomerExempt),
            "none" => Ok(None),
            "reverse_charge" => Ok(ReverseCharge),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CustomerDetailsTaxabilityOverride {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CustomerDetailsTaxabilityOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CustomerDetailsTaxabilityOverride {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CustomerDetailsTaxabilityOverride {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CustomerDetailsTaxabilityOverride")
        })
    }
}
pub mod tax_id;
pub use tax_id::TaxId;
pub mod postal_address;
pub use postal_address::PostalAddress;
