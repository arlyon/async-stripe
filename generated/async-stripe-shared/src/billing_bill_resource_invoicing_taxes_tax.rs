#[derive(Clone, Debug)]
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
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: BillingBillResourceInvoicingTaxesTaxBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BillingBillResourceInvoicingTaxesTaxBuilder {
        type Out = BillingBillResourceInvoicingTaxesTax;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "tax_behavior" => Deserialize::begin(&mut self.tax_behavior),
                "tax_rate_details" => Deserialize::begin(&mut self.tax_rate_details),
                "taxability_reason" => Deserialize::begin(&mut self.taxability_reason),
                "taxable_amount" => Deserialize::begin(&mut self.taxable_amount),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                tax_behavior: Deserialize::default(),
                tax_rate_details: Deserialize::default(),
                taxability_reason: Deserialize::default(),
                taxable_amount: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(amount),
                Some(tax_behavior),
                Some(tax_rate_details),
                Some(taxability_reason),
                Some(taxable_amount),
                Some(type_),
            ) = (
                self.amount,
                self.tax_behavior,
                self.tax_rate_details.take(),
                self.taxability_reason.take(),
                self.taxable_amount,
                self.type_,
            )
            else {
                return None;
            };
            Some(Self::Out {
                amount,
                tax_behavior,
                tax_rate_details,
                taxability_reason,
                taxable_amount,
                type_,
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

    impl ObjectDeser for BillingBillResourceInvoicingTaxesTax {
        type Builder = BillingBillResourceInvoicingTaxesTaxBuilder;
    }

    impl FromValueOpt for BillingBillResourceInvoicingTaxesTax {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BillingBillResourceInvoicingTaxesTaxBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "tax_behavior" => b.tax_behavior = FromValueOpt::from_value(v),
                    "tax_rate_details" => b.tax_rate_details = FromValueOpt::from_value(v),
                    "taxability_reason" => b.taxability_reason = FromValueOpt::from_value(v),
                    "taxable_amount" => b.taxable_amount = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Whether this tax is inclusive or exclusive.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BillingBillResourceInvoicingTaxesTaxTaxBehavior {
    Exclusive,
    Inclusive,
}
impl BillingBillResourceInvoicingTaxesTaxTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use BillingBillResourceInvoicingTaxesTaxTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
        }
    }
}

impl std::str::FromStr for BillingBillResourceInvoicingTaxesTaxTaxBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BillingBillResourceInvoicingTaxesTaxTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for BillingBillResourceInvoicingTaxesTaxTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for BillingBillResourceInvoicingTaxesTaxTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for BillingBillResourceInvoicingTaxesTaxTaxBehavior {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<BillingBillResourceInvoicingTaxesTaxTaxBehavior> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            BillingBillResourceInvoicingTaxesTaxTaxBehavior::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(BillingBillResourceInvoicingTaxesTaxTaxBehavior);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BillingBillResourceInvoicingTaxesTaxTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for BillingBillResourceInvoicingTaxesTaxTaxBehavior",
            )
        })
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
            v => Ok(Unknown(v.to_owned())),
        }
    }
}
impl std::fmt::Display for BillingBillResourceInvoicingTaxesTaxTaxabilityReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for BillingBillResourceInvoicingTaxesTaxTaxabilityReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for BillingBillResourceInvoicingTaxesTaxTaxabilityReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<BillingBillResourceInvoicingTaxesTaxTaxabilityReason> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(BillingBillResourceInvoicingTaxesTaxTaxabilityReason::from_str(s).unwrap());
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(BillingBillResourceInvoicingTaxesTaxTaxabilityReason);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BillingBillResourceInvoicingTaxesTaxTaxabilityReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
/// The type of tax information.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BillingBillResourceInvoicingTaxesTaxType {
    TaxRateDetails,
}
impl BillingBillResourceInvoicingTaxesTaxType {
    pub fn as_str(self) -> &'static str {
        use BillingBillResourceInvoicingTaxesTaxType::*;
        match self {
            TaxRateDetails => "tax_rate_details",
        }
    }
}

impl std::str::FromStr for BillingBillResourceInvoicingTaxesTaxType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BillingBillResourceInvoicingTaxesTaxType::*;
        match s {
            "tax_rate_details" => Ok(TaxRateDetails),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for BillingBillResourceInvoicingTaxesTaxType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for BillingBillResourceInvoicingTaxesTaxType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for BillingBillResourceInvoicingTaxesTaxType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<BillingBillResourceInvoicingTaxesTaxType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            BillingBillResourceInvoicingTaxesTaxType::from_str(s).map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(BillingBillResourceInvoicingTaxesTaxType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BillingBillResourceInvoicingTaxesTaxType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for BillingBillResourceInvoicingTaxesTaxType")
        })
    }
}
