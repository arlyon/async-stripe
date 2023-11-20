#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InvoiceTaxAmount {
    /// The amount, in cents (or local equivalent), of the tax.
    pub amount: i64,
    /// Whether this tax amount is inclusive or exclusive.
    pub inclusive: bool,
    /// The tax rate that was applied to get this tax amount.
    pub tax_rate: stripe_types::Expandable<stripe_types::TaxRate>,
    /// The reasoning behind this tax, for example, if the product is tax exempt.
    ///
    /// The possible values for this field may be extended as new tax rules are supported.
    pub taxability_reason: Option<InvoiceTaxAmountTaxabilityReason>,
    /// The amount on which tax is calculated, in cents (or local equivalent).
    pub taxable_amount: Option<i64>,
}
/// The reasoning behind this tax, for example, if the product is tax exempt.
///
/// The possible values for this field may be extended as new tax rules are supported.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum InvoiceTaxAmountTaxabilityReason {
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

impl InvoiceTaxAmountTaxabilityReason {
    pub fn as_str(self) -> &'static str {
        use InvoiceTaxAmountTaxabilityReason::*;
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

impl std::str::FromStr for InvoiceTaxAmountTaxabilityReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InvoiceTaxAmountTaxabilityReason::*;
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

impl AsRef<str> for InvoiceTaxAmountTaxabilityReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InvoiceTaxAmountTaxabilityReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InvoiceTaxAmountTaxabilityReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for InvoiceTaxAmountTaxabilityReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InvoiceTaxAmountTaxabilityReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap_or(InvoiceTaxAmountTaxabilityReason::Unknown))
    }
}
