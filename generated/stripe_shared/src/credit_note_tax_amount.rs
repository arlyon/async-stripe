#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CreditNoteTaxAmount {
    /// The amount, in cents (or local equivalent), of the tax.
    pub amount: i64,
    /// Whether this tax amount is inclusive or exclusive.
    pub inclusive: bool,
    /// The tax rate that was applied to get this tax amount.
    pub tax_rate: stripe_types::Expandable<stripe_shared::TaxRate>,
    /// The reasoning behind this tax, for example, if the product is tax exempt.
    /// The possible values for this field may be extended as new tax rules are supported.
    pub taxability_reason: Option<CreditNoteTaxAmountTaxabilityReason>,
    /// The amount on which tax is calculated, in cents (or local equivalent).
    pub taxable_amount: Option<i64>,
}
#[doc(hidden)]
pub struct CreditNoteTaxAmountBuilder {
    amount: Option<i64>,
    inclusive: Option<bool>,
    tax_rate: Option<stripe_types::Expandable<stripe_shared::TaxRate>>,
    taxability_reason: Option<Option<CreditNoteTaxAmountTaxabilityReason>>,
    taxable_amount: Option<Option<i64>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for CreditNoteTaxAmount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CreditNoteTaxAmount>,
        builder: CreditNoteTaxAmountBuilder,
    }

    impl Visitor for Place<CreditNoteTaxAmount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CreditNoteTaxAmountBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for CreditNoteTaxAmountBuilder {
        type Out = CreditNoteTaxAmount;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "inclusive" => Deserialize::begin(&mut self.inclusive),
                "tax_rate" => Deserialize::begin(&mut self.tax_rate),
                "taxability_reason" => Deserialize::begin(&mut self.taxability_reason),
                "taxable_amount" => Deserialize::begin(&mut self.taxable_amount),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                inclusive: Deserialize::default(),
                tax_rate: Deserialize::default(),
                taxability_reason: Deserialize::default(),
                taxable_amount: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                amount: self.amount?,
                inclusive: self.inclusive?,
                tax_rate: self.tax_rate.take()?,
                taxability_reason: self.taxability_reason?,
                taxable_amount: self.taxable_amount?,
            })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for CreditNoteTaxAmount {
        type Builder = CreditNoteTaxAmountBuilder;
    }

    impl FromValueOpt for CreditNoteTaxAmount {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = CreditNoteTaxAmountBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = Some(FromValueOpt::from_value(v)?),
                    "inclusive" => b.inclusive = Some(FromValueOpt::from_value(v)?),
                    "tax_rate" => b.tax_rate = Some(FromValueOpt::from_value(v)?),
                    "taxability_reason" => b.taxability_reason = Some(FromValueOpt::from_value(v)?),
                    "taxable_amount" => b.taxable_amount = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The reasoning behind this tax, for example, if the product is tax exempt.
/// The possible values for this field may be extended as new tax rules are supported.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreditNoteTaxAmountTaxabilityReason {
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
impl CreditNoteTaxAmountTaxabilityReason {
    pub fn as_str(self) -> &'static str {
        use CreditNoteTaxAmountTaxabilityReason::*;
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

impl std::str::FromStr for CreditNoteTaxAmountTaxabilityReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreditNoteTaxAmountTaxabilityReason::*;
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
impl std::fmt::Display for CreditNoteTaxAmountTaxabilityReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreditNoteTaxAmountTaxabilityReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CreditNoteTaxAmountTaxabilityReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CreditNoteTaxAmountTaxabilityReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CreditNoteTaxAmountTaxabilityReason> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            CreditNoteTaxAmountTaxabilityReason::from_str(s)
                .unwrap_or(CreditNoteTaxAmountTaxabilityReason::Unknown),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CreditNoteTaxAmountTaxabilityReason);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreditNoteTaxAmountTaxabilityReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap_or(Self::Unknown))
    }
}
