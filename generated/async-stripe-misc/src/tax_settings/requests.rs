use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveForMyAccountTaxSettingsBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveForMyAccountTaxSettingsBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves Tax `Settings` for a merchant.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveForMyAccountTaxSettings<'a> {
    inner: RetrieveForMyAccountTaxSettingsBuilder<'a>,
}
impl<'a> RetrieveForMyAccountTaxSettings<'a> {
    /// Construct a new `RetrieveForMyAccountTaxSettings`.
    pub fn new() -> Self {
        Self { inner: RetrieveForMyAccountTaxSettingsBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl<'a> Default for RetrieveForMyAccountTaxSettings<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl RetrieveForMyAccountTaxSettings<'_> {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for RetrieveForMyAccountTaxSettings<'_> {
    type Output = stripe_misc::TaxSettings;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/tax/settings").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct UpdateTaxSettingsBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    defaults: Option<UpdateTaxSettingsDefaults<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    head_office: Option<UpdateTaxSettingsHeadOffice<'a>>,
}
impl<'a> UpdateTaxSettingsBuilder<'a> {
    fn new() -> Self {
        Self { defaults: None, expand: None, head_office: None }
    }
}
/// Default configuration to be used on Stripe Tax calculations.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateTaxSettingsDefaults<'a> {
    /// Specifies the default [tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#tax-behavior) to be used when the item's price has unspecified tax behavior.
    /// One of inclusive, exclusive, or inferred_by_currency.
    /// Once specified, it cannot be changed back to null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<UpdateTaxSettingsDefaultsTaxBehavior>,
    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<&'a str>,
}
impl<'a> UpdateTaxSettingsDefaults<'a> {
    pub fn new() -> Self {
        Self { tax_behavior: None, tax_code: None }
    }
}
impl<'a> Default for UpdateTaxSettingsDefaults<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Specifies the default [tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#tax-behavior) to be used when the item's price has unspecified tax behavior.
/// One of inclusive, exclusive, or inferred_by_currency.
/// Once specified, it cannot be changed back to null.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateTaxSettingsDefaultsTaxBehavior {
    Exclusive,
    Inclusive,
    InferredByCurrency,
}
impl UpdateTaxSettingsDefaultsTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use UpdateTaxSettingsDefaultsTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            InferredByCurrency => "inferred_by_currency",
        }
    }
}

impl std::str::FromStr for UpdateTaxSettingsDefaultsTaxBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateTaxSettingsDefaultsTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "inferred_by_currency" => Ok(InferredByCurrency),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateTaxSettingsDefaultsTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateTaxSettingsDefaultsTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateTaxSettingsDefaultsTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateTaxSettingsDefaultsTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for UpdateTaxSettingsDefaultsTaxBehavior")
        })
    }
}
/// The place where your business is located.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateTaxSettingsHeadOffice<'a> {
    /// The location of the business for tax purposes.
    pub address: UpdateTaxSettingsHeadOfficeAddress<'a>,
}
impl<'a> UpdateTaxSettingsHeadOffice<'a> {
    pub fn new(address: UpdateTaxSettingsHeadOfficeAddress<'a>) -> Self {
        Self { address }
    }
}
/// The location of the business for tax purposes.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateTaxSettingsHeadOfficeAddress<'a> {
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
    /// Example: "NY" or "TX".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
}
impl<'a> UpdateTaxSettingsHeadOfficeAddress<'a> {
    pub fn new() -> Self {
        Self { city: None, country: None, line1: None, line2: None, postal_code: None, state: None }
    }
}
impl<'a> Default for UpdateTaxSettingsHeadOfficeAddress<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Updates Tax `Settings` parameters used in tax calculations.
/// All parameters are editable but none can be removed once set.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateTaxSettings<'a> {
    inner: UpdateTaxSettingsBuilder<'a>,
}
impl<'a> UpdateTaxSettings<'a> {
    /// Construct a new `UpdateTaxSettings`.
    pub fn new() -> Self {
        Self { inner: UpdateTaxSettingsBuilder::new() }
    }
    /// Default configuration to be used on Stripe Tax calculations.
    pub fn defaults(mut self, defaults: UpdateTaxSettingsDefaults<'a>) -> Self {
        self.inner.defaults = Some(defaults);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// The place where your business is located.
    pub fn head_office(mut self, head_office: UpdateTaxSettingsHeadOffice<'a>) -> Self {
        self.inner.head_office = Some(head_office);
        self
    }
}
impl<'a> Default for UpdateTaxSettings<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl UpdateTaxSettings<'_> {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for UpdateTaxSettings<'_> {
    type Output = stripe_misc::TaxSettings;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/tax/settings").form(&self.inner)
    }
}
