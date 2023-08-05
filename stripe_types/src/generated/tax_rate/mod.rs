/// Tax rates can be applied to [invoices](https://stripe.com/docs/billing/invoices/tax-rates), [subscriptions](https://stripe.com/docs/billing/subscriptions/taxes) and [Checkout Sessions](https://stripe.com/docs/payments/checkout/set-up-a-subscription#tax-rates) to collect tax.
///
/// Related guide: [Tax rates](https://stripe.com/docs/billing/taxes/tax-rates).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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
    /// Actual/effective tax rate percentage out of 100.
    ///
    /// For tax calculations with automatic_tax[enabled]=true, this percentage does not include the statutory tax rate of non-taxable jurisdictions.
    pub effective_percentage: Option<f64>,
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
    /// Tax rate percentage out of 100.
    ///
    /// For tax calculations with automatic_tax[enabled]=true, this percentage includes the statutory tax rate of non-taxable jurisdictions.
    pub percentage: f64,
    /// [ISO 3166-2 subdivision code](https://en.wikipedia.org/wiki/ISO_3166-2:US), without country prefix.
    ///
    /// For example, "NY" for New York, United States.
    pub state: Option<String>,
    /// The high-level tax type, such as `vat` or `sales_tax`.
    pub tax_type: Option<TaxRateTaxType>,
}
/// The high-level tax type, such as `vat` or `sales_tax`.
#[derive(Copy, Clone, Eq, PartialEq)]
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

impl AsRef<str> for TaxRateTaxType {
    fn as_ref(&self) -> &str {
        self.as_str()
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
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TaxRateTaxType"))
    }
}
impl stripe_types::Object for TaxRate {
    type Id = stripe_types::tax_rate::TaxRateId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(TaxRateId, "txr_");
