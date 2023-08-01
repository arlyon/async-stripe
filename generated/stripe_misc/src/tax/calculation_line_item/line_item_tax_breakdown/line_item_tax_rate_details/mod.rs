#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LineItemTaxRateDetails {
    /// A localized display name for tax type, intended to be human-readable.
    ///
    /// For example, "Local Sales and Use Tax", "Value-added tax (VAT)", or "Umsatzsteuer (USt.)".
    pub display_name: String,
    /// The tax rate percentage as a string.
    ///
    /// For example, 8.5% is represented as "8.5".
    pub percentage_decimal: String,
    /// The tax type, such as `vat` or `sales_tax`.
    pub tax_type: LineItemTaxRateDetailsTaxType,
}
/// The tax type, such as `vat` or `sales_tax`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum LineItemTaxRateDetailsTaxType {
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

impl LineItemTaxRateDetailsTaxType {
    pub fn as_str(self) -> &'static str {
        use LineItemTaxRateDetailsTaxType::*;
        match self {
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

impl std::str::FromStr for LineItemTaxRateDetailsTaxType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use LineItemTaxRateDetailsTaxType::*;
        match s {
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

impl AsRef<str> for LineItemTaxRateDetailsTaxType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for LineItemTaxRateDetailsTaxType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for LineItemTaxRateDetailsTaxType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LineItemTaxRateDetailsTaxType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom("Unknown value for LineItemTaxRateDetailsTaxType")
        })
    }
}
