use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListIssuingPersonalizationDesignBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lookup_keys: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preferences: Option<ListIssuingPersonalizationDesignPreferences>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<stripe_shared::IssuingPersonalizationDesignStatus>,
}
impl<'a> ListIssuingPersonalizationDesignBuilder<'a> {
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
pub struct ListIssuingPersonalizationDesign<'a> {
    inner: ListIssuingPersonalizationDesignBuilder<'a>,
}
impl<'a> ListIssuingPersonalizationDesign<'a> {
    /// Construct a new `ListIssuingPersonalizationDesign`.
    pub fn new() -> Self {
        Self { inner: ListIssuingPersonalizationDesignBuilder::new() }
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: &'a str) -> Self {
        self.inner.ending_before = Some(ending_before);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: i64) -> Self {
        self.inner.limit = Some(limit);
        self
    }
    /// Only return personalization designs with the given lookup keys.
    pub fn lookup_keys(mut self, lookup_keys: &'a [&'a str]) -> Self {
        self.inner.lookup_keys = Some(lookup_keys);
        self
    }
    /// Only return personalization designs with the given preferences.
    pub fn preferences(mut self, preferences: ListIssuingPersonalizationDesignPreferences) -> Self {
        self.inner.preferences = Some(preferences);
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: &'a str) -> Self {
        self.inner.starting_after = Some(starting_after);
        self
    }
    /// Only return personalization designs with the given status.
    pub fn status(mut self, status: stripe_shared::IssuingPersonalizationDesignStatus) -> Self {
        self.inner.status = Some(status);
        self
    }
}
impl<'a> Default for ListIssuingPersonalizationDesign<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl ListIssuingPersonalizationDesign<'_> {
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
        stripe_client_core::ListPaginator::new_list("/issuing/personalization_designs", self.inner)
    }
}

