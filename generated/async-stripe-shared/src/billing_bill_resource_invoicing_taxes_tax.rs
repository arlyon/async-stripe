#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingBillResourceInvoicingTaxesTax {
    /// The amount of the tax, in cents (or local equivalent).
    pub amount: i64,
    /// Whether this tax is inclusive or exclusive.
    pub tax_behavior: BillingBillResourceInvoicingTaxesTaxTaxBehavior,
    /// Additional details about the tax rate. Only present when `type` is `tax_rate_details`.
    pub tax_rate_details: Option<stripe_shared::BillingBillResourceInvoicingTaxesTaxRateDetails>,
    /// The reasoning behind this tax, for example, if the product is tax exempt.
    /// The possible values for this field may be extended as new tax rules are supported.
    pub taxability_reason: BillingBillResourceInvoicingTaxesTaxTaxabilityReason,
    /// The amount on which tax is calculated, in cents (or local equivalent).
    pub taxable_amount: Option<i64>,
    /// The type of tax information.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: BillingBillResourceInvoicingTaxesTaxType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingBillResourceInvoicingTaxesTax {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BillingBillResourceInvoicingTaxesTax").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct BillingBillResourceInvoicingTaxesTaxBuilder {
    amount: Option<i64>,
    tax_behavior: Option<BillingBillResourceInvoicingTaxesTaxTaxBehavior>,
    tax_rate_details:
        Option<Option<stripe_shared::BillingBillResourceInvoicingTaxesTaxRateDetails>>,
    taxability_reason: Option<BillingBillResourceInvoicingTaxesTaxTaxabilityReason>,
    taxable_amount: Option<Option<i64>>,
    type_: Option<BillingBillResourceInvoicingTaxesTaxType>,
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

    impl Deserialize for BillingBillResourceInvoicingTaxesTax {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BillingBillResourceInvoicingTaxesTax>,
        builder: BillingBillResourceInvoicingTaxesTaxBuilder,
    }

    impl Visitor for Place<BillingBillResourceInvoicingTaxesTax> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BillingBillResourceInvoicingTaxesTaxBuilder {
                    amount: Deserialize::default(),
                    tax_behavior: Deserialize::default(),
                    tax_rate_details: Deserialize::default(),
                    taxability_reason: Deserialize::default(),
                    taxable_amount: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.builder.amount),
                "tax_behavior" => Deserialize::begin(&mut self.builder.tax_behavior),
                "tax_rate_details" => Deserialize::begin(&mut self.builder.tax_rate_details),
                "taxability_reason" => Deserialize::begin(&mut self.builder.taxability_reason),
                "taxable_amount" => Deserialize::begin(&mut self.builder.taxable_amount),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(amount),
                Some(tax_behavior),
                Some(tax_rate_details),
                Some(taxability_reason),
                Some(taxable_amount),
                Some(type_),
            ) = (
                self.builder.amount,
                self.builder.tax_behavior.take(),
                self.builder.tax_rate_details.take(),
                self.builder.taxability_reason.take(),
                self.builder.taxable_amount,
                self.builder.type_.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(BillingBillResourceInvoicingTaxesTax {
                amount,
                tax_behavior,
                tax_rate_details,
                taxability_reason,
                taxable_amount,
                type_,
            });
            Ok(())
        }
    }
};
/// Whether this tax is inclusive or exclusive.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum BillingBillResourceInvoicingTaxesTaxTaxBehavior {
    Exclusive,
    Inclusive,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl BillingBillResourceInvoicingTaxesTaxTaxBehavior {
    pub fn as_str(&self) -> &str {
        use BillingBillResourceInvoicingTaxesTaxTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for BillingBillResourceInvoicingTaxesTaxTaxBehavior {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BillingBillResourceInvoicingTaxesTaxTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "BillingBillResourceInvoicingTaxesTaxTaxBehavior"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for BillingBillResourceInvoicingTaxesTaxTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for BillingBillResourceInvoicingTaxesTaxTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingBillResourceInvoicingTaxesTaxTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(BillingBillResourceInvoicingTaxesTaxTaxBehavior))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for BillingBillResourceInvoicingTaxesTaxTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for BillingBillResourceInvoicingTaxesTaxTaxBehavior {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<BillingBillResourceInvoicingTaxesTaxTaxBehavior> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(BillingBillResourceInvoicingTaxesTaxTaxBehavior::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BillingBillResourceInvoicingTaxesTaxTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The reasoning behind this tax, for example, if the product is tax exempt.
/// The possible values for this field may be extended as new tax rules are supported.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum BillingBillResourceInvoicingTaxesTaxTaxabilityReason {
    CustomerExempt,
    NotAvailable,
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
impl BillingBillResourceInvoicingTaxesTaxTaxabilityReason {
    pub fn as_str(&self) -> &str {
        use BillingBillResourceInvoicingTaxesTaxTaxabilityReason::*;
        match self {
            CustomerExempt => "customer_exempt",
            NotAvailable => "not_available",
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

impl std::str::FromStr for BillingBillResourceInvoicingTaxesTaxTaxabilityReason {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BillingBillResourceInvoicingTaxesTaxTaxabilityReason::*;
        match s {
            "customer_exempt" => Ok(CustomerExempt),
            "not_available" => Ok(NotAvailable),
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
                    "BillingBillResourceInvoicingTaxesTaxTaxabilityReason"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for BillingBillResourceInvoicingTaxesTaxTaxabilityReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for BillingBillResourceInvoicingTaxesTaxTaxabilityReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingBillResourceInvoicingTaxesTaxTaxabilityReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(BillingBillResourceInvoicingTaxesTaxTaxabilityReason))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for BillingBillResourceInvoicingTaxesTaxTaxabilityReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for BillingBillResourceInvoicingTaxesTaxTaxabilityReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<BillingBillResourceInvoicingTaxesTaxTaxabilityReason> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            BillingBillResourceInvoicingTaxesTaxTaxabilityReason::from_str(s).expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BillingBillResourceInvoicingTaxesTaxTaxabilityReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The type of tax information.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum BillingBillResourceInvoicingTaxesTaxType {
    TaxRateDetails,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl BillingBillResourceInvoicingTaxesTaxType {
    pub fn as_str(&self) -> &str {
        use BillingBillResourceInvoicingTaxesTaxType::*;
        match self {
            TaxRateDetails => "tax_rate_details",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for BillingBillResourceInvoicingTaxesTaxType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BillingBillResourceInvoicingTaxesTaxType::*;
        match s {
            "tax_rate_details" => Ok(TaxRateDetails),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "BillingBillResourceInvoicingTaxesTaxType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for BillingBillResourceInvoicingTaxesTaxType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for BillingBillResourceInvoicingTaxesTaxType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingBillResourceInvoicingTaxesTaxType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(BillingBillResourceInvoicingTaxesTaxType)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for BillingBillResourceInvoicingTaxesTaxType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for BillingBillResourceInvoicingTaxesTaxType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<BillingBillResourceInvoicingTaxesTaxType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(BillingBillResourceInvoicingTaxesTaxType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BillingBillResourceInvoicingTaxesTaxType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
