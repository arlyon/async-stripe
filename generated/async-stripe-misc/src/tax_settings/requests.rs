use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct RetrieveForMyAccountTaxSettingsBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveForMyAccountTaxSettingsBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveForMyAccountTaxSettingsBuilder").finish_non_exhaustive()
    }
}
impl RetrieveForMyAccountTaxSettingsBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves Tax `Settings` for a merchant.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct RetrieveForMyAccountTaxSettings {
    inner: RetrieveForMyAccountTaxSettingsBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveForMyAccountTaxSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveForMyAccountTaxSettings").finish_non_exhaustive()
    }
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
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct UpdateTaxSettingsBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    defaults: Option<UpdateTaxSettingsDefaults>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    head_office: Option<UpdateTaxSettingsHeadOffice>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateTaxSettingsBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateTaxSettingsBuilder").finish_non_exhaustive()
    }
}
impl UpdateTaxSettingsBuilder {
    fn new() -> Self {
        Self { defaults: None, expand: None, head_office: None }
    }
}
/// Default configuration to be used on Stripe Tax calculations.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateTaxSettingsDefaults {
    /// Specifies the default [tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#tax-behavior) to be used when the item's price has unspecified tax behavior.
    /// One of inclusive, exclusive, or inferred_by_currency.
    /// Once specified, it cannot be changed back to null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<UpdateTaxSettingsDefaultsTaxBehavior>,
    /// A [tax code](https://docs.stripe.com/tax/tax-categories) ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateTaxSettingsDefaults {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateTaxSettingsDefaults").finish_non_exhaustive()
    }
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdateTaxSettingsDefaultsTaxBehavior {
    Exclusive,
    Inclusive,
    InferredByCurrency,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdateTaxSettingsDefaultsTaxBehavior {
    pub fn as_str(&self) -> &str {
        use UpdateTaxSettingsDefaultsTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            InferredByCurrency => "inferred_by_currency",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdateTaxSettingsDefaultsTaxBehavior {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateTaxSettingsDefaultsTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "inferred_by_currency" => Ok(InferredByCurrency),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdateTaxSettingsDefaultsTaxBehavior"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for UpdateTaxSettingsDefaultsTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for UpdateTaxSettingsDefaultsTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateTaxSettingsDefaultsTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(UpdateTaxSettingsDefaultsTaxBehavior)).finish_non_exhaustive()
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The place where your business is located.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateTaxSettingsHeadOffice {
    /// The location of the business for tax purposes.
    pub address: UpdateTaxSettingsHeadOfficeAddress,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateTaxSettingsHeadOffice {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateTaxSettingsHeadOffice").finish_non_exhaustive()
    }
}
impl UpdateTaxSettingsHeadOffice {
    pub fn new(address: impl Into<UpdateTaxSettingsHeadOfficeAddress>) -> Self {
        Self { address: address.into() }
    }
}
/// The location of the business for tax purposes.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateTaxSettingsHeadOfficeAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Address line 1, such as the street, PO Box, or company name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Address line 2, such as the apartment, suite, unit, or building.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State/province as an [ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2) subdivision code, without country prefix, such as "NY" or "TX".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateTaxSettingsHeadOfficeAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateTaxSettingsHeadOfficeAddress").finish_non_exhaustive()
    }
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
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateTaxSettings {
    inner: UpdateTaxSettingsBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateTaxSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateTaxSettings").finish_non_exhaustive()
    }
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
