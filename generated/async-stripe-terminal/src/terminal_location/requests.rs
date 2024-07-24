use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

/// Deletes a `Location` object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct DeleteTerminalLocation<'a> {
    location: &'a stripe_terminal::TerminalLocationId,
}
impl<'a> DeleteTerminalLocation<'a> {
    /// Construct a new `DeleteTerminalLocation`.
    pub fn new(location: &'a stripe_terminal::TerminalLocationId) -> Self {
        Self { location }
    }
}
impl DeleteTerminalLocation<'_> {
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

impl StripeRequest for DeleteTerminalLocation<'_> {
    type Output = stripe_terminal::DeletedTerminalLocation;

    fn build(&self) -> RequestBuilder {
        let location = self.location;
        RequestBuilder::new(StripeMethod::Delete, format!("/terminal/locations/{location}"))
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListTerminalLocationBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
}
impl<'a> ListTerminalLocationBuilder<'a> {
    fn new() -> Self {
        Self { ending_before: None, expand: None, limit: None, starting_after: None }
    }
}
/// Returns a list of `Location` objects.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListTerminalLocation<'a> {
    inner: ListTerminalLocationBuilder<'a>,
}
impl<'a> ListTerminalLocation<'a> {
    /// Construct a new `ListTerminalLocation`.
    pub fn new() -> Self {
        Self { inner: ListTerminalLocationBuilder::new() }
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
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: &'a str) -> Self {
        self.inner.starting_after = Some(starting_after);
        self
    }
}
impl<'a> Default for ListTerminalLocation<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl ListTerminalLocation<'_> {
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
        stripe_client_core::ListPaginator::new_list("/terminal/locations", self.inner)
    }
}

impl StripeRequest for ListTerminalLocation<'_> {
    type Output = stripe_types::List<stripe_terminal::TerminalLocation>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/terminal/locations").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveTerminalLocationBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveTerminalLocationBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a `Location` object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveTerminalLocation<'a> {
    inner: RetrieveTerminalLocationBuilder<'a>,
    location: &'a stripe_terminal::TerminalLocationId,
}
impl<'a> RetrieveTerminalLocation<'a> {
    /// Construct a new `RetrieveTerminalLocation`.
    pub fn new(location: &'a stripe_terminal::TerminalLocationId) -> Self {
        Self { location, inner: RetrieveTerminalLocationBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveTerminalLocation<'_> {
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

impl StripeRequest for RetrieveTerminalLocation<'_> {
    type Output = RetrieveTerminalLocationReturned;

    fn build(&self) -> RequestBuilder {
        let location = self.location;
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::MapBuilder;

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

    impl<'a> Map for Builder<'a> {
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

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreateTerminalLocationBuilder<'a> {
    address: CreateTerminalLocationAddress<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration_overrides: Option<&'a str>,
    display_name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
}
impl<'a> CreateTerminalLocationBuilder<'a> {
    fn new(address: CreateTerminalLocationAddress<'a>, display_name: &'a str) -> Self {
        Self { address, configuration_overrides: None, display_name, expand: None, metadata: None }
    }
}
/// The full address of the location.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTerminalLocationAddress<'a> {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: &'a str,
    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
}
impl<'a> CreateTerminalLocationAddress<'a> {
    pub fn new(country: &'a str) -> Self {
        Self { city: None, country, line1: None, line2: None, postal_code: None, state: None }
    }
}
/// Creates a new `Location` object.
/// For further details, including which address fields are required in each country, see the [Manage locations](https://stripe.com/docs/terminal/fleet/locations) guide.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTerminalLocation<'a> {
    inner: CreateTerminalLocationBuilder<'a>,
}
impl<'a> CreateTerminalLocation<'a> {
    /// Construct a new `CreateTerminalLocation`.
    pub fn new(address: CreateTerminalLocationAddress<'a>, display_name: &'a str) -> Self {
        Self { inner: CreateTerminalLocationBuilder::new(address, display_name) }
    }
    /// The ID of a configuration that will be used to customize all readers in this location.
    pub fn configuration_overrides(mut self, configuration_overrides: &'a str) -> Self {
        self.inner.configuration_overrides = Some(configuration_overrides);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
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
}
impl CreateTerminalLocation<'_> {
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

impl StripeRequest for CreateTerminalLocation<'_> {
    type Output = stripe_terminal::TerminalLocation;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/terminal/locations").form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct UpdateTerminalLocationBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<UpdateTerminalLocationAddress<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration_overrides: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
}
impl<'a> UpdateTerminalLocationBuilder<'a> {
    fn new() -> Self {
        Self {
            address: None,
            configuration_overrides: None,
            display_name: None,
            expand: None,
            metadata: None,
        }
    }
}
/// The full address of the location.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateTerminalLocationAddress<'a> {
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
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
}
impl<'a> UpdateTerminalLocationAddress<'a> {
    pub fn new() -> Self {
        Self { city: None, country: None, line1: None, line2: None, postal_code: None, state: None }
    }
}
impl<'a> Default for UpdateTerminalLocationAddress<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Updates a `Location` object by setting the values of the parameters passed.
/// Any parameters not provided will be left unchanged.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateTerminalLocation<'a> {
    inner: UpdateTerminalLocationBuilder<'a>,
    location: &'a stripe_terminal::TerminalLocationId,
}
impl<'a> UpdateTerminalLocation<'a> {
    /// Construct a new `UpdateTerminalLocation`.
    pub fn new(location: &'a stripe_terminal::TerminalLocationId) -> Self {
        Self { location, inner: UpdateTerminalLocationBuilder::new() }
    }
    /// The full address of the location.
    pub fn address(mut self, address: UpdateTerminalLocationAddress<'a>) -> Self {
        self.inner.address = Some(address);
        self
    }
    /// The ID of a configuration that will be used to customize all readers in this location.
    pub fn configuration_overrides(mut self, configuration_overrides: &'a str) -> Self {
        self.inner.configuration_overrides = Some(configuration_overrides);
        self
    }
    /// A name for the location.
    pub fn display_name(mut self, display_name: &'a str) -> Self {
        self.inner.display_name = Some(display_name);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
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
}
impl UpdateTerminalLocation<'_> {
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

impl StripeRequest for UpdateTerminalLocation<'_> {
    type Output = UpdateTerminalLocationReturned;

    fn build(&self) -> RequestBuilder {
        let location = self.location;
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::MapBuilder;

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

    impl<'a> Map for Builder<'a> {
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
