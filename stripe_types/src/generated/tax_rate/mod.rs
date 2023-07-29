/// Tax rates can be applied to [invoices](https://stripe.com/docs/billing/invoices/tax-rates), [subscriptions](https://stripe.com/docs/billing/subscriptions/taxes) and [Checkout Sessions](https://stripe.com/docs/payments/checkout/set-up-a-subscription#tax-rates) to collect tax.
///
/// Related guide: [Tax Rates](https://stripe.com/docs/billing/taxes/tax-rates).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TaxRate {
    /// Defaults to `true`.
    ///
    /// When set to `false`, this tax rate cannot be used with new applications or Checkout Sessions, but will still work for subscriptions and invoices that already have it set.
    pub active: bool,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: Option<String>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// An arbitrary string attached to the tax rate for your internal use only.
    ///
    /// It will not be visible to your customers.
    pub description: Option<String>,
    /// The display name of the tax rates as it will appear to your customer on their receipt email, PDF, and the hosted invoice page.
    pub display_name: String,
    /// Unique identifier for the object.
    pub id: stripe_types::tax_rate::TaxRateId,
    /// This specifies if the tax rate is inclusive or exclusive.
    pub inclusive: bool,
    /// The jurisdiction for the tax rate.
    ///
    /// You can use this label field for tax reporting purposes.
    /// It also appears on your customer’s invoice.
    pub jurisdiction: Option<String>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: TaxRateObject,
    /// This represents the tax rate percent out of 100.
    pub percentage: f64,
    /// [ISO 3166-2 subdivision code](https://en.wikipedia.org/wiki/ISO_3166-2:US), without country prefix.
    ///
    /// For example, "NY" for New York, United States.
    pub state: Option<String>,
    /// The high-level tax type, such as `vat` or `sales_tax`.
    pub tax_type: Option<TaxRateTaxType>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TaxRate {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TaxRateObject {
    TaxRate,
}

impl TaxRateObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TaxRate => "tax_rate",
        }
    }
}

impl std::str::FromStr for TaxRateObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "tax_rate" => Ok(Self::TaxRate),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for TaxRateObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxRateObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for TaxRateObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TaxRateObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TaxRateObject"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TaxRateObject {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TaxRateObject> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TaxRateObject::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// The high-level tax type, such as `vat` or `sales_tax`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TaxRateTaxType {
    Gst,
    Hst,
    Jct,
    Pst,
    Qst,
    Rst,
    SalesTax,
    Vat,
}

impl TaxRateTaxType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Gst => "gst",
            Self::Hst => "hst",
            Self::Jct => "jct",
            Self::Pst => "pst",
            Self::Qst => "qst",
            Self::Rst => "rst",
            Self::SalesTax => "sales_tax",
            Self::Vat => "vat",
        }
    }
}

impl std::str::FromStr for TaxRateTaxType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "gst" => Ok(Self::Gst),
            "hst" => Ok(Self::Hst),
            "jct" => Ok(Self::Jct),
            "pst" => Ok(Self::Pst),
            "qst" => Ok(Self::Qst),
            "rst" => Ok(Self::Rst),
            "sales_tax" => Ok(Self::SalesTax),
            "vat" => Ok(Self::Vat),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for TaxRateTaxType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxRateTaxType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TaxRateTaxType"))
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
        self.out = Some(TaxRateTaxType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
impl stripe_types::Object for TaxRate {
    type Id = stripe_types::tax_rate::TaxRateId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(TaxRateId, "txr_");
