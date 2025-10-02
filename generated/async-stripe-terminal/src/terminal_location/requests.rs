use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

/// Deletes a `Location` object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct DeleteTerminalLocation {
    location: stripe_terminal::TerminalLocationId,
}
impl DeleteTerminalLocation {
    /// Construct a new `DeleteTerminalLocation`.
    pub fn new(location: impl Into<stripe_terminal::TerminalLocationId>) -> Self {
        Self { location: location.into() }
    }
}
impl DeleteTerminalLocation {
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

impl StripeRequest for DeleteTerminalLocation {
    type Output = stripe_terminal::DeletedTerminalLocation;

    fn build(&self) -> RequestBuilder {
        let location = &self.location;
        RequestBuilder::new(StripeMethod::Delete, format!("/terminal/locations/{location}"))
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct ListTerminalLocationBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
impl ListTerminalLocationBuilder {
    fn new() -> Self {
        Self { ending_before: None, expand: None, limit: None, starting_after: None }
    }
}
/// Returns a list of `Location` objects.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListTerminalLocation {
    inner: ListTerminalLocationBuilder,
}
impl ListTerminalLocation {
    /// Construct a new `ListTerminalLocation`.
    pub fn new() -> Self {
        Self { inner: ListTerminalLocationBuilder::new() }
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
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
}
impl Default for ListTerminalLocation {
    fn default() -> Self {
        Self::new()
    }
}
impl ListTerminalLocation {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_terminal::TerminalLocation>>
    {
        stripe_client_core::ListPaginator::new_list("/terminal/locations", &self.inner)
    }
}

impl StripeRequest for ListTerminalLocation {
    type Output = stripe_types::List<stripe_terminal::TerminalLocation>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/terminal/locations").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct RetrieveTerminalLocationBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveTerminalLocationBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a `Location` object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveTerminalLocation {
    inner: RetrieveTerminalLocationBuilder,
    location: stripe_terminal::TerminalLocationId,
}
impl RetrieveTerminalLocation {
    /// Construct a new `RetrieveTerminalLocation`.
    pub fn new(location: impl Into<stripe_terminal::TerminalLocationId>) -> Self {
        Self { location: location.into(), inner: RetrieveTerminalLocationBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveTerminalLocation {
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

impl StripeRequest for RetrieveTerminalLocation {
    type Output = RetrieveTerminalLocationReturned;

    fn build(&self) -> RequestBuilder {
        let location = &self.location;
        RequestBuilder::new(StripeMethod::Get, format!("/terminal/locations/{location}"))
            .query(&self.inner)
    }
}
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(untagged))]
pub enum RetrieveTerminalLocationReturned {
    TerminalLocation(stripe_terminal::TerminalLocation),
    DeletedTerminalLocation(stripe_terminal::DeletedTerminalLocation),
}

#[derive(Default)]
pub struct RetrieveTerminalLocationReturnedBuilder {
    inner: stripe_types::miniserde_helpers::MaybeDeletedBuilderInner,
}

const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::MapBuilder;
    use stripe_types::miniserde_helpers::FromValueOpt;

    use super::*;

    make_place!(Place);

    struct Builder<'a> {
        out: &'a mut Option<RetrieveTerminalLocationReturned>,
        builder: RetrieveTerminalLocationReturnedBuilder,
    }

    impl Deserialize for RetrieveTerminalLocationReturned {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    impl Visitor for Place<RetrieveTerminalLocationReturned> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: Default::default() }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl MapBuilder for RetrieveTerminalLocationReturnedBuilder {
        type Out = RetrieveTerminalLocationReturned;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.inner.key_inner(k)
        }

        fn deser_default() -> Self {
            Self::default()
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (deleted, o) = self.inner.finish_inner()?;
            Some(if deleted {
                RetrieveTerminalLocationReturned::DeletedTerminalLocation(FromValueOpt::from_value(
                    Value::Object(o),
                )?)
            } else {
                RetrieveTerminalLocationReturned::TerminalLocation(FromValueOpt::from_value(
                    Value::Object(o),
                )?)
            })
        }
    }

