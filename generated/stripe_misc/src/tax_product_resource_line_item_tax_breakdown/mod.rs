#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TaxProductResourceLineItemTaxBreakdown {
    /// The amount of tax, in integer cents.
    pub amount: i64,
    pub jurisdiction: stripe_misc::TaxProductResourceJurisdiction,
    /// Indicates whether the jurisdiction was determined by the origin (merchant's address) or destination (customer's address).
    pub sourcing: TaxProductResourceLineItemTaxBreakdownSourcing,
    /// Details regarding the rate for this tax.
    ///
    /// This field will be `null` when the tax is not imposed, for example if the product is exempt from tax.
    pub tax_rate_details: Option<stripe_misc::TaxProductResourceLineItemTaxRateDetails>,
    /// The reasoning behind this tax, for example, if the product is tax exempt.
    ///
    /// The possible values for this field may be extended as new tax rules are supported.
    pub taxability_reason: TaxProductResourceLineItemTaxBreakdownTaxabilityReason,
    /// The amount on which tax is calculated, in integer cents.
    pub taxable_amount: i64,
}
/// Indicates whether the jurisdiction was determined by the origin (merchant's address) or destination (customer's address).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxProductResourceLineItemTaxBreakdownSourcing {
    Destination,
    Origin,
}

impl TaxProductResourceLineItemTaxBreakdownSourcing {
    pub fn as_str(self) -> &'static str {
        use TaxProductResourceLineItemTaxBreakdownSourcing::*;
        match self {
            Destination => "destination",
            Origin => "origin",
        }
    }
}

impl std::str::FromStr for TaxProductResourceLineItemTaxBreakdownSourcing {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductResourceLineItemTaxBreakdownSourcing::*;
        match s {
            "destination" => Ok(Destination),
            "origin" => Ok(Origin),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for TaxProductResourceLineItemTaxBreakdownSourcing {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxProductResourceLineItemTaxBreakdownSourcing {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxProductResourceLineItemTaxBreakdownSourcing {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TaxProductResourceLineItemTaxBreakdownSourcing {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TaxProductResourceLineItemTaxBreakdownSourcing {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for TaxProductResourceLineItemTaxBreakdownSourcing",
            )
        })
    }
}
/// The reasoning behind this tax, for example, if the product is tax exempt.
///
/// The possible values for this field may be extended as new tax rules are supported.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxProductResourceLineItemTaxBreakdownTaxabilityReason {
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

impl TaxProductResourceLineItemTaxBreakdownTaxabilityReason {
    pub fn as_str(self) -> &'static str {
        use TaxProductResourceLineItemTaxBreakdownTaxabilityReason::*;
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

impl std::str::FromStr for TaxProductResourceLineItemTaxBreakdownTaxabilityReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductResourceLineItemTaxBreakdownTaxabilityReason::*;
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

impl AsRef<str> for TaxProductResourceLineItemTaxBreakdownTaxabilityReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxProductResourceLineItemTaxBreakdownTaxabilityReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxProductResourceLineItemTaxBreakdownTaxabilityReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TaxProductResourceLineItemTaxBreakdownTaxabilityReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TaxProductResourceLineItemTaxBreakdownTaxabilityReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for TaxProductResourceLineItemTaxBreakdownTaxabilityReason",
            )
        })
    }
}
