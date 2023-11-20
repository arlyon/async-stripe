#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TaxProductRegistrationsResourceCountryOptionsUnitedStates {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_amusement_tax:
        Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsLocalAmusementTax>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_lease_tax:
        Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsLocalLeaseTax>,
    /// Two-letter US state code ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2)).
    pub state: String,
    /// Type of registration in the US.
    #[serde(rename = "type")]
    pub type_: TaxProductRegistrationsResourceCountryOptionsUnitedStatesType,
}
/// Type of registration in the US.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxProductRegistrationsResourceCountryOptionsUnitedStatesType {
    LocalAmusementTax,
    LocalLeaseTax,
    StateCommunicationsTax,
    StateSalesTax,
}

impl TaxProductRegistrationsResourceCountryOptionsUnitedStatesType {
    pub fn as_str(self) -> &'static str {
        use TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::*;
        match self {
            LocalAmusementTax => "local_amusement_tax",
            LocalLeaseTax => "local_lease_tax",
            StateCommunicationsTax => "state_communications_tax",
            StateSalesTax => "state_sales_tax",
        }
    }
}

impl std::str::FromStr for TaxProductRegistrationsResourceCountryOptionsUnitedStatesType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::*;
        match s {
            "local_amusement_tax" => Ok(LocalAmusementTax),
            "local_lease_tax" => Ok(LocalLeaseTax),
            "state_communications_tax" => Ok(StateCommunicationsTax),
            "state_sales_tax" => Ok(StateSalesTax),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for TaxProductRegistrationsResourceCountryOptionsUnitedStatesType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxProductRegistrationsResourceCountryOptionsUnitedStatesType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxProductRegistrationsResourceCountryOptionsUnitedStatesType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TaxProductRegistrationsResourceCountryOptionsUnitedStatesType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de>
    for TaxProductRegistrationsResourceCountryOptionsUnitedStatesType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for TaxProductRegistrationsResourceCountryOptionsUnitedStatesType",
            )
        })
    }
}
