/// Tax rates can be applied to [invoices](https://stripe.com/docs/billing/invoices/tax-rates), [subscriptions](https://stripe.com/docs/billing/subscriptions/taxes) and [Checkout Sessions](https://stripe.com/docs/payments/checkout/set-up-a-subscription#tax-rates) to collect tax.
///
/// Related guide: [Tax rates](https://stripe.com/docs/billing/taxes/tax-rates)
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
    /// [ISO 3166-2 subdivision code](https://en.wikipedia.org/wiki/ISO_3166-2:US), without country prefix.
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
    id: Option<stripe_shared::TaxRateId>,
    inclusive: Option<bool>,
    jurisdiction: Option<Option<String>>,
    jurisdiction_level: Option<Option<TaxRateJurisdictionLevel>>,
    livemode: Option<bool>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    percentage: Option<f64>,
    state: Option<Option<String>>,
    tax_type: Option<Option<stripe_shared::TaxRateTaxType>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
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
                "id" => Deserialize::begin(&mut self.id),
                "inclusive" => Deserialize::begin(&mut self.inclusive),
                "jurisdiction" => Deserialize::begin(&mut self.jurisdiction),
                "jurisdiction_level" => Deserialize::begin(&mut self.jurisdiction_level),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "percentage" => Deserialize::begin(&mut self.percentage),
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
                id: Deserialize::default(),
                inclusive: Deserialize::default(),
                jurisdiction: Deserialize::default(),
                jurisdiction_level: Deserialize::default(),
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
                percentage: Deserialize::default(),
                state: Deserialize::default(),
                tax_type: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                active: self.active?,
                country: self.country.take()?,
                created: self.created?,
                description: self.description.take()?,
                display_name: self.display_name.take()?,
                effective_percentage: self.effective_percentage?,
                id: self.id.take()?,
                inclusive: self.inclusive?,
                jurisdiction: self.jurisdiction.take()?,
                jurisdiction_level: self.jurisdiction_level?,
                livemode: self.livemode?,
                metadata: self.metadata.take()?,
                percentage: self.percentage?,
                state: self.state.take()?,
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
                    "active" => b.active = Some(FromValueOpt::from_value(v)?),
                    "country" => b.country = Some(FromValueOpt::from_value(v)?),
                    "created" => b.created = Some(FromValueOpt::from_value(v)?),
                    "description" => b.description = Some(FromValueOpt::from_value(v)?),
                    "display_name" => b.display_name = Some(FromValueOpt::from_value(v)?),
                    "effective_percentage" => {
                        b.effective_percentage = Some(FromValueOpt::from_value(v)?)
                    }
                    "id" => b.id = Some(FromValueOpt::from_value(v)?),
                    "inclusive" => b.inclusive = Some(FromValueOpt::from_value(v)?),
                    "jurisdiction" => b.jurisdiction = Some(FromValueOpt::from_value(v)?),
                    "jurisdiction_level" => {
                        b.jurisdiction_level = Some(FromValueOpt::from_value(v)?)
                    }
                    "livemode" => b.livemode = Some(FromValueOpt::from_value(v)?),
                    "metadata" => b.metadata = Some(FromValueOpt::from_value(v)?),
                    "percentage" => b.percentage = Some(FromValueOpt::from_value(v)?),
                    "state" => b.state = Some(FromValueOpt::from_value(v)?),
                    "tax_type" => b.tax_type = Some(FromValueOpt::from_value(v)?),

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
        let mut s = s.serialize_struct("TaxRate", 16)?;
        s.serialize_field("active", &self.active)?;
        s.serialize_field("country", &self.country)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("description", &self.description)?;
        s.serialize_field("display_name", &self.display_name)?;
        s.serialize_field("effective_percentage", &self.effective_percentage)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("inclusive", &self.inclusive)?;
        s.serialize_field("jurisdiction", &self.jurisdiction)?;
        s.serialize_field("jurisdiction_level", &self.jurisdiction_level)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("percentage", &self.percentage)?;
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxRateJurisdictionLevel::*;
        match s {
            "city" => Ok(City),
            "country" => Ok(Country),
            "county" => Ok(County),
            "district" => Ok(District),
            "multiple" => Ok(Multiple),
            "state" => Ok(State),
            _ => Err(()),
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
#[derive(Copy, Clone, Eq, PartialEq)]
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
    Rst,
    SalesTax,
    ServiceTax,
    Vat,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl TaxRateTaxType {
    pub fn as_str(self) -> &'static str {
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
            Rst => "rst",
            SalesTax => "sales_tax",
            ServiceTax => "service_tax",
            Vat => "vat",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for TaxRateTaxType {
    type Err = ();
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
            "rst" => Ok(Rst),
            "sales_tax" => Ok(SalesTax),
            "service_tax" => Ok(ServiceTax),
            "vat" => Ok(Vat),
            _ => Err(()),
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
        self.out = Some(TaxRateTaxType::from_str(s).unwrap_or(TaxRateTaxType::Unknown));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TaxRateTaxType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TaxRateTaxType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap_or(Self::Unknown))
    }
}
