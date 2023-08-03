
/// Retrieves Tax `Settings` for a merchant.
pub fn retrieve_for_my_account(client: &stripe::Client, params: RetrieveForMyAccountTaxProductResourceTaxSettings) -> stripe::Response<stripe_misc::TaxProductResourceTaxSettings> {
    client.get_query("/tax/settings", params)
}
/// Updates Tax `Settings` parameters used in tax calculations.
///
/// All parameters are editable but none can be removed once set.
pub fn update(client: &stripe::Client, params: UpdateTaxProductResourceTaxSettings) -> stripe::Response<stripe_misc::TaxProductResourceTaxSettings> {
    client.send_form("/tax/settings", params, http_types::Method::Post)
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveForMyAccountTaxProductResourceTaxSettings<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveForMyAccountTaxProductResourceTaxSettings<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTaxProductResourceTaxSettings<'a> {
    /// Default configuration to be used on Stripe Tax calculations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaults: Option<UpdateTaxProductResourceTaxSettingsDefaults<'a>>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The place where your business is located.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_office: Option<UpdateTaxProductResourceTaxSettingsHeadOffice<'a>>,
}
impl<'a> UpdateTaxProductResourceTaxSettings<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Default configuration to be used on Stripe Tax calculations.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTaxProductResourceTaxSettingsDefaults<'a> {
    /// Specifies the default [tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#tax-behavior) to be used when the item's price has unspecified tax behavior.
    ///
    /// One of inclusive, exclusive, or inferred_by_currency.
    /// Once specified, it cannot be changed back to null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<UpdateTaxProductResourceTaxSettingsDefaultsTaxBehavior>,
    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<&'a str>,
}
impl<'a> UpdateTaxProductResourceTaxSettingsDefaults<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Specifies the default [tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#tax-behavior) to be used when the item's price has unspecified tax behavior.
///
/// One of inclusive, exclusive, or inferred_by_currency.
/// Once specified, it cannot be changed back to null.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateTaxProductResourceTaxSettingsDefaultsTaxBehavior {
    Exclusive,
    Inclusive,
    InferredByCurrency,
}

impl UpdateTaxProductResourceTaxSettingsDefaultsTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use UpdateTaxProductResourceTaxSettingsDefaultsTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            InferredByCurrency => "inferred_by_currency",
        }
    }
}

impl std::str::FromStr for UpdateTaxProductResourceTaxSettingsDefaultsTaxBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateTaxProductResourceTaxSettingsDefaultsTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "inferred_by_currency" => Ok(InferredByCurrency),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpdateTaxProductResourceTaxSettingsDefaultsTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateTaxProductResourceTaxSettingsDefaultsTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateTaxProductResourceTaxSettingsDefaultsTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateTaxProductResourceTaxSettingsDefaultsTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// The place where your business is located.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateTaxProductResourceTaxSettingsHeadOffice<'a> {
    /// The location of the business for tax purposes.
    pub address: UpdateTaxProductResourceTaxSettingsHeadOfficeAddress<'a>,
}
impl<'a> UpdateTaxProductResourceTaxSettingsHeadOffice<'a> {
    pub fn new(address: UpdateTaxProductResourceTaxSettingsHeadOfficeAddress<'a>) -> Self {
        Self { address }
    }
}
/// The location of the business for tax purposes.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTaxProductResourceTaxSettingsHeadOfficeAddress<'a> {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// State/province as an [ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2) subdivision code, without country prefix.
    ///
    /// Example: "NY" or "TX".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
}
impl<'a> UpdateTaxProductResourceTaxSettingsHeadOfficeAddress<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
