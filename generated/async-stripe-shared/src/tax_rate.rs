/// Tax rates can be applied to [invoices](/invoicing/taxes/tax-rates), [subscriptions](/billing/taxes/tax-rates) and [Checkout Sessions](/payments/checkout/use-manual-tax-rates) to collect tax.
///
/// Related guide: [Tax rates](/billing/taxes/tax-rates)
///
/// For more details see <<https://stripe.com/docs/api/tax_rates/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxRate {
    /// Defaults to `true`.
    /// When set to `false`, this tax rate cannot be used with new applications or Checkout Sessions, but will still work for subscriptions and invoices that already have it set.
    pub active: bool,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: Option<String>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// An arbitrary string attached to the tax rate for your internal use only.
    /// It will not be visible to your customers.
    pub description: Option<String>,
    /// The display name of the tax rates as it will appear to your customer on their receipt email, PDF, and the hosted invoice page.
    pub display_name: String,
    /// Actual/effective tax rate percentage out of 100.
    /// For tax calculations with automatic_tax[enabled]=true,.
    /// this percentage reflects the rate actually used to calculate tax based on the product's taxability
    /// and whether the user is registered to collect taxes in the corresponding jurisdiction.
    pub effective_percentage: Option<f64>,
    /// The amount of the tax rate when the `rate_type` is `flat_amount`.
    /// Tax rates with `rate_type` `percentage` can vary based on the transaction, resulting in this field being `null`.
    /// This field exposes the amount and currency of the flat tax rate.
    pub flat_amount: Option<stripe_shared::TaxRateFlatAmount>,
    /// Unique identifier for the object.
    pub id: stripe_shared::TaxRateId,
    /// This specifies if the tax rate is inclusive or exclusive.
    pub inclusive: bool,
    /// The jurisdiction for the tax rate.
    /// You can use this label field for tax reporting purposes.
    /// It also appears on your customer’s invoice.
    pub jurisdiction: Option<String>,
    /// The level of the jurisdiction that imposes this tax rate.
    /// Will be `null` for manually defined tax rates.
    pub jurisdiction_level: Option<TaxRateJurisdictionLevel>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Tax rate percentage out of 100.
    /// For tax calculations with automatic_tax[enabled]=true, this percentage includes the statutory tax rate of non-taxable jurisdictions.
    pub percentage: f64,
    /// Indicates the type of tax rate applied to the taxable amount.
    /// This value can be `null` when no tax applies to the location.
    /// This field is only present for TaxRates created by Stripe Tax.
    pub rate_type: Option<TaxRateRateType>,
    /// [ISO 3166-2 subdivision code](https://en.wikipedia.org/wiki/ISO_3166-2), without country prefix.
    /// For example, "NY" for New York, United States.
    pub state: Option<String>,
    /// The high-level tax type, such as `vat` or `sales_tax`.
    pub tax_type: Option<stripe_shared::TaxRateTaxType>,
}
#[doc(hidden)]
pub struct TaxRateBuilder {
    active: Option<bool>,
    country: Option<Option<String>>,
    created: Option<stripe_types::Timestamp>,
    description: Option<Option<String>>,
    display_name: Option<String>,
    effective_percentage: Option<Option<f64>>,
    flat_amount: Option<Option<stripe_shared::TaxRateFlatAmount>>,
    id: Option<stripe_shared::TaxRateId>,
    inclusive: Option<bool>,
    jurisdiction: Option<Option<String>>,
    jurisdiction_level: Option<Option<TaxRateJurisdictionLevel>>,
    livemode: Option<bool>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    percentage: Option<f64>,
    rate_type: Option<Option<TaxRateRateType>>,
    state: Option<Option<String>>,
    tax_type: Option<Option<stripe_shared::TaxRateTaxType>>,
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

    impl Deserialize for TaxRate {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxRate>,
        builder: TaxRateBuilder,
    }

