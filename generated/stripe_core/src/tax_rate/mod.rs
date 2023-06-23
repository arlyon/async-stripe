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
    pub id: stripe_core::tax_rate::TaxRateId,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
/// The high-level tax type, such as `vat` or `sales_tax`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
impl stripe_types::Object for TaxRate {
    type Id = stripe_core::tax_rate::TaxRateId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(TaxRateId, "txr_");
pub mod requests;