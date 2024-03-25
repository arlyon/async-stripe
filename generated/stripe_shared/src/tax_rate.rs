/// Tax rates can be applied to [invoices](https://stripe.com/docs/billing/invoices/tax-rates), [subscriptions](https://stripe.com/docs/billing/subscriptions/taxes) and [Checkout Sessions](https://stripe.com/docs/payments/checkout/set-up-a-subscription#tax-rates) to collect tax.
///
/// Related guide: [Tax rates](https://stripe.com/docs/billing/taxes/tax-rates)
///
/// For more details see <<https://stripe.com/docs/api/tax_rates/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
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
#[cfg(feature = "min-ser")]
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
    livemode: Option<bool>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    percentage: Option<f64>,
    state: Option<Option<String>>,
    tax_type: Option<Option<stripe_shared::TaxRateTaxType>>,
}

#[cfg(feature = "min-ser")]
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
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
                percentage: Deserialize::default(),
                state: Deserialize::default(),
                tax_type: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let active = self.active.take()?;
            let country = self.country.take()?;
            let created = self.created.take()?;
            let description = self.description.take()?;
            let display_name = self.display_name.take()?;
            let effective_percentage = self.effective_percentage.take()?;
            let id = self.id.take()?;
            let inclusive = self.inclusive.take()?;
            let jurisdiction = self.jurisdiction.take()?;
            let livemode = self.livemode.take()?;
            let metadata = self.metadata.take()?;
            let percentage = self.percentage.take()?;
            let state = self.state.take()?;
            let tax_type = self.tax_type.take()?;

            Some(Self::Out { active, country, created, description, display_name, effective_percentage, id, inclusive, jurisdiction, livemode, metadata, percentage, state, tax_type })
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
                    "effective_percentage" => b.effective_percentage = Some(FromValueOpt::from_value(v)?),
                    "id" => b.id = Some(FromValueOpt::from_value(v)?),
                    "inclusive" => b.inclusive = Some(FromValueOpt::from_value(v)?),
                    "jurisdiction" => b.jurisdiction = Some(FromValueOpt::from_value(v)?),
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
impl stripe_types::Object for TaxRate {
    type Id = stripe_shared::TaxRateId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(TaxRateId, "txr_");
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
impl<'de> serde::Deserialize<'de> for TaxRateTaxType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap_or(Self::Unknown))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TaxRateTaxType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TaxRateTaxType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TaxRateTaxType::from_str(s).unwrap_or(TaxRateTaxType::Unknown));
        Ok(())
    }
}

#[cfg(feature = "min-ser")]
stripe_types::impl_from_val_with_from_str!(TaxRateTaxType);
