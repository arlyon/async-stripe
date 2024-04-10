#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxProductResourceLineItemTaxRateDetails {
    /// A localized display name for tax type, intended to be human-readable.
    /// For example, "Local Sales and Use Tax", "Value-added tax (VAT)", or "Umsatzsteuer (USt.)".
    pub display_name: String,
    /// The tax rate percentage as a string. For example, 8.5% is represented as "8.5".
    pub percentage_decimal: String,
    /// The tax type, such as `vat` or `sales_tax`.
    pub tax_type: TaxProductResourceLineItemTaxRateDetailsTaxType,
}
#[doc(hidden)]
pub struct TaxProductResourceLineItemTaxRateDetailsBuilder {
    display_name: Option<String>,
    percentage_decimal: Option<String>,
    tax_type: Option<TaxProductResourceLineItemTaxRateDetailsTaxType>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TaxProductResourceLineItemTaxRateDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductResourceLineItemTaxRateDetails>,
        builder: TaxProductResourceLineItemTaxRateDetailsBuilder,
    }

    impl Visitor for Place<TaxProductResourceLineItemTaxRateDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TaxProductResourceLineItemTaxRateDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TaxProductResourceLineItemTaxRateDetailsBuilder {
        type Out = TaxProductResourceLineItemTaxRateDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "display_name" => Deserialize::begin(&mut self.display_name),
                "percentage_decimal" => Deserialize::begin(&mut self.percentage_decimal),
                "tax_type" => Deserialize::begin(&mut self.tax_type),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                display_name: Deserialize::default(),
                percentage_decimal: Deserialize::default(),
                tax_type: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                display_name: self.display_name.take()?,
                percentage_decimal: self.percentage_decimal.take()?,
                tax_type: self.tax_type?,
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

    impl ObjectDeser for TaxProductResourceLineItemTaxRateDetails {
        type Builder = TaxProductResourceLineItemTaxRateDetailsBuilder;
    }

    impl FromValueOpt for TaxProductResourceLineItemTaxRateDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TaxProductResourceLineItemTaxRateDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "display_name" => b.display_name = Some(FromValueOpt::from_value(v)?),
                    "percentage_decimal" => {
                        b.percentage_decimal = Some(FromValueOpt::from_value(v)?)
                    }
                    "tax_type" => b.tax_type = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The tax type, such as `vat` or `sales_tax`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxProductResourceLineItemTaxRateDetailsTaxType {
    AmusementTax,
    CommunicationsTax,
    Gst,
    Hst,
    Igst,
    Jct,
    LeaseTax,
    Pst,
    Qst,
    Rst,
    SalesTax,
    Vat,
}
impl TaxProductResourceLineItemTaxRateDetailsTaxType {
    pub fn as_str(self) -> &'static str {
        use TaxProductResourceLineItemTaxRateDetailsTaxType::*;
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
            Rst => "rst",
            SalesTax => "sales_tax",
            Vat => "vat",
        }
    }
}

impl std::str::FromStr for TaxProductResourceLineItemTaxRateDetailsTaxType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductResourceLineItemTaxRateDetailsTaxType::*;
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
            "rst" => Ok(Rst),
            "sales_tax" => Ok(SalesTax),
            "vat" => Ok(Vat),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for TaxProductResourceLineItemTaxRateDetailsTaxType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxProductResourceLineItemTaxRateDetailsTaxType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TaxProductResourceLineItemTaxRateDetailsTaxType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TaxProductResourceLineItemTaxRateDetailsTaxType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<TaxProductResourceLineItemTaxRateDetailsTaxType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TaxProductResourceLineItemTaxRateDetailsTaxType::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TaxProductResourceLineItemTaxRateDetailsTaxType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TaxProductResourceLineItemTaxRateDetailsTaxType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for TaxProductResourceLineItemTaxRateDetailsTaxType",
            )
        })
    }
}
