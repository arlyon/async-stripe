#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxProductResourceTaxBreakdown {
    /// The amount of tax, in the [smallest currency unit](https://docs.stripe.com/currencies#minor-units).
    pub amount: i64,
    /// Specifies whether the tax amount is included in the line item amount.
    pub inclusive: bool,
    pub tax_rate_details: stripe_misc::TaxProductResourceTaxRateDetails,
    /// The reasoning behind this tax, for example, if the product is tax exempt.
    /// We might extend the possible values for this field to support new tax rules.
    pub taxability_reason: TaxProductResourceTaxBreakdownTaxabilityReason,
    /// The amount on which tax is calculated, in the [smallest currency unit](https://docs.stripe.com/currencies#minor-units).
    pub taxable_amount: i64,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxProductResourceTaxBreakdown {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TaxProductResourceTaxBreakdown").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TaxProductResourceTaxBreakdownBuilder {
    amount: Option<i64>,
    inclusive: Option<bool>,
    tax_rate_details: Option<stripe_misc::TaxProductResourceTaxRateDetails>,
    taxability_reason: Option<TaxProductResourceTaxBreakdownTaxabilityReason>,
    taxable_amount: Option<i64>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

    make_place!(Place);

    impl Deserialize for TaxProductResourceTaxBreakdown {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductResourceTaxBreakdown>,
        builder: TaxProductResourceTaxBreakdownBuilder,
    }

    impl Visitor for Place<TaxProductResourceTaxBreakdown> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TaxProductResourceTaxBreakdownBuilder {
                    amount: Deserialize::default(),
                    inclusive: Deserialize::default(),
                    tax_rate_details: Deserialize::default(),
                    taxability_reason: Deserialize::default(),
                    taxable_amount: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.builder.amount),
                "inclusive" => Deserialize::begin(&mut self.builder.inclusive),
                "tax_rate_details" => Deserialize::begin(&mut self.builder.tax_rate_details),
                "taxability_reason" => Deserialize::begin(&mut self.builder.taxability_reason),
                "taxable_amount" => Deserialize::begin(&mut self.builder.taxable_amount),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(amount),
                Some(inclusive),
                Some(tax_rate_details),
                Some(taxability_reason),
                Some(taxable_amount),
            ) = (
                self.builder.amount,
                self.builder.inclusive,
                self.builder.tax_rate_details.take(),
                self.builder.taxability_reason.take(),
                self.builder.taxable_amount,
            )
            else {
                return Ok(());
            };
            *self.out = Some(TaxProductResourceTaxBreakdown {
                amount,
                inclusive,
                tax_rate_details,
                taxability_reason,
                taxable_amount,
            });
            Ok(())
        }
    }
};
/// The reasoning behind this tax, for example, if the product is tax exempt.
/// We might extend the possible values for this field to support new tax rules.
#[derive(Clone, Eq, PartialEq)]
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
    Unknown(String),
}
impl TaxProductResourceTaxBreakdownTaxabilityReason {
    pub fn as_str(&self) -> &str {
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
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TaxProductResourceTaxBreakdownTaxabilityReason {
    type Err = std::convert::Infallible;
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
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TaxProductResourceTaxBreakdownTaxabilityReason"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TaxProductResourceTaxBreakdownTaxabilityReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for TaxProductResourceTaxBreakdownTaxabilityReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxProductResourceTaxBreakdownTaxabilityReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(TaxProductResourceTaxBreakdownTaxabilityReason))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TaxProductResourceTaxBreakdownTaxabilityReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for TaxProductResourceTaxBreakdownTaxabilityReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<TaxProductResourceTaxBreakdownTaxabilityReason> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(TaxProductResourceTaxBreakdownTaxabilityReason::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TaxProductResourceTaxBreakdownTaxabilityReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
