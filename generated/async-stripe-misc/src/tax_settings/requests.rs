use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
struct RetrieveForMyAccountTaxSettingsBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveForMyAccountTaxSettingsBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves Tax `Settings` for a merchant.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveForMyAccountTaxSettings {
    inner: RetrieveForMyAccountTaxSettingsBuilder,
}
impl RetrieveForMyAccountTaxSettings {
    /// Construct a new `RetrieveForMyAccountTaxSettings`.
    pub fn new() -> Self {
        Self { inner: RetrieveForMyAccountTaxSettingsBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl Default for RetrieveForMyAccountTaxSettings {
    fn default() -> Self {
        Self::new()
    }
}
impl RetrieveForMyAccountTaxSettings {
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

impl StripeRequest for RetrieveForMyAccountTaxSettings {
    type Output = stripe_misc::TaxSettings;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/tax/settings").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct UpdateTaxSettingsBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    defaults: Option<UpdateTaxSettingsDefaults>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    head_office: Option<UpdateTaxSettingsHeadOffice>,
}
impl UpdateTaxSettingsBuilder {
    fn new() -> Self {
        Self { defaults: None, expand: None, head_office: None }
    }
}
/// Default configuration to be used on Stripe Tax calculations.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateTaxSettingsDefaults {
    /// Specifies the default [tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#tax-behavior) to be used when the item's price has unspecified tax behavior.
    /// One of inclusive, exclusive, or inferred_by_currency.
    /// Once specified, it cannot be changed back to null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<UpdateTaxSettingsDefaultsTaxBehavior>,
    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
}
impl UpdateTaxSettingsDefaults {
    pub fn new() -> Self {
        Self { tax_behavior: None, tax_code: None }
    }
}
impl Default for UpdateTaxSettingsDefaults {
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
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateTaxSettingsHeadOffice {
    /// The location of the business for tax purposes.
    pub address: UpdateTaxSettingsHeadOfficeAddress,
}
impl UpdateTaxSettingsHeadOffice {
    pub fn new(address: impl Into<UpdateTaxSettingsHeadOfficeAddress>) -> Self {
        Self { address: address.into() }
    }
}
/// The location of the business for tax purposes.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateTaxSettingsHeadOfficeAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State/province as an [ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2) subdivision code, without country prefix.
    /// Example: "NY" or "TX".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl UpdateTaxSettingsHeadOfficeAddress {
    pub fn new() -> Self {
        Self { city: None, country: None, line1: None, line2: None, postal_code: None, state: None }
    }
}
impl Default for UpdateTaxSettingsHeadOfficeAddress {
    fn default() -> Self {
        Self::new()
    }
}
/// Updates Tax `Settings` parameters used in tax calculations.
/// All parameters are editable but none can be removed once set.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateTaxSettings {
    inner: UpdateTaxSettingsBuilder,
}
impl UpdateTaxSettings {
    /// Construct a new `UpdateTaxSettings`.
    pub fn new() -> Self {
        Self { inner: UpdateTaxSettingsBuilder::new() }
    }
    /// Default configuration to be used on Stripe Tax calculations.
    pub fn defaults(mut self, defaults: impl Into<UpdateTaxSettingsDefaults>) -> Self {
        self.inner.defaults = Some(defaults.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// The place where your business is located.
    pub fn head_office(mut self, head_office: impl Into<UpdateTaxSettingsHeadOffice>) -> Self {
        self.inner.head_office = Some(head_office.into());
        self
    }
}
impl Default for UpdateTaxSettings {
    fn default() -> Self {
        Self::new()
    }
}
impl UpdateTaxSettings {
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

impl StripeRequest for UpdateTaxSettings {
    type Output = stripe_misc::TaxSettings;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/tax/settings").form(&self.inner)
    }
}
