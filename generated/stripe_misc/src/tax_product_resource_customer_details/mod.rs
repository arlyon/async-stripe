#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TaxProductResourceCustomerDetails {
    /// The customer's postal address (for example, home or business location).
    pub address: Option<stripe_misc::TaxProductResourcePostalAddress>,
    /// The type of customer address provided.
    pub address_source: Option<TaxProductResourceCustomerDetailsAddressSource>,
    /// The customer's IP address (IPv4 or IPv6).
    pub ip_address: Option<String>,
    /// The customer's tax IDs (for example, EU VAT numbers).
    pub tax_ids: Vec<stripe_misc::TaxProductResourceCustomerDetailsResourceTaxId>,
    /// The taxability override used for taxation.
    pub taxability_override: TaxProductResourceCustomerDetailsTaxabilityOverride,
}
/// The type of customer address provided.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxProductResourceCustomerDetailsAddressSource {
    Billing,
    Shipping,
}

impl TaxProductResourceCustomerDetailsAddressSource {
    pub fn as_str(self) -> &'static str {
        use TaxProductResourceCustomerDetailsAddressSource::*;
        match self {
            Billing => "billing",
            Shipping => "shipping",
        }
    }
}

impl std::str::FromStr for TaxProductResourceCustomerDetailsAddressSource {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductResourceCustomerDetailsAddressSource::*;
        match s {
            "billing" => Ok(Billing),
            "shipping" => Ok(Shipping),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for TaxProductResourceCustomerDetailsAddressSource {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxProductResourceCustomerDetailsAddressSource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxProductResourceCustomerDetailsAddressSource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TaxProductResourceCustomerDetailsAddressSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TaxProductResourceCustomerDetailsAddressSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for TaxProductResourceCustomerDetailsAddressSource"))
    }
}
/// The taxability override used for taxation.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxProductResourceCustomerDetailsTaxabilityOverride {
    CustomerExempt,
    None,
    ReverseCharge,
}

impl TaxProductResourceCustomerDetailsTaxabilityOverride {
    pub fn as_str(self) -> &'static str {
        use TaxProductResourceCustomerDetailsTaxabilityOverride::*;
        match self {
            CustomerExempt => "customer_exempt",
            None => "none",
            ReverseCharge => "reverse_charge",
        }
    }
}

impl std::str::FromStr for TaxProductResourceCustomerDetailsTaxabilityOverride {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductResourceCustomerDetailsTaxabilityOverride::*;
        match s {
            "customer_exempt" => Ok(CustomerExempt),
            "none" => Ok(None),
            "reverse_charge" => Ok(ReverseCharge),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for TaxProductResourceCustomerDetailsTaxabilityOverride {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxProductResourceCustomerDetailsTaxabilityOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxProductResourceCustomerDetailsTaxabilityOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TaxProductResourceCustomerDetailsTaxabilityOverride {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TaxProductResourceCustomerDetailsTaxabilityOverride {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for TaxProductResourceCustomerDetailsTaxabilityOverride"))
    }
}