impl StripeRequest for ListIssuingPersonalizationDesign<'_> {
    type Output = stripe_types::List<stripe_shared::IssuingPersonalizationDesign>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/issuing/personalization_designs")
            .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveIssuingPersonalizationDesignBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveIssuingPersonalizationDesignBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a personalization design object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveIssuingPersonalizationDesign<'a> {
    inner: RetrieveIssuingPersonalizationDesignBuilder<'a>,
    personalization_design: &'a stripe_shared::IssuingPersonalizationDesignId,
}
impl<'a> RetrieveIssuingPersonalizationDesign<'a> {
    /// Construct a new `RetrieveIssuingPersonalizationDesign`.
    pub fn new(personalization_design: &'a stripe_shared::IssuingPersonalizationDesignId) -> Self {
        Self { personalization_design, inner: RetrieveIssuingPersonalizationDesignBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveIssuingPersonalizationDesign<'_> {
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

impl StripeRequest for RetrieveIssuingPersonalizationDesign<'_> {
    type Output = stripe_shared::IssuingPersonalizationDesign;

    fn build(&self) -> RequestBuilder {
        let personalization_design = self.personalization_design;
        RequestBuilder::new(
            StripeMethod::Get,
            format!("/issuing/personalization_designs/{personalization_design}"),
        )
        .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreateIssuingPersonalizationDesignBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    card_logo: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    carrier_text: Option<CarrierTextParam<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lookup_key: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    physical_bundle: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    preferences: Option<PreferencesParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer_lookup_key: Option<bool>,
}
impl<'a> CreateIssuingPersonalizationDesignBuilder<'a> {
    fn new(physical_bundle: &'a str) -> Self {
        Self {
            card_logo: None,
            carrier_text: None,
            expand: None,
            lookup_key: None,
            metadata: None,
            name: None,
            physical_bundle,
            preferences: None,
            transfer_lookup_key: None,
        }
    }
}
/// Creates a personalization design object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateIssuingPersonalizationDesign<'a> {
    inner: CreateIssuingPersonalizationDesignBuilder<'a>,
}
impl<'a> CreateIssuingPersonalizationDesign<'a> {
    /// Construct a new `CreateIssuingPersonalizationDesign`.
    pub fn new(physical_bundle: &'a str) -> Self {
        Self { inner: CreateIssuingPersonalizationDesignBuilder::new(physical_bundle) }
    }
    /// The file for the card logo, for use with physical bundles that support card logos.
    /// Must have a `purpose` value of `issuing_logo`.
    pub fn card_logo(mut self, card_logo: &'a str) -> Self {
        self.inner.card_logo = Some(card_logo);
        self
    }
    /// Hash containing carrier text, for use with physical bundles that support carrier text.
    pub fn carrier_text(mut self, carrier_text: CarrierTextParam<'a>) -> Self {
        self.inner.carrier_text = Some(carrier_text);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// A lookup key used to retrieve personalization designs dynamically from a static string.
    /// This may be up to 200 characters.
    pub fn lookup_key(mut self, lookup_key: &'a str) -> Self {
        self.inner.lookup_key = Some(lookup_key);
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(mut self, metadata: &'a std::collections::HashMap<String, String>) -> Self {
        self.inner.metadata = Some(metadata);
        self
    }
    /// Friendly display name.
    pub fn name(mut self, name: &'a str) -> Self {
        self.inner.name = Some(name);
        self
    }
    /// Information on whether this personalization design is used to create cards when one is not specified.
    pub fn preferences(mut self, preferences: PreferencesParam) -> Self {
        self.inner.preferences = Some(preferences);
        self
    }
    /// If set to true, will atomically remove the lookup key from the existing personalization design, and assign it to this personalization design.
    pub fn transfer_lookup_key(mut self, transfer_lookup_key: bool) -> Self {
        self.inner.transfer_lookup_key = Some(transfer_lookup_key);
        self
    }
}
impl CreateIssuingPersonalizationDesign<'_> {
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

impl StripeRequest for CreateIssuingPersonalizationDesign<'_> {
    type Output = stripe_shared::IssuingPersonalizationDesign;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/issuing/personalization_designs")
            .form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct UpdateIssuingPersonalizationDesignBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    card_logo: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    carrier_text: Option<CarrierTextParam<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lookup_key: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    physical_bundle: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preferences: Option<PreferencesParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer_lookup_key: Option<bool>,
}
impl<'a> UpdateIssuingPersonalizationDesignBuilder<'a> {
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
pub struct UpdateIssuingPersonalizationDesign<'a> {
    inner: UpdateIssuingPersonalizationDesignBuilder<'a>,
    personalization_design: &'a stripe_shared::IssuingPersonalizationDesignId,
}
impl<'a> UpdateIssuingPersonalizationDesign<'a> {
    /// Construct a new `UpdateIssuingPersonalizationDesign`.
    pub fn new(personalization_design: &'a stripe_shared::IssuingPersonalizationDesignId) -> Self {
        Self { personalization_design, inner: UpdateIssuingPersonalizationDesignBuilder::new() }
    }
    /// The file for the card logo, for use with physical bundles that support card logos.
    /// Must have a `purpose` value of `issuing_logo`.
    pub fn card_logo(mut self, card_logo: &'a str) -> Self {
        self.inner.card_logo = Some(card_logo);
        self
    }
    /// Hash containing carrier text, for use with physical bundles that support carrier text.
    pub fn carrier_text(mut self, carrier_text: CarrierTextParam<'a>) -> Self {
        self.inner.carrier_text = Some(carrier_text);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// A lookup key used to retrieve personalization designs dynamically from a static string.
    /// This may be up to 200 characters.
    pub fn lookup_key(mut self, lookup_key: &'a str) -> Self {
        self.inner.lookup_key = Some(lookup_key);
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(mut self, metadata: &'a std::collections::HashMap<String, String>) -> Self {
        self.inner.metadata = Some(metadata);
        self
    }
    /// Friendly display name. Providing an empty string will set the field to null.
    pub fn name(mut self, name: &'a str) -> Self {
        self.inner.name = Some(name);
        self
    }
    /// The physical bundle object belonging to this personalization design.
    pub fn physical_bundle(mut self, physical_bundle: &'a str) -> Self {
        self.inner.physical_bundle = Some(physical_bundle);
        self
    }
    /// Information on whether this personalization design is used to create cards when one is not specified.
    pub fn preferences(mut self, preferences: PreferencesParam) -> Self {
        self.inner.preferences = Some(preferences);
        self
    }
    /// If set to true, will atomically remove the lookup key from the existing personalization design, and assign it to this personalization design.
    pub fn transfer_lookup_key(mut self, transfer_lookup_key: bool) -> Self {
        self.inner.transfer_lookup_key = Some(transfer_lookup_key);
        self
    }
}
impl UpdateIssuingPersonalizationDesign<'_> {
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

impl StripeRequest for UpdateIssuingPersonalizationDesign<'_> {
    type Output = stripe_shared::IssuingPersonalizationDesign;

    fn build(&self) -> RequestBuilder {
        let personalization_design = self.personalization_design;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/issuing/personalization_designs/{personalization_design}"),
        )
        .form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ActivateIssuingPersonalizationDesignBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> ActivateIssuingPersonalizationDesignBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Updates the `status` of the specified testmode personalization design object to `active`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ActivateIssuingPersonalizationDesign<'a> {
    inner: ActivateIssuingPersonalizationDesignBuilder<'a>,
    personalization_design: &'a str,
}
impl<'a> ActivateIssuingPersonalizationDesign<'a> {
    /// Construct a new `ActivateIssuingPersonalizationDesign`.
    pub fn new(personalization_design: &'a str) -> Self {
        Self { personalization_design, inner: ActivateIssuingPersonalizationDesignBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl ActivateIssuingPersonalizationDesign<'_> {
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

impl StripeRequest for ActivateIssuingPersonalizationDesign<'_> {
    type Output = stripe_shared::IssuingPersonalizationDesign;

    fn build(&self) -> RequestBuilder {
        let personalization_design = self.personalization_design;
        RequestBuilder::new(
            StripeMethod::Post,
            format!(
                "/test_helpers/issuing/personalization_designs/{personalization_design}/activate"
            ),
        )
        .form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct DeactivateIssuingPersonalizationDesignBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> DeactivateIssuingPersonalizationDesignBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Updates the `status` of the specified testmode personalization design object to `inactive`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct DeactivateIssuingPersonalizationDesign<'a> {
    inner: DeactivateIssuingPersonalizationDesignBuilder<'a>,
    personalization_design: &'a str,
}
impl<'a> DeactivateIssuingPersonalizationDesign<'a> {
    /// Construct a new `DeactivateIssuingPersonalizationDesign`.
    pub fn new(personalization_design: &'a str) -> Self {
        Self { personalization_design, inner: DeactivateIssuingPersonalizationDesignBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl DeactivateIssuingPersonalizationDesign<'_> {
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

impl StripeRequest for DeactivateIssuingPersonalizationDesign<'_> {
    type Output = stripe_shared::IssuingPersonalizationDesign;

    fn build(&self) -> RequestBuilder {
        let personalization_design = self.personalization_design;
        RequestBuilder::new(
            StripeMethod::Post,
            format!(
                "/test_helpers/issuing/personalization_designs/{personalization_design}/deactivate"
            ),
        )
        .form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RejectIssuingPersonalizationDesignBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    rejection_reasons: RejectIssuingPersonalizationDesignRejectionReasons<'a>,
}
impl<'a> RejectIssuingPersonalizationDesignBuilder<'a> {
    fn new(rejection_reasons: RejectIssuingPersonalizationDesignRejectionReasons<'a>) -> Self {
        Self { expand: None, rejection_reasons }
    }
}
/// The reason(s) the personalization design was rejected.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct RejectIssuingPersonalizationDesignRejectionReasons<'a> {
    /// The reason(s) the card logo was rejected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_logo: Option<&'a [RejectIssuingPersonalizationDesignRejectionReasonsCardLogo]>,
    /// The reason(s) the carrier text was rejected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier_text: Option<&'a [RejectIssuingPersonalizationDesignRejectionReasonsCarrierText]>,
}
impl<'a> RejectIssuingPersonalizationDesignRejectionReasons<'a> {
    pub fn new() -> Self {
        Self { card_logo: None, carrier_text: None }
    }
}
impl<'a> Default for RejectIssuingPersonalizationDesignRejectionReasons<'a> {
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
pub struct RejectIssuingPersonalizationDesign<'a> {
    inner: RejectIssuingPersonalizationDesignBuilder<'a>,
    personalization_design: &'a str,
}
impl<'a> RejectIssuingPersonalizationDesign<'a> {
    /// Construct a new `RejectIssuingPersonalizationDesign`.
    pub fn new(
        personalization_design: &'a str,
        rejection_reasons: RejectIssuingPersonalizationDesignRejectionReasons<'a>,
    ) -> Self {
        Self {
            personalization_design,
            inner: RejectIssuingPersonalizationDesignBuilder::new(rejection_reasons),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RejectIssuingPersonalizationDesign<'_> {
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

impl StripeRequest for RejectIssuingPersonalizationDesign<'_> {
    type Output = stripe_shared::IssuingPersonalizationDesign;

    fn build(&self) -> RequestBuilder {
        let personalization_design = self.personalization_design;
        RequestBuilder::new(
            StripeMethod::Post,
            format!(
                "/test_helpers/issuing/personalization_designs/{personalization_design}/reject"
            ),
        )
        .form(&self.inner)
    }
}

#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CarrierTextParam<'a> {
    /// The footer body text of the carrier letter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer_body: Option<&'a str>,
    /// The footer title text of the carrier letter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer_title: Option<&'a str>,
    /// The header body text of the carrier letter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_body: Option<&'a str>,
    /// The header title text of the carrier letter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_title: Option<&'a str>,
}
impl<'a> CarrierTextParam<'a> {
    pub fn new() -> Self {
        Self { footer_body: None, footer_title: None, header_body: None, header_title: None }
    }
}
impl<'a> Default for CarrierTextParam<'a> {
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
    pub fn new(is_default: bool) -> Self {
        Self { is_default }
    }
}
