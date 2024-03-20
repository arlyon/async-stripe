#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TaxProductResourceLineItemTaxRateDetails {
    /// A localized display name for tax type, intended to be human-readable.
    /// For example, "Local Sales and Use Tax", "Value-added tax (VAT)", or "Umsatzsteuer (USt.)".
    pub display_name: String,
    /// The tax rate percentage as a string. For example, 8.5% is represented as "8.5".
    pub percentage_decimal: String,
    /// The tax type, such as `vat` or `sales_tax`.
    pub tax_type: TaxProductResourceLineItemTaxRateDetailsTaxType,
}
/// The tax type, such as `vat` or `sales_tax`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxProductResourceLineItemTaxRateDetailsTaxType {
    AmusementTax,
    CommunicationsTax,
    Gst,
    Hst,
    Igst,
    Jct,
    LeaseTax,
    Pst,
    Qst,
    Rst,
    SalesTax,
    Vat,
}
impl TaxProductResourceLineItemTaxRateDetailsTaxType {
    pub fn as_str(self) -> &'static str {
        use TaxProductResourceLineItemTaxRateDetailsTaxType::*;
        match self {
            AmusementTax => "amusement_tax",
            CommunicationsTax => "communications_tax",
            Gst => "gst",
            Hst => "hst",
            Igst => "igst",
            Jct => "jct",
            LeaseTax => "lease_tax",
            Pst => "pst",
            Qst => "qst",
            Rst => "rst",
            SalesTax => "sales_tax",
            Vat => "vat",
        }
    }
}

impl std::str::FromStr for TaxProductResourceLineItemTaxRateDetailsTaxType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductResourceLineItemTaxRateDetailsTaxType::*;
        match s {
            "amusement_tax" => Ok(AmusementTax),
            "communications_tax" => Ok(CommunicationsTax),
            "gst" => Ok(Gst),
            "hst" => Ok(Hst),
            "igst" => Ok(Igst),
            "jct" => Ok(Jct),
            "lease_tax" => Ok(LeaseTax),
            "pst" => Ok(Pst),
            "qst" => Ok(Qst),
            "rst" => Ok(Rst),
            "sales_tax" => Ok(SalesTax),
            "vat" => Ok(Vat),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for TaxProductResourceLineItemTaxRateDetailsTaxType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxProductResourceLineItemTaxRateDetailsTaxType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TaxProductResourceLineItemTaxRateDetailsTaxType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TaxProductResourceLineItemTaxRateDetailsTaxType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for TaxProductResourceLineItemTaxRateDetailsTaxType",
            )
        })
    }
}