    impl stripe_types::ObjectDeser for RetrieveTerminalLocationReturned {
        type Builder = RetrieveTerminalLocationReturnedBuilder;
    }
};

#[derive(Clone, Debug, serde::Serialize)]
struct CreateTerminalLocationBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<CreateTerminalLocationAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_kana: Option<CreateTerminalLocationAddressKana>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_kanji: Option<CreateTerminalLocationAddressKanji>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration_overrides: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name_kana: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name_kanji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone: Option<String>,
}
impl CreateTerminalLocationBuilder {
    fn new() -> Self {
        Self {
            address: None,
            address_kana: None,
            address_kanji: None,
            configuration_overrides: None,
            display_name: None,
            display_name_kana: None,
            display_name_kanji: None,
            expand: None,
            metadata: None,
            phone: None,
        }
    }
}
/// The full address of the location.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTerminalLocationAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: String,
    /// Address line 1, such as the street, PO Box, or company name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Address line 2, such as the apartment, suite, unit, or building.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl CreateTerminalLocationAddress {
    pub fn new(country: impl Into<String>) -> Self {
        Self {
            city: None,
            country: country.into(),
            line1: None,
            line2: None,
            postal_code: None,
            state: None,
        }
    }
}
/// The Kana variation of the full address of the location (Japan only).
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTerminalLocationAddressKana {
    /// City or ward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Block or building number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Building details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// Prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Town or cho-me.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<String>,
}
impl CreateTerminalLocationAddressKana {
    pub fn new() -> Self {
        Self {
            city: None,
            country: None,
            line1: None,
            line2: None,
            postal_code: None,
            state: None,
            town: None,
        }
    }
}
impl Default for CreateTerminalLocationAddressKana {
    fn default() -> Self {
        Self::new()
    }
}
/// The Kanji variation of the full address of the location (Japan only).
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTerminalLocationAddressKanji {
    /// City or ward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Block or building number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Building details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// Prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Town or cho-me.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<String>,
}
impl CreateTerminalLocationAddressKanji {
    pub fn new() -> Self {
        Self {
            city: None,
            country: None,
            line1: None,
            line2: None,
            postal_code: None,
            state: None,
            town: None,
        }
    }
}
impl Default for CreateTerminalLocationAddressKanji {
    fn default() -> Self {
        Self::new()
    }
}
/// Creates a new `Location` object.
/// For further details, including which address fields are required in each country, see the [Manage locations](https://stripe.com/docs/terminal/fleet/locations) guide.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTerminalLocation {
    inner: CreateTerminalLocationBuilder,
}
impl CreateTerminalLocation {
    /// Construct a new `CreateTerminalLocation`.
    pub fn new() -> Self {
        Self { inner: CreateTerminalLocationBuilder::new() }
    }
    /// The full address of the location.
    pub fn address(mut self, address: impl Into<CreateTerminalLocationAddress>) -> Self {
        self.inner.address = Some(address.into());
        self
    }
    /// The Kana variation of the full address of the location (Japan only).
    pub fn address_kana(
        mut self,
        address_kana: impl Into<CreateTerminalLocationAddressKana>,
    ) -> Self {
        self.inner.address_kana = Some(address_kana.into());
        self
    }
    /// The Kanji variation of the full address of the location (Japan only).
    pub fn address_kanji(
        mut self,
        address_kanji: impl Into<CreateTerminalLocationAddressKanji>,
    ) -> Self {
        self.inner.address_kanji = Some(address_kanji.into());
        self
    }
    /// The ID of a configuration that will be used to customize all readers in this location.
    pub fn configuration_overrides(mut self, configuration_overrides: impl Into<String>) -> Self {
        self.inner.configuration_overrides = Some(configuration_overrides.into());
        self
    }
    /// A name for the location. Maximum length is 1000 characters.
    pub fn display_name(mut self, display_name: impl Into<String>) -> Self {
        self.inner.display_name = Some(display_name.into());
        self
    }
    /// The Kana variation of the name for the location (Japan only). Maximum length is 1000 characters.
    pub fn display_name_kana(mut self, display_name_kana: impl Into<String>) -> Self {
        self.inner.display_name_kana = Some(display_name_kana.into());
        self
    }
    /// The Kanji variation of the name for the location (Japan only). Maximum length is 1000 characters.
    pub fn display_name_kanji(mut self, display_name_kanji: impl Into<String>) -> Self {
        self.inner.display_name_kanji = Some(display_name_kanji.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
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
    /// The phone number for the location.
    pub fn phone(mut self, phone: impl Into<String>) -> Self {
        self.inner.phone = Some(phone.into());
        self
    }
}
impl Default for CreateTerminalLocation {
    fn default() -> Self {
        Self::new()
    }
}
impl CreateTerminalLocation {
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

impl StripeRequest for CreateTerminalLocation {
    type Output = stripe_terminal::TerminalLocation;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/terminal/locations").form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct UpdateTerminalLocationBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<UpdateTerminalLocationAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_kana: Option<UpdateTerminalLocationAddressKana>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_kanji: Option<UpdateTerminalLocationAddressKanji>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration_overrides: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name_kana: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name_kanji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone: Option<String>,
}
impl UpdateTerminalLocationBuilder {
    fn new() -> Self {
        Self {
            address: None,
            address_kana: None,
            address_kanji: None,
            configuration_overrides: None,
            display_name: None,
            display_name_kana: None,
            display_name_kanji: None,
            expand: None,
            metadata: None,
            phone: None,
        }
    }
}
/// The full address of the location.
/// You can't change the location's `country`.
/// If you need to modify the `country` field, create a new `Location` object and re-register any existing readers to that location.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateTerminalLocationAddress {
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
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl UpdateTerminalLocationAddress {
    pub fn new() -> Self {
        Self { city: None, country: None, line1: None, line2: None, postal_code: None, state: None }
    }
}
impl Default for UpdateTerminalLocationAddress {
    fn default() -> Self {
        Self::new()
    }
}
/// The Kana variation of the full address of the location (Japan only).
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateTerminalLocationAddressKana {
    /// City or ward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Block or building number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Building details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// Prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Town or cho-me.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<String>,
}
impl UpdateTerminalLocationAddressKana {
    pub fn new() -> Self {
        Self {
            city: None,
            country: None,
            line1: None,
            line2: None,
            postal_code: None,
            state: None,
            town: None,
        }
    }
}
impl Default for UpdateTerminalLocationAddressKana {
    fn default() -> Self {
        Self::new()
    }
}
/// The Kanji variation of the full address of the location (Japan only).
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateTerminalLocationAddressKanji {
    /// City or ward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Block or building number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Building details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// Prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Town or cho-me.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<String>,
}
impl UpdateTerminalLocationAddressKanji {
    pub fn new() -> Self {
        Self {
            city: None,
            country: None,
            line1: None,
            line2: None,
            postal_code: None,
            state: None,
            town: None,
        }
    }
}
impl Default for UpdateTerminalLocationAddressKanji {
    fn default() -> Self {
        Self::new()
    }
}
/// Updates a `Location` object by setting the values of the parameters passed.
/// Any parameters not provided will be left unchanged.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateTerminalLocation {
    inner: UpdateTerminalLocationBuilder,
    location: stripe_terminal::TerminalLocationId,
}
impl UpdateTerminalLocation {
    /// Construct a new `UpdateTerminalLocation`.
    pub fn new(location: impl Into<stripe_terminal::TerminalLocationId>) -> Self {
        Self { location: location.into(), inner: UpdateTerminalLocationBuilder::new() }
    }
    /// The full address of the location.
    /// You can't change the location's `country`.
    /// If you need to modify the `country` field, create a new `Location` object and re-register any existing readers to that location.
    pub fn address(mut self, address: impl Into<UpdateTerminalLocationAddress>) -> Self {
        self.inner.address = Some(address.into());
        self
    }
    /// The Kana variation of the full address of the location (Japan only).
    pub fn address_kana(
        mut self,
        address_kana: impl Into<UpdateTerminalLocationAddressKana>,
    ) -> Self {
        self.inner.address_kana = Some(address_kana.into());
        self
    }
    /// The Kanji variation of the full address of the location (Japan only).
    pub fn address_kanji(
        mut self,
        address_kanji: impl Into<UpdateTerminalLocationAddressKanji>,
    ) -> Self {
        self.inner.address_kanji = Some(address_kanji.into());
        self
    }
    /// The ID of a configuration that will be used to customize all readers in this location.
    pub fn configuration_overrides(mut self, configuration_overrides: impl Into<String>) -> Self {
        self.inner.configuration_overrides = Some(configuration_overrides.into());
        self
    }
    /// A name for the location.
    pub fn display_name(mut self, display_name: impl Into<String>) -> Self {
        self.inner.display_name = Some(display_name.into());
        self
    }
    /// The Kana variation of the name for the location (Japan only).
    pub fn display_name_kana(mut self, display_name_kana: impl Into<String>) -> Self {
        self.inner.display_name_kana = Some(display_name_kana.into());
        self
    }
    /// The Kanji variation of the name for the location (Japan only).
    pub fn display_name_kanji(mut self, display_name_kanji: impl Into<String>) -> Self {
        self.inner.display_name_kanji = Some(display_name_kanji.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
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
    /// The phone number for the location.
    pub fn phone(mut self, phone: impl Into<String>) -> Self {
        self.inner.phone = Some(phone.into());
        self
    }
}
impl UpdateTerminalLocation {
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

impl StripeRequest for UpdateTerminalLocation {
    type Output = UpdateTerminalLocationReturned;

    fn build(&self) -> RequestBuilder {
        let location = &self.location;
        RequestBuilder::new(StripeMethod::Post, format!("/terminal/locations/{location}"))
            .form(&self.inner)
    }
}
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(untagged))]
pub enum UpdateTerminalLocationReturned {
    TerminalLocation(stripe_terminal::TerminalLocation),
    DeletedTerminalLocation(stripe_terminal::DeletedTerminalLocation),
}

#[derive(Default)]
pub struct UpdateTerminalLocationReturnedBuilder {
    inner: stripe_types::miniserde_helpers::MaybeDeletedBuilderInner,
}

const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::MapBuilder;
    use stripe_types::miniserde_helpers::FromValueOpt;

    use super::*;

    make_place!(Place);

    struct Builder<'a> {
        out: &'a mut Option<UpdateTerminalLocationReturned>,
        builder: UpdateTerminalLocationReturnedBuilder,
    }

    impl Deserialize for UpdateTerminalLocationReturned {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    impl Visitor for Place<UpdateTerminalLocationReturned> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: Default::default() }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl MapBuilder for UpdateTerminalLocationReturnedBuilder {
        type Out = UpdateTerminalLocationReturned;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.inner.key_inner(k)
        }

        fn deser_default() -> Self {
            Self::default()
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (deleted, o) = self.inner.finish_inner()?;
            Some(if deleted {
                UpdateTerminalLocationReturned::DeletedTerminalLocation(FromValueOpt::from_value(
                    Value::Object(o),
                )?)
            } else {
                UpdateTerminalLocationReturned::TerminalLocation(FromValueOpt::from_value(
                    Value::Object(o),
                )?)
            })
        }
    }

    impl stripe_types::ObjectDeser for UpdateTerminalLocationReturned {
        type Builder = UpdateTerminalLocationReturnedBuilder;
    }
};
