#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Tax {
    /// Amount of tax applied for this rate.
    pub amount: i64,
    pub rate: stripe_types::tax_rate::TaxRate,
    /// The reasoning behind this tax, for example, if the product is tax exempt.
    ///
    /// The possible values for this field may be extended as new tax rules are supported.
    pub taxability_reason: Option<TaxTaxabilityReason>,
    /// The amount on which tax is calculated, in %s.
    pub taxable_amount: Option<i64>,
}
/// The reasoning behind this tax, for example, if the product is tax exempt.
///
/// The possible values for this field may be extended as new tax rules are supported.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TaxTaxabilityReason {
    CustomerExempt,
    ExcludedTerritory,
    JurisdictionUnsupported,
    NotCollecting,
    NotSubjectToTax,
    NotSupported,
    PortionProductExempt,
    PortionReducedRated,
    PortionStandardRated,
    ProductExempt,
    ProductExemptHoliday,
    ProportionallyRated,
    ReducedRated,
    ReverseCharge,
    StandardRated,
    TaxableBasisReduced,
    VatExempt,
    ZeroRated,
}

impl TaxTaxabilityReason {
    pub fn as_str(self) -> &'static str {
        use TaxTaxabilityReason::*;
        match self {
            CustomerExempt => "customer_exempt",
            ExcludedTerritory => "excluded_territory",
            JurisdictionUnsupported => "jurisdiction_unsupported",
            NotCollecting => "not_collecting",
            NotSubjectToTax => "not_subject_to_tax",
            NotSupported => "not_supported",
            PortionProductExempt => "portion_product_exempt",
            PortionReducedRated => "portion_reduced_rated",
            PortionStandardRated => "portion_standard_rated",
            ProductExempt => "product_exempt",
            ProductExemptHoliday => "product_exempt_holiday",
            ProportionallyRated => "proportionally_rated",
            ReducedRated => "reduced_rated",
            ReverseCharge => "reverse_charge",
            StandardRated => "standard_rated",
            TaxableBasisReduced => "taxable_basis_reduced",
            VatExempt => "vat_exempt",
            ZeroRated => "zero_rated",
        }
    }
}

impl std::str::FromStr for TaxTaxabilityReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxTaxabilityReason::*;
        match s {
            "customer_exempt" => Ok(CustomerExempt),
            "excluded_territory" => Ok(ExcludedTerritory),
            "jurisdiction_unsupported" => Ok(JurisdictionUnsupported),
            "not_collecting" => Ok(NotCollecting),
            "not_subject_to_tax" => Ok(NotSubjectToTax),
            "not_supported" => Ok(NotSupported),
            "portion_product_exempt" => Ok(PortionProductExempt),
            "portion_reduced_rated" => Ok(PortionReducedRated),
            "portion_standard_rated" => Ok(PortionStandardRated),
            "product_exempt" => Ok(ProductExempt),
            "product_exempt_holiday" => Ok(ProductExemptHoliday),
            "proportionally_rated" => Ok(ProportionallyRated),
            "reduced_rated" => Ok(ReducedRated),
            "reverse_charge" => Ok(ReverseCharge),
            "standard_rated" => Ok(StandardRated),
            "taxable_basis_reduced" => Ok(TaxableBasisReduced),
            "vat_exempt" => Ok(VatExempt),
            "zero_rated" => Ok(ZeroRated),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for TaxTaxabilityReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxTaxabilityReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for TaxTaxabilityReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TaxTaxabilityReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for TaxTaxabilityReason"))
    }
}
