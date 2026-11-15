#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxProductResourceLineItemTaxRateDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TaxProductResourceLineItemTaxRateDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TaxProductResourceLineItemTaxRateDetailsBuilder {
    display_name: Option<String>,
    percentage_decimal: Option<String>,
    tax_type: Option<TaxProductResourceLineItemTaxRateDetailsTaxType>,
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
            Self { display_name: None, percentage_decimal: None, tax_type: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(display_name), Some(percentage_decimal), Some(tax_type)) =
                (self.display_name.take(), self.percentage_decimal.take(), self.tax_type.take())
            else {
                return None;
            };
            Some(Self::Out { display_name, percentage_decimal, tax_type })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
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
                    "display_name" => b.display_name = FromValueOpt::from_value(v),
                    "percentage_decimal" => b.percentage_decimal = FromValueOpt::from_value(v),
                    "tax_type" => b.tax_type = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The tax type, such as `vat` or `sales_tax`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TaxProductResourceLineItemTaxRateDetailsTaxType {
    AdmissionsTax,
    AmusementTax,
    AttendanceTax,
    CommunicationsTax,
    DigitalExciseTax,
    EntertainmentTax,
    GetFund,
    GrossReceiptsTax,
    Gst,
    HospitalityTax,
    Hst,
    Igst,
    Jct,
    LeaseTax,
    LuxuryTax,
    MassTransitParkingTax,
    Nhil,
    ParkingTax,
    Pst,
    Qst,
    RecyclingFee,
    ResortTax,
    RetailDeliveryFee,
    Rst,
    SalesTax,
    ServiceTax,
    TourismTax,
    UtilityUsersTax,
    Vat,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TaxProductResourceLineItemTaxRateDetailsTaxType {
    pub fn as_str(&self) -> &str {
        use TaxProductResourceLineItemTaxRateDetailsTaxType::*;
        match self {
            AdmissionsTax => "admissions_tax",
            AmusementTax => "amusement_tax",
            AttendanceTax => "attendance_tax",
            CommunicationsTax => "communications_tax",
            DigitalExciseTax => "digital_excise_tax",
            EntertainmentTax => "entertainment_tax",
            GetFund => "get_fund",
            GrossReceiptsTax => "gross_receipts_tax",
            Gst => "gst",
            HospitalityTax => "hospitality_tax",
            Hst => "hst",
            Igst => "igst",
            Jct => "jct",
            LeaseTax => "lease_tax",
            LuxuryTax => "luxury_tax",
            MassTransitParkingTax => "mass_transit_parking_tax",
            Nhil => "nhil",
            ParkingTax => "parking_tax",
            Pst => "pst",
            Qst => "qst",
            RecyclingFee => "recycling_fee",
            ResortTax => "resort_tax",
            RetailDeliveryFee => "retail_delivery_fee",
            Rst => "rst",
            SalesTax => "sales_tax",
            ServiceTax => "service_tax",
            TourismTax => "tourism_tax",
            UtilityUsersTax => "utility_users_tax",
            Vat => "vat",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TaxProductResourceLineItemTaxRateDetailsTaxType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductResourceLineItemTaxRateDetailsTaxType::*;
        match s {
            "admissions_tax" => Ok(AdmissionsTax),
            "amusement_tax" => Ok(AmusementTax),
            "attendance_tax" => Ok(AttendanceTax),
            "communications_tax" => Ok(CommunicationsTax),
            "digital_excise_tax" => Ok(DigitalExciseTax),
            "entertainment_tax" => Ok(EntertainmentTax),
            "get_fund" => Ok(GetFund),
            "gross_receipts_tax" => Ok(GrossReceiptsTax),
            "gst" => Ok(Gst),
            "hospitality_tax" => Ok(HospitalityTax),
            "hst" => Ok(Hst),
            "igst" => Ok(Igst),
            "jct" => Ok(Jct),
            "lease_tax" => Ok(LeaseTax),
            "luxury_tax" => Ok(LuxuryTax),
            "mass_transit_parking_tax" => Ok(MassTransitParkingTax),
            "nhil" => Ok(Nhil),
            "parking_tax" => Ok(ParkingTax),
            "pst" => Ok(Pst),
            "qst" => Ok(Qst),
            "recycling_fee" => Ok(RecyclingFee),
            "resort_tax" => Ok(ResortTax),
            "retail_delivery_fee" => Ok(RetailDeliveryFee),
            "rst" => Ok(Rst),
            "sales_tax" => Ok(SalesTax),
            "service_tax" => Ok(ServiceTax),
            "tourism_tax" => Ok(TourismTax),
            "utility_users_tax" => Ok(UtilityUsersTax),
            "vat" => Ok(Vat),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TaxProductResourceLineItemTaxRateDetailsTaxType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TaxProductResourceLineItemTaxRateDetailsTaxType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for TaxProductResourceLineItemTaxRateDetailsTaxType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxProductResourceLineItemTaxRateDetailsTaxType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(TaxProductResourceLineItemTaxRateDetailsTaxType))
            .finish_non_exhaustive()
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
        self.out =
            Some(TaxProductResourceLineItemTaxRateDetailsTaxType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TaxProductResourceLineItemTaxRateDetailsTaxType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TaxProductResourceLineItemTaxRateDetailsTaxType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
