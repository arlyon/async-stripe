#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LineItemTaxBreakdown {
    /// The amount of tax, in integer cents.
pub amount: i64,
pub jurisdiction: stripe_misc::tax::calculation_line_item::line_item_tax_breakdown::jurisdiction::Jurisdiction,
    /// Indicates whether the jurisdiction was determined by the origin (merchant's address) or destination (customer's address).
pub sourcing: LineItemTaxBreakdownSourcing,
    /// Details regarding the rate for this tax.
    ///
    /// This field will be `null` when the tax is not imposed, for example if the product is exempt from tax.
pub tax_rate_details: Option<stripe_misc::tax::calculation_line_item::line_item_tax_breakdown::line_item_tax_rate_details::LineItemTaxRateDetails>,
    /// The reasoning behind this tax, for example, if the product is tax exempt.
    ///
    /// The possible values for this field may be extended as new tax rules are supported.
pub taxability_reason: LineItemTaxBreakdownTaxabilityReason,
    /// The amount on which tax is calculated, in integer cents.
pub taxable_amount: i64,

}
/// Indicates whether the jurisdiction was determined by the origin (merchant's address) or destination (customer's address).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum LineItemTaxBreakdownSourcing {
    Destination,
    Origin,
}

impl LineItemTaxBreakdownSourcing {
    pub fn as_str(self) -> &'static str {
        use LineItemTaxBreakdownSourcing::*;
        match self {
            Destination => "destination",
            Origin => "origin",
        }
    }
}

impl std::str::FromStr for LineItemTaxBreakdownSourcing {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use LineItemTaxBreakdownSourcing::*;
        match s {
            "destination" => Ok(Destination),
            "origin" => Ok(Origin),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for LineItemTaxBreakdownSourcing {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for LineItemTaxBreakdownSourcing {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for LineItemTaxBreakdownSourcing {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LineItemTaxBreakdownSourcing {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for LineItemTaxBreakdownSourcing"))
    }
}
/// The reasoning behind this tax, for example, if the product is tax exempt.
///
/// The possible values for this field may be extended as new tax rules are supported.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum LineItemTaxBreakdownTaxabilityReason {
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
}

impl LineItemTaxBreakdownTaxabilityReason {
    pub fn as_str(self) -> &'static str {
        use LineItemTaxBreakdownTaxabilityReason::*;
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
        }
    }
}

impl std::str::FromStr for LineItemTaxBreakdownTaxabilityReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use LineItemTaxBreakdownTaxabilityReason::*;
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

impl AsRef<str> for LineItemTaxBreakdownTaxabilityReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for LineItemTaxBreakdownTaxabilityReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for LineItemTaxBreakdownTaxabilityReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LineItemTaxBreakdownTaxabilityReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom("Unknown value for LineItemTaxBreakdownTaxabilityReason")
        })
    }
}
pub mod jurisdiction;
pub use jurisdiction::Jurisdiction;
pub mod line_item_tax_rate_details;
pub use line_item_tax_rate_details::LineItemTaxRateDetails;
