#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TaxProductResourceTaxBreakdown {
    /// The amount of tax, in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount: i64,
    /// Specifies whether the tax amount is included in the line item amount.
    pub inclusive: bool,
    pub tax_rate_details: stripe_misc::TaxProductResourceTaxRateDetails,
    /// The reasoning behind this tax, for example, if the product is tax exempt.
    /// We might extend the possible values for this field to support new tax rules.
    pub taxability_reason: TaxProductResourceTaxBreakdownTaxabilityReason,
    /// The amount on which tax is calculated, in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub taxable_amount: i64,
}
/// The reasoning behind this tax, for example, if the product is tax exempt.
/// We might extend the possible values for this field to support new tax rules.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TaxProductResourceTaxBreakdownTaxabilityReason {
    CustomerExempt,
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
    ZeroRated,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl TaxProductResourceTaxBreakdownTaxabilityReason {
    pub fn as_str(self) -> &'static str {
        use TaxProductResourceTaxBreakdownTaxabilityReason::*;
        match self {
            CustomerExempt => "customer_exempt",
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
            ZeroRated => "zero_rated",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for TaxProductResourceTaxBreakdownTaxabilityReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductResourceTaxBreakdownTaxabilityReason::*;
        match s {
            "customer_exempt" => Ok(CustomerExempt),
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
            "zero_rated" => Ok(ZeroRated),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for TaxProductResourceTaxBreakdownTaxabilityReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxProductResourceTaxBreakdownTaxabilityReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TaxProductResourceTaxBreakdownTaxabilityReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TaxProductResourceTaxBreakdownTaxabilityReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap_or(TaxProductResourceTaxBreakdownTaxabilityReason::Unknown))
    }
}