    impl Visitor for Place<TaxRate> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: TaxRateBuilder::deser_default() }))
        }
    }

    impl MapBuilder for TaxRateBuilder {
        type Out = TaxRate;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "active" => Deserialize::begin(&mut self.active),
                "country" => Deserialize::begin(&mut self.country),
                "created" => Deserialize::begin(&mut self.created),
                "description" => Deserialize::begin(&mut self.description),
                "display_name" => Deserialize::begin(&mut self.display_name),
                "effective_percentage" => Deserialize::begin(&mut self.effective_percentage),
                "flat_amount" => Deserialize::begin(&mut self.flat_amount),
                "id" => Deserialize::begin(&mut self.id),
                "inclusive" => Deserialize::begin(&mut self.inclusive),
                "jurisdiction" => Deserialize::begin(&mut self.jurisdiction),
                "jurisdiction_level" => Deserialize::begin(&mut self.jurisdiction_level),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "percentage" => Deserialize::begin(&mut self.percentage),
                "rate_type" => Deserialize::begin(&mut self.rate_type),
                "state" => Deserialize::begin(&mut self.state),
                "tax_type" => Deserialize::begin(&mut self.tax_type),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                active: Deserialize::default(),
                country: Deserialize::default(),
                created: Deserialize::default(),
                description: Deserialize::default(),
                display_name: Deserialize::default(),
                effective_percentage: Deserialize::default(),
                flat_amount: Deserialize::default(),
                id: Deserialize::default(),
                inclusive: Deserialize::default(),
                jurisdiction: Deserialize::default(),
                jurisdiction_level: Deserialize::default(),
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
                percentage: Deserialize::default(),
                rate_type: Deserialize::default(),
                state: Deserialize::default(),
                tax_type: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(active),
                Some(country),
                Some(created),
                Some(description),
                Some(display_name),
                Some(effective_percentage),
                Some(flat_amount),
                Some(id),
                Some(inclusive),
                Some(jurisdiction),
                Some(jurisdiction_level),
                Some(livemode),
                Some(metadata),
                Some(percentage),
                Some(rate_type),
                Some(state),
                Some(tax_type),
            ) = (
                self.active,
                self.country.take(),
                self.created,
                self.description.take(),
                self.display_name.take(),
                self.effective_percentage,
                self.flat_amount.take(),
                self.id.take(),
                self.inclusive,
                self.jurisdiction.take(),
                self.jurisdiction_level,
                self.livemode,
                self.metadata.take(),
                self.percentage,
                self.rate_type,
                self.state.take(),
                self.tax_type.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                active,
                country,
                created,
                description,
                display_name,
                effective_percentage,
                flat_amount,
                id,
                inclusive,
                jurisdiction,
                jurisdiction_level,
                livemode,
                metadata,
                percentage,
                rate_type,
                state,
                tax_type,
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

    impl ObjectDeser for TaxRate {
        type Builder = TaxRateBuilder;
    }

    impl FromValueOpt for TaxRate {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TaxRateBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "active" => b.active = FromValueOpt::from_value(v),
                    "country" => b.country = FromValueOpt::from_value(v),
                    "created" => b.created = FromValueOpt::from_value(v),
                    "description" => b.description = FromValueOpt::from_value(v),
                    "display_name" => b.display_name = FromValueOpt::from_value(v),
                    "effective_percentage" => b.effective_percentage = FromValueOpt::from_value(v),
                    "flat_amount" => b.flat_amount = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "inclusive" => b.inclusive = FromValueOpt::from_value(v),
                    "jurisdiction" => b.jurisdiction = FromValueOpt::from_value(v),
                    "jurisdiction_level" => b.jurisdiction_level = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "metadata" => b.metadata = FromValueOpt::from_value(v),
                    "percentage" => b.percentage = FromValueOpt::from_value(v),
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
#[cfg(feature = "serialize")]
impl serde::Serialize for TaxRate {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("TaxRate", 18)?;
        s.serialize_field("active", &self.active)?;
        s.serialize_field("country", &self.country)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("description", &self.description)?;
        s.serialize_field("display_name", &self.display_name)?;
        s.serialize_field("effective_percentage", &self.effective_percentage)?;
        s.serialize_field("flat_amount", &self.flat_amount)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("inclusive", &self.inclusive)?;
        s.serialize_field("jurisdiction", &self.jurisdiction)?;
        s.serialize_field("jurisdiction_level", &self.jurisdiction_level)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("percentage", &self.percentage)?;
        s.serialize_field("rate_type", &self.rate_type)?;
        s.serialize_field("state", &self.state)?;
        s.serialize_field("tax_type", &self.tax_type)?;

        s.serialize_field("object", "tax_rate")?;
        s.end()
    }
}
/// The level of the jurisdiction that imposes this tax rate.
/// Will be `null` for manually defined tax rates.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxRateJurisdictionLevel {
    City,
    Country,
    County,
    District,
    Multiple,
    State,
}
impl TaxRateJurisdictionLevel {
    pub fn as_str(self) -> &'static str {
        use TaxRateJurisdictionLevel::*;
        match self {
            City => "city",
            Country => "country",
            County => "county",
            District => "district",
            Multiple => "multiple",
            State => "state",
        }
    }
}

impl std::str::FromStr for TaxRateJurisdictionLevel {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxRateJurisdictionLevel::*;
        match s {
            "city" => Ok(City),
            "country" => Ok(Country),
            "county" => Ok(County),
            "district" => Ok(District),
            "multiple" => Ok(Multiple),
            "state" => Ok(State),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for TaxRateJurisdictionLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxRateJurisdictionLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TaxRateJurisdictionLevel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TaxRateJurisdictionLevel {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<TaxRateJurisdictionLevel> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TaxRateJurisdictionLevel::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TaxRateJurisdictionLevel);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TaxRateJurisdictionLevel {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for TaxRateJurisdictionLevel"))
    }
}
/// Indicates the type of tax rate applied to the taxable amount.
/// This value can be `null` when no tax applies to the location.
/// This field is only present for TaxRates created by Stripe Tax.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxRateRateType {
    FlatAmount,
    Percentage,
}
impl TaxRateRateType {
    pub fn as_str(self) -> &'static str {
        use TaxRateRateType::*;
        match self {
            FlatAmount => "flat_amount",
            Percentage => "percentage",
        }
    }
}

impl std::str::FromStr for TaxRateRateType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxRateRateType::*;
        match s {
            "flat_amount" => Ok(FlatAmount),
            "percentage" => Ok(Percentage),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for TaxRateRateType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxRateRateType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TaxRateRateType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TaxRateRateType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<TaxRateRateType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TaxRateRateType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TaxRateRateType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TaxRateRateType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for TaxRateRateType"))
    }
}
impl stripe_types::Object for TaxRate {
    type Id = stripe_shared::TaxRateId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(TaxRateId);
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TaxRateTaxType {
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
impl TaxRateTaxType {
    pub fn as_str(&self) -> &str {
        use TaxRateTaxType::*;
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

impl std::str::FromStr for TaxRateTaxType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxRateTaxType::*;
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
impl std::fmt::Display for TaxRateTaxType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxRateTaxType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TaxRateTaxType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TaxRateTaxType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<TaxRateTaxType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TaxRateTaxType::from_str(s).unwrap());
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TaxRateTaxType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TaxRateTaxType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
