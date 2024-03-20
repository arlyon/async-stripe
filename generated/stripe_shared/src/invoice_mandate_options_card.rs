#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct InvoiceMandateOptionsCard {
    /// Amount to be charged for future payments.
    pub amount: Option<i64>,
    /// One of `fixed` or `maximum`.
    /// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
    /// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
    pub amount_type: Option<InvoiceMandateOptionsCardAmountType>,
    /// A description of the mandate or subscription that is meant to be displayed to the customer.
    pub description: Option<String>,
}
/// One of `fixed` or `maximum`.
/// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
/// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum InvoiceMandateOptionsCardAmountType {
    Fixed,
    Maximum,
}
impl InvoiceMandateOptionsCardAmountType {
    pub fn as_str(self) -> &'static str {
        use InvoiceMandateOptionsCardAmountType::*;
        match self {
            Fixed => "fixed",
            Maximum => "maximum",
        }
    }
}

impl std::str::FromStr for InvoiceMandateOptionsCardAmountType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InvoiceMandateOptionsCardAmountType::*;
        match s {
            "fixed" => Ok(Fixed),
            "maximum" => Ok(Maximum),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for InvoiceMandateOptionsCardAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InvoiceMandateOptionsCardAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for InvoiceMandateOptionsCardAmountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InvoiceMandateOptionsCardAmountType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for InvoiceMandateOptionsCardAmountType")
        })
    }
}
