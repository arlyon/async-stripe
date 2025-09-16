use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
struct ListIssuingPersonalizationDesignBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lookup_keys: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preferences: Option<ListIssuingPersonalizationDesignPreferences>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<stripe_shared::IssuingPersonalizationDesignStatus>,
}
impl ListIssuingPersonalizationDesignBuilder {
    fn new() -> Self {
        Self {
            ending_before: None,
            expand: None,
            limit: None,
            lookup_keys: None,
            preferences: None,
            starting_after: None,
            status: None,
        }
    }
}
/// Only return personalization designs with the given preferences.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ListIssuingPersonalizationDesignPreferences {
    /// Only return the personalization design that's set as the default.
    /// A connected account uses the Connect platform's default design if no personalization design is set as the default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// Only return the personalization design that is set as the Connect platform's default.
    /// This parameter is only applicable to connected accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_platform_default: Option<bool>,
}
impl ListIssuingPersonalizationDesignPreferences {
    pub fn new() -> Self {
        Self { is_default: None, is_platform_default: None }
    }
}
impl Default for ListIssuingPersonalizationDesignPreferences {
    fn default() -> Self {
        Self::new()
    }
}
/// Returns a list of personalization design objects.
/// The objects are sorted in descending order by creation date, with the most recently created object appearing first.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListIssuingPersonalizationDesign {
    inner: ListIssuingPersonalizationDesignBuilder,
}
impl ListIssuingPersonalizationDesign {
    /// Construct a new `ListIssuingPersonalizationDesign`.
    pub fn new() -> Self {
        Self { inner: ListIssuingPersonalizationDesignBuilder::new() }
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: impl Into<String>) -> Self {
        self.inner.ending_before = Some(ending_before.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: impl Into<i64>) -> Self {
        self.inner.limit = Some(limit.into());
        self
    }
    /// Only return personalization designs with the given lookup keys.
    pub fn lookup_keys(mut self, lookup_keys: impl Into<Vec<String>>) -> Self {
        self.inner.lookup_keys = Some(lookup_keys.into());
        self
    }
    /// Only return personalization designs with the given preferences.
    pub fn preferences(
        mut self,
        preferences: impl Into<ListIssuingPersonalizationDesignPreferences>,
    ) -> Self {
        self.inner.preferences = Some(preferences.into());
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
    /// Only return personalization designs with the given status.
    pub fn status(
        mut self,
        status: impl Into<stripe_shared::IssuingPersonalizationDesignStatus>,
    ) -> Self {
        self.inner.status = Some(status.into());
        self
    }
}
impl Default for ListIssuingPersonalizationDesign {
    fn default() -> Self {
        Self::new()
    }
}
impl ListIssuingPersonalizationDesign {
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

    pub fn paginate(
        &self,
    ) -> stripe_client_core::ListPaginator<
        stripe_types::List<stripe_shared::IssuingPersonalizationDesign>,
    > {
        stripe_client_core::ListPaginator::new_list("/issuing/personalization_designs", &self.inner)
    }
}

impl StripeRequest for ListIssuingPersonalizationDesign {
    type Output = stripe_types::List<stripe_shared::IssuingPersonalizationDesign>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/issuing/personalization_designs")
            .query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct RetrieveIssuingPersonalizationDesignBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveIssuingPersonalizationDesignBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a personalization design object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveIssuingPersonalizationDesign {
    inner: RetrieveIssuingPersonalizationDesignBuilder,
    personalization_design: stripe_shared::IssuingPersonalizationDesignId,
}
impl RetrieveIssuingPersonalizationDesign {
    /// Construct a new `RetrieveIssuingPersonalizationDesign`.
    pub fn new(
        personalization_design: impl Into<stripe_shared::IssuingPersonalizationDesignId>,
    ) -> Self {
        Self {
            personalization_design: personalization_design.into(),
            inner: RetrieveIssuingPersonalizationDesignBuilder::new(),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveIssuingPersonalizationDesign {
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

impl StripeRequest for RetrieveIssuingPersonalizationDesign {
    type Output = stripe_shared::IssuingPersonalizationDesign;

    fn build(&self) -> RequestBuilder {
        let personalization_design = &self.personalization_design;
        RequestBuilder::new(
            StripeMethod::Get,
            format!("/issuing/personalization_designs/{personalization_design}"),
        )
        .query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct CreateIssuingPersonalizationDesignBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    card_logo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    carrier_text: Option<CarrierTextParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lookup_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    physical_bundle: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    preferences: Option<PreferencesParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer_lookup_key: Option<bool>,
}
impl CreateIssuingPersonalizationDesignBuilder {
    fn new(physical_bundle: impl Into<String>) -> Self {
        Self {
            card_logo: None,
            carrier_text: None,
            expand: None,
            lookup_key: None,
            metadata: None,
            name: None,
            physical_bundle: physical_bundle.into(),
            preferences: None,
            transfer_lookup_key: None,
        }
    }
}
/// Creates a personalization design object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateIssuingPersonalizationDesign {
    inner: CreateIssuingPersonalizationDesignBuilder,
}
impl CreateIssuingPersonalizationDesign {
    /// Construct a new `CreateIssuingPersonalizationDesign`.
    pub fn new(physical_bundle: impl Into<String>) -> Self {
        Self { inner: CreateIssuingPersonalizationDesignBuilder::new(physical_bundle.into()) }
    }
    /// The file for the card logo, for use with physical bundles that support card logos.
    /// Must have a `purpose` value of `issuing_logo`.
    pub fn card_logo(mut self, card_logo: impl Into<String>) -> Self {
        self.inner.card_logo = Some(card_logo.into());
        self
    }
    /// Hash containing carrier text, for use with physical bundles that support carrier text.
    pub fn carrier_text(mut self, carrier_text: impl Into<CarrierTextParam>) -> Self {
        self.inner.carrier_text = Some(carrier_text.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// A lookup key used to retrieve personalization designs dynamically from a static string.
    /// This may be up to 200 characters.
    pub fn lookup_key(mut self, lookup_key: impl Into<String>) -> Self {
        self.inner.lookup_key = Some(lookup_key.into());
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
    /// Friendly display name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.inner.name = Some(name.into());
        self
    }
    /// Information on whether this personalization design is used to create cards when one is not specified.
    pub fn preferences(mut self, preferences: impl Into<PreferencesParam>) -> Self {
        self.inner.preferences = Some(preferences.into());
        self
    }
    /// If set to true, will atomically remove the lookup key from the existing personalization design, and assign it to this personalization design.
    pub fn transfer_lookup_key(mut self, transfer_lookup_key: impl Into<bool>) -> Self {
        self.inner.transfer_lookup_key = Some(transfer_lookup_key.into());
        self
    }
}
impl CreateIssuingPersonalizationDesign {
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

impl StripeRequest for CreateIssuingPersonalizationDesign {
    type Output = stripe_shared::IssuingPersonalizationDesign;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/issuing/personalization_designs")
            .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct UpdateIssuingPersonalizationDesignBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    card_logo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    carrier_text: Option<CarrierTextParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lookup_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    physical_bundle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preferences: Option<PreferencesParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer_lookup_key: Option<bool>,
}
impl UpdateIssuingPersonalizationDesignBuilder {
    fn new() -> Self {
        Self {
            card_logo: None,
            carrier_text: None,
            expand: None,
            lookup_key: None,
            metadata: None,
            name: None,
            physical_bundle: None,
            preferences: None,
            transfer_lookup_key: None,
        }
    }
}
/// Updates a card personalization object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateIssuingPersonalizationDesign {
    inner: UpdateIssuingPersonalizationDesignBuilder,
    personalization_design: stripe_shared::IssuingPersonalizationDesignId,
}
impl UpdateIssuingPersonalizationDesign {
    /// Construct a new `UpdateIssuingPersonalizationDesign`.
    pub fn new(
        personalization_design: impl Into<stripe_shared::IssuingPersonalizationDesignId>,
    ) -> Self {
        Self {
            personalization_design: personalization_design.into(),
            inner: UpdateIssuingPersonalizationDesignBuilder::new(),
        }
    }
    /// The file for the card logo, for use with physical bundles that support card logos.
    /// Must have a `purpose` value of `issuing_logo`.
    pub fn card_logo(mut self, card_logo: impl Into<String>) -> Self {
        self.inner.card_logo = Some(card_logo.into());
        self
    }
    /// Hash containing carrier text, for use with physical bundles that support carrier text.
    pub fn carrier_text(mut self, carrier_text: impl Into<CarrierTextParam>) -> Self {
        self.inner.carrier_text = Some(carrier_text.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// A lookup key used to retrieve personalization designs dynamically from a static string.
    /// This may be up to 200 characters.
    pub fn lookup_key(mut self, lookup_key: impl Into<String>) -> Self {
        self.inner.lookup_key = Some(lookup_key.into());
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
    /// Friendly display name. Providing an empty string will set the field to null.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.inner.name = Some(name.into());
        self
    }
    /// The physical bundle object belonging to this personalization design.
    pub fn physical_bundle(mut self, physical_bundle: impl Into<String>) -> Self {
        self.inner.physical_bundle = Some(physical_bundle.into());
        self
    }
    /// Information on whether this personalization design is used to create cards when one is not specified.
    pub fn preferences(mut self, preferences: impl Into<PreferencesParam>) -> Self {
        self.inner.preferences = Some(preferences.into());
        self
    }
    /// If set to true, will atomically remove the lookup key from the existing personalization design, and assign it to this personalization design.
    pub fn transfer_lookup_key(mut self, transfer_lookup_key: impl Into<bool>) -> Self {
        self.inner.transfer_lookup_key = Some(transfer_lookup_key.into());
        self
    }
}
impl UpdateIssuingPersonalizationDesign {
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

impl StripeRequest for UpdateIssuingPersonalizationDesign {
    type Output = stripe_shared::IssuingPersonalizationDesign;

    fn build(&self) -> RequestBuilder {
        let personalization_design = &self.personalization_design;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/issuing/personalization_designs/{personalization_design}"),
        )
        .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct ActivateIssuingPersonalizationDesignBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl ActivateIssuingPersonalizationDesignBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Updates the `status` of the specified testmode personalization design object to `active`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ActivateIssuingPersonalizationDesign {
    inner: ActivateIssuingPersonalizationDesignBuilder,
    personalization_design: String,
}
impl ActivateIssuingPersonalizationDesign {
    /// Construct a new `ActivateIssuingPersonalizationDesign`.
    pub fn new(personalization_design: impl Into<String>) -> Self {
        Self {
            personalization_design: personalization_design.into(),
            inner: ActivateIssuingPersonalizationDesignBuilder::new(),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl ActivateIssuingPersonalizationDesign {
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

impl StripeRequest for ActivateIssuingPersonalizationDesign {
    type Output = stripe_shared::IssuingPersonalizationDesign;

    fn build(&self) -> RequestBuilder {
        let personalization_design = &self.personalization_design;
        RequestBuilder::new(
            StripeMethod::Post,
            format!(
                "/test_helpers/issuing/personalization_designs/{personalization_design}/activate"
            ),
        )
        .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct DeactivateIssuingPersonalizationDesignBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl DeactivateIssuingPersonalizationDesignBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Updates the `status` of the specified testmode personalization design object to `inactive`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct DeactivateIssuingPersonalizationDesign {
    inner: DeactivateIssuingPersonalizationDesignBuilder,
    personalization_design: String,
}
impl DeactivateIssuingPersonalizationDesign {
    /// Construct a new `DeactivateIssuingPersonalizationDesign`.
    pub fn new(personalization_design: impl Into<String>) -> Self {
        Self {
            personalization_design: personalization_design.into(),
            inner: DeactivateIssuingPersonalizationDesignBuilder::new(),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl DeactivateIssuingPersonalizationDesign {
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

impl StripeRequest for DeactivateIssuingPersonalizationDesign {
    type Output = stripe_shared::IssuingPersonalizationDesign;

    fn build(&self) -> RequestBuilder {
        let personalization_design = &self.personalization_design;
        RequestBuilder::new(
            StripeMethod::Post,
            format!(
                "/test_helpers/issuing/personalization_designs/{personalization_design}/deactivate"
            ),
        )
        .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct RejectIssuingPersonalizationDesignBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    rejection_reasons: RejectIssuingPersonalizationDesignRejectionReasons,
}
impl RejectIssuingPersonalizationDesignBuilder {
    fn new(
        rejection_reasons: impl Into<RejectIssuingPersonalizationDesignRejectionReasons>,
    ) -> Self {
        Self { expand: None, rejection_reasons: rejection_reasons.into() }
    }
}
/// The reason(s) the personalization design was rejected.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RejectIssuingPersonalizationDesignRejectionReasons {
    /// The reason(s) the card logo was rejected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_logo: Option<Vec<RejectIssuingPersonalizationDesignRejectionReasonsCardLogo>>,
    /// The reason(s) the carrier text was rejected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier_text: Option<Vec<RejectIssuingPersonalizationDesignRejectionReasonsCarrierText>>,
}
impl RejectIssuingPersonalizationDesignRejectionReasons {
    pub fn new() -> Self {
        Self { card_logo: None, carrier_text: None }
    }
}
impl Default for RejectIssuingPersonalizationDesignRejectionReasons {
    fn default() -> Self {
        Self::new()
    }
}
/// The reason(s) the card logo was rejected.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum RejectIssuingPersonalizationDesignRejectionReasonsCardLogo {
    GeographicLocation,
    Inappropriate,
    NetworkName,
    NonBinaryImage,
    NonFiatCurrency,
    Other,
    OtherEntity,
    PromotionalMaterial,
}
impl RejectIssuingPersonalizationDesignRejectionReasonsCardLogo {
    pub fn as_str(self) -> &'static str {
        use RejectIssuingPersonalizationDesignRejectionReasonsCardLogo::*;
        match self {
            GeographicLocation => "geographic_location",
            Inappropriate => "inappropriate",
            NetworkName => "network_name",
            NonBinaryImage => "non_binary_image",
            NonFiatCurrency => "non_fiat_currency",
            Other => "other",
            OtherEntity => "other_entity",
            PromotionalMaterial => "promotional_material",
        }
    }
}

impl std::str::FromStr for RejectIssuingPersonalizationDesignRejectionReasonsCardLogo {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use RejectIssuingPersonalizationDesignRejectionReasonsCardLogo::*;
        match s {
            "geographic_location" => Ok(GeographicLocation),
            "inappropriate" => Ok(Inappropriate),
            "network_name" => Ok(NetworkName),
            "non_binary_image" => Ok(NonBinaryImage),
            "non_fiat_currency" => Ok(NonFiatCurrency),
            "other" => Ok(Other),
            "other_entity" => Ok(OtherEntity),
            "promotional_material" => Ok(PromotionalMaterial),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for RejectIssuingPersonalizationDesignRejectionReasonsCardLogo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for RejectIssuingPersonalizationDesignRejectionReasonsCardLogo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for RejectIssuingPersonalizationDesignRejectionReasonsCardLogo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for RejectIssuingPersonalizationDesignRejectionReasonsCardLogo {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for RejectIssuingPersonalizationDesignRejectionReasonsCardLogo",
            )
        })
    }
}
/// The reason(s) the carrier text was rejected.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum RejectIssuingPersonalizationDesignRejectionReasonsCarrierText {
    GeographicLocation,
    Inappropriate,
    NetworkName,
    NonFiatCurrency,
    Other,
    OtherEntity,
    PromotionalMaterial,
}
impl RejectIssuingPersonalizationDesignRejectionReasonsCarrierText {
    pub fn as_str(self) -> &'static str {
        use RejectIssuingPersonalizationDesignRejectionReasonsCarrierText::*;
        match self {
            GeographicLocation => "geographic_location",
            Inappropriate => "inappropriate",
            NetworkName => "network_name",
            NonFiatCurrency => "non_fiat_currency",
            Other => "other",
            OtherEntity => "other_entity",
            PromotionalMaterial => "promotional_material",
        }
    }
}

impl std::str::FromStr for RejectIssuingPersonalizationDesignRejectionReasonsCarrierText {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use RejectIssuingPersonalizationDesignRejectionReasonsCarrierText::*;
        match s {
            "geographic_location" => Ok(GeographicLocation),
            "inappropriate" => Ok(Inappropriate),
            "network_name" => Ok(NetworkName),
            "non_fiat_currency" => Ok(NonFiatCurrency),
            "other" => Ok(Other),
            "other_entity" => Ok(OtherEntity),
            "promotional_material" => Ok(PromotionalMaterial),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for RejectIssuingPersonalizationDesignRejectionReasonsCarrierText {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for RejectIssuingPersonalizationDesignRejectionReasonsCarrierText {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for RejectIssuingPersonalizationDesignRejectionReasonsCarrierText {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for RejectIssuingPersonalizationDesignRejectionReasonsCarrierText
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for RejectIssuingPersonalizationDesignRejectionReasonsCarrierText",
            )
        })
    }
}
/// Updates the `status` of the specified testmode personalization design object to `rejected`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RejectIssuingPersonalizationDesign {
    inner: RejectIssuingPersonalizationDesignBuilder,
    personalization_design: String,
}
impl RejectIssuingPersonalizationDesign {
    /// Construct a new `RejectIssuingPersonalizationDesign`.
    pub fn new(
        personalization_design: impl Into<String>,
        rejection_reasons: impl Into<RejectIssuingPersonalizationDesignRejectionReasons>,
    ) -> Self {
        Self {
            personalization_design: personalization_design.into(),
            inner: RejectIssuingPersonalizationDesignBuilder::new(rejection_reasons.into()),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RejectIssuingPersonalizationDesign {
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

impl StripeRequest for RejectIssuingPersonalizationDesign {
    type Output = stripe_shared::IssuingPersonalizationDesign;

    fn build(&self) -> RequestBuilder {
        let personalization_design = &self.personalization_design;
        RequestBuilder::new(
            StripeMethod::Post,
            format!(
                "/test_helpers/issuing/personalization_designs/{personalization_design}/reject"
            ),
        )
        .form(&self.inner)
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct CarrierTextParam {
    /// The footer body text of the carrier letter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer_body: Option<String>,
    /// The footer title text of the carrier letter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer_title: Option<String>,
    /// The header body text of the carrier letter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_body: Option<String>,
    /// The header title text of the carrier letter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_title: Option<String>,
}
impl CarrierTextParam {
    pub fn new() -> Self {
        Self { footer_body: None, footer_title: None, header_body: None, header_title: None }
    }
}
impl Default for CarrierTextParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct PreferencesParam {
    /// Whether we use this personalization design to create cards when one isn't specified.
    /// A connected account uses the Connect platform's default design if no personalization design is set as the default design.
    pub is_default: bool,
}
impl PreferencesParam {
    pub fn new(is_default: impl Into<bool>) -> Self {
        Self { is_default: is_default.into() }
    }
}
