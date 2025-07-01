#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxProductResourceTaxRateDetails {
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: Option<String>,
    /// The amount of the tax rate when the `rate_type` is `flat_amount`.
    /// Tax rates with `rate_type` `percentage` can vary based on the transaction, resulting in this field being `null`.
    /// This field exposes the amount and currency of the flat tax rate.
    pub flat_amount: Option<stripe_shared::TaxRateFlatAmount>,
    /// The tax rate percentage as a string. For example, 8.5% is represented as `"8.5"`.
    pub percentage_decimal: String,
    /// Indicates the type of tax rate applied to the taxable amount.
    /// This value can be `null` when no tax applies to the location.
    /// This field is only present for TaxRates created by Stripe Tax.
    pub rate_type: Option<TaxProductResourceTaxRateDetailsRateType>,
    /// State, county, province, or region.
    pub state: Option<String>,
    /// The tax type, such as `vat` or `sales_tax`.
    pub tax_type: Option<TaxProductResourceTaxRateDetailsTaxType>,
}
#[doc(hidden)]
pub struct TaxProductResourceTaxRateDetailsBuilder {
    country: Option<Option<String>>,
    flat_amount: Option<Option<stripe_shared::TaxRateFlatAmount>>,
    percentage_decimal: Option<String>,
    rate_type: Option<Option<TaxProductResourceTaxRateDetailsRateType>>,
    state: Option<Option<String>>,
    tax_type: Option<Option<TaxProductResourceTaxRateDetailsTaxType>>,
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

    impl Deserialize for TaxProductResourceTaxRateDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductResourceTaxRateDetails>,
        builder: TaxProductResourceTaxRateDetailsBuilder,
    }

    impl Visitor for Place<TaxProductResourceTaxRateDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TaxProductResourceTaxRateDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TaxProductResourceTaxRateDetailsBuilder {
        type Out = TaxProductResourceTaxRateDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "country" => Deserialize::begin(&mut self.country),
                "flat_amount" => Deserialize::begin(&mut self.flat_amount),
                "percentage_decimal" => Deserialize::begin(&mut self.percentage_decimal),
                "rate_type" => Deserialize::begin(&mut self.rate_type),
                "state" => Deserialize::begin(&mut self.state),
                "tax_type" => Deserialize::begin(&mut self.tax_type),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                country: Deserialize::default(),
                flat_amount: Deserialize::default(),
                percentage_decimal: Deserialize::default(),
                rate_type: Deserialize::default(),
                state: Deserialize::default(),
                tax_type: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(country),
                Some(flat_amount),
                Some(percentage_decimal),
                Some(rate_type),
                Some(state),
                Some(tax_type),
            ) = (
                self.country.take(),
                self.flat_amount.take(),
                self.percentage_decimal.take(),
                self.rate_type,
                self.state.take(),
                self.tax_type.take(),
            )
            else {
                return None;
            };
            Some(Self::Out { country, flat_amount, percentage_decimal, rate_type, state, tax_type })
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

    impl ObjectDeser for TaxProductResourceTaxRateDetails {
        type Builder = TaxProductResourceTaxRateDetailsBuilder;
    }

    impl FromValueOpt for TaxProductResourceTaxRateDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TaxProductResourceTaxRateDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "country" => b.country = FromValueOpt::from_value(v),
                    "flat_amount" => b.flat_amount = FromValueOpt::from_value(v),
                    "percentage_decimal" => b.percentage_decimal = FromValueOpt::from_value(v),
                    "rate_type" => b.rate_type = FromValueOpt::from_value(v),
                    "state" => b.state = FromValueOpt::from_value(v),
                    "tax_type" => b.tax_type = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Indicates the type of tax rate applied to the taxable amount.
/// This value can be `null` when no tax applies to the location.
/// This field is only present for TaxRates created by Stripe Tax.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxProductResourceTaxRateDetailsRateType {
    FlatAmount,
    Percentage,
}
impl TaxProductResourceTaxRateDetailsRateType {
    pub fn as_str(self) -> &'static str {
        use TaxProductResourceTaxRateDetailsRateType::*;
        match self {
            FlatAmount => "flat_amount",
            Percentage => "percentage",
        }
    }
}

impl std::str::FromStr for TaxProductResourceTaxRateDetailsRateType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductResourceTaxRateDetailsRateType::*;
        match s {
            "flat_amount" => Ok(FlatAmount),
            "percentage" => Ok(Percentage),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for TaxProductResourceTaxRateDetailsRateType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxProductResourceTaxRateDetailsRateType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TaxProductResourceTaxRateDetailsRateType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TaxProductResourceTaxRateDetailsRateType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<TaxProductResourceTaxRateDetailsRateType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TaxProductResourceTaxRateDetailsRateType::from_str(s).map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TaxProductResourceTaxRateDetailsRateType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TaxProductResourceTaxRateDetailsRateType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for TaxProductResourceTaxRateDetailsRateType")
        })
    }
}
/// The tax type, such as `vat` or `sales_tax`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TaxProductResourceTaxRateDetailsTaxType {
    AmusementTax,
    CommunicationsTax,
    Gst,
    Hst,
    Igst,
    Jct,
    LeaseTax,
    Pst,
    Qst,
    RetailDeliveryFee,
    Rst,
    SalesTax,
    ServiceTax,
    Vat,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TaxProductResourceTaxRateDetailsTaxType {
    pub fn as_str(&self) -> &str {
        use TaxProductResourceTaxRateDetailsTaxType::*;
        match self {
            AmusementTax => "amusement_tax",
            CommunicationsTax => "communications_tax",
            Gst => "gst",
            Hst => "hst",
            Igst => "igst",
            Jct => "jct",
            LeaseTax => "lease_tax",
            Pst => "pst",
            Qst => "qst",
            RetailDeliveryFee => "retail_delivery_fee",
            Rst => "rst",
            SalesTax => "sales_tax",
            ServiceTax => "service_tax",
            Vat => "vat",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TaxProductResourceTaxRateDetailsTaxType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductResourceTaxRateDetailsTaxType::*;
        match s {
            "amusement_tax" => Ok(AmusementTax),
            "communications_tax" => Ok(CommunicationsTax),
            "gst" => Ok(Gst),
            "hst" => Ok(Hst),
            "igst" => Ok(Igst),
            "jct" => Ok(Jct),
            "lease_tax" => Ok(LeaseTax),
            "pst" => Ok(Pst),
            "qst" => Ok(Qst),
            "retail_delivery_fee" => Ok(RetailDeliveryFee),
            "rst" => Ok(Rst),
            "sales_tax" => Ok(SalesTax),
            "service_tax" => Ok(ServiceTax),
            "vat" => Ok(Vat),
            v => Ok(Unknown(v.to_owned())),
        }
    }
}
impl std::fmt::Display for TaxProductResourceTaxRateDetailsTaxType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxProductResourceTaxRateDetailsTaxType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TaxProductResourceTaxRateDetailsTaxType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TaxProductResourceTaxRateDetailsTaxType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<TaxProductResourceTaxRateDetailsTaxType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TaxProductResourceTaxRateDetailsTaxType::from_str(s).unwrap());
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TaxProductResourceTaxRateDetailsTaxType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TaxProductResourceTaxRateDetailsTaxType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
