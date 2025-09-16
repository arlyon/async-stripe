#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxProductResourceLineItemTaxBreakdown {
    /// The amount of tax, in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount: i64,
    pub jurisdiction: stripe_misc::TaxProductResourceJurisdiction,
    /// Indicates whether the jurisdiction was determined by the origin (merchant's address) or destination (customer's address).
    pub sourcing: TaxProductResourceLineItemTaxBreakdownSourcing,
    /// Details regarding the rate for this tax.
    /// This field will be `null` when the tax is not imposed, for example if the product is exempt from tax.
    pub tax_rate_details: Option<stripe_misc::TaxProductResourceLineItemTaxRateDetails>,
    /// The reasoning behind this tax, for example, if the product is tax exempt.
    /// The possible values for this field may be extended as new tax rules are supported.
    pub taxability_reason: TaxProductResourceLineItemTaxBreakdownTaxabilityReason,
    /// The amount on which tax is calculated, in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub taxable_amount: i64,
}
#[doc(hidden)]
pub struct TaxProductResourceLineItemTaxBreakdownBuilder {
    amount: Option<i64>,
    jurisdiction: Option<stripe_misc::TaxProductResourceJurisdiction>,
    sourcing: Option<TaxProductResourceLineItemTaxBreakdownSourcing>,
    tax_rate_details: Option<Option<stripe_misc::TaxProductResourceLineItemTaxRateDetails>>,
    taxability_reason: Option<TaxProductResourceLineItemTaxBreakdownTaxabilityReason>,
    taxable_amount: Option<i64>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TaxProductResourceLineItemTaxBreakdown {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductResourceLineItemTaxBreakdown>,
        builder: TaxProductResourceLineItemTaxBreakdownBuilder,
    }

    impl Visitor for Place<TaxProductResourceLineItemTaxBreakdown> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TaxProductResourceLineItemTaxBreakdownBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TaxProductResourceLineItemTaxBreakdownBuilder {
        type Out = TaxProductResourceLineItemTaxBreakdown;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "jurisdiction" => Deserialize::begin(&mut self.jurisdiction),
                "sourcing" => Deserialize::begin(&mut self.sourcing),
                "tax_rate_details" => Deserialize::begin(&mut self.tax_rate_details),
                "taxability_reason" => Deserialize::begin(&mut self.taxability_reason),
                "taxable_amount" => Deserialize::begin(&mut self.taxable_amount),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                jurisdiction: Deserialize::default(),
                sourcing: Deserialize::default(),
                tax_rate_details: Deserialize::default(),
                taxability_reason: Deserialize::default(),
                taxable_amount: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(amount),
                Some(jurisdiction),
                Some(sourcing),
                Some(tax_rate_details),
                Some(taxability_reason),
                Some(taxable_amount),
            ) = (
                self.amount,
                self.jurisdiction.take(),
                self.sourcing,
                self.tax_rate_details.take(),
                self.taxability_reason.take(),
                self.taxable_amount,
            )
            else {
                return None;
            };
            Some(Self::Out {
                amount,
                jurisdiction,
                sourcing,
                tax_rate_details,
                taxability_reason,
                taxable_amount,
            })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for TaxProductResourceLineItemTaxBreakdown {
        type Builder = TaxProductResourceLineItemTaxBreakdownBuilder;
    }

    impl FromValueOpt for TaxProductResourceLineItemTaxBreakdown {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TaxProductResourceLineItemTaxBreakdownBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "jurisdiction" => b.jurisdiction = FromValueOpt::from_value(v),
                    "sourcing" => b.sourcing = FromValueOpt::from_value(v),
                    "tax_rate_details" => b.tax_rate_details = FromValueOpt::from_value(v),
                    "taxability_reason" => b.taxability_reason = FromValueOpt::from_value(v),
                    "taxable_amount" => b.taxable_amount = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
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
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductResourceLineItemTaxBreakdownSourcing::*;
        match s {
            "destination" => Ok(Destination),
            "origin" => Ok(Origin),
            _ => Err(stripe_types::StripeParseError),
        }
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
#[cfg(feature = "serialize")]
impl serde::Serialize for TaxProductResourceLineItemTaxBreakdownSourcing {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TaxProductResourceLineItemTaxBreakdownSourcing {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<TaxProductResourceLineItemTaxBreakdownSourcing> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TaxProductResourceLineItemTaxBreakdownSourcing::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TaxProductResourceLineItemTaxBreakdownSourcing);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TaxProductResourceLineItemTaxBreakdownSourcing {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for TaxProductResourceLineItemTaxBreakdownSourcing",
            )
        })
    }
}
/// The reasoning behind this tax, for example, if the product is tax exempt.
/// The possible values for this field may be extended as new tax rules are supported.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
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
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TaxProductResourceLineItemTaxBreakdownTaxabilityReason {
    pub fn as_str(&self) -> &str {
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
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TaxProductResourceLineItemTaxBreakdownTaxabilityReason {
    type Err = std::convert::Infallible;
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
            v => Ok(Unknown(v.to_owned())),
        }
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
#[cfg(feature = "serialize")]
impl serde::Serialize for TaxProductResourceLineItemTaxBreakdownTaxabilityReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TaxProductResourceLineItemTaxBreakdownTaxabilityReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<TaxProductResourceLineItemTaxBreakdownTaxabilityReason>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(TaxProductResourceLineItemTaxBreakdownTaxabilityReason::from_str(s).unwrap());
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TaxProductResourceLineItemTaxBreakdownTaxabilityReason);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TaxProductResourceLineItemTaxBreakdownTaxabilityReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
