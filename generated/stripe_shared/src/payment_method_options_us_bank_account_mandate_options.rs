#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodOptionsUsBankAccountMandateOptions {
    /// Mandate collection method
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<PaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod>,
}
/// Mandate collection method
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod {
    Paper,
}
impl PaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod::*;
        match self {
            Paper => "paper",
        }
    }
}

impl std::str::FromStr for PaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod::*;
        match s {
            "paper" => Ok(Paper),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for PaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de>
    for PaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod",
            )
        })
    }
}
