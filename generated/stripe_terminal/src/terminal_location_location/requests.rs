#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveTerminalLocationLocation<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveTerminalLocationLocation<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveTerminalLocationLocation<'a> {
    /// Retrieves a `Location` object.
    pub fn send(
        &self,
        client: &stripe::Client,
        location: &stripe_terminal::terminal_location_location::TerminalLocationId,
    ) -> stripe::Response<RetrieveReturned> {
        client.get_query(&format!("/terminal/locations/{location}", location = location), self)
    }
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum RetrieveReturned {
    TerminalLocation(stripe_terminal::TerminalLocationLocation),
    DeletedTerminalLocation(stripe_terminal::TerminalLocationDeletedLocation),
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTerminalLocationLocation<'a> {
    /// The full address of the location.
    pub address: CreateTerminalLocationLocationAddress<'a>,
    /// The ID of a configuration that will be used to customize all readers in this location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_overrides: Option<&'a str>,
    /// A name for the location.
    pub display_name: &'a str,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
}
impl<'a> CreateTerminalLocationLocation<'a> {
    pub fn new(address: CreateTerminalLocationLocationAddress<'a>, display_name: &'a str) -> Self {
        Self {
            address,
            configuration_overrides: Default::default(),
            display_name,
            expand: Default::default(),
            metadata: Default::default(),
        }
    }
}
/// The full address of the location.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTerminalLocationLocationAddress<'a> {
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
impl<'a> CreateTerminalLocationLocationAddress<'a> {
    pub fn new(country: &'a str) -> Self {
        Self {
            city: Default::default(),
            country,
            line1: Default::default(),
            line2: Default::default(),
            postal_code: Default::default(),
            state: Default::default(),
        }
    }
}
impl<'a> CreateTerminalLocationLocation<'a> {
    /// Creates a new `Location` object.
    /// For further details, including which address fields are required in each country, see the [Manage locations](https://stripe.com/docs/terminal/fleet/locations) guide.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_terminal::TerminalLocationLocation> {
        client.send_form("/terminal/locations", self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTerminalLocationLocation<'a> {
    /// The full address of the location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<UpdateTerminalLocationLocationAddress<'a>>,
    /// The ID of a configuration that will be used to customize all readers in this location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_overrides: Option<&'a str>,
    /// A name for the location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
}
impl<'a> UpdateTerminalLocationLocation<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The full address of the location.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTerminalLocationLocationAddress<'a> {
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
impl<'a> UpdateTerminalLocationLocationAddress<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> UpdateTerminalLocationLocation<'a> {
    /// Updates a `Location` object by setting the values of the parameters passed.
    ///
    /// Any parameters not provided will be left unchanged.
    pub fn send(
        &self,
        client: &stripe::Client,
        location: &stripe_terminal::terminal_location_location::TerminalLocationId,
    ) -> stripe::Response<UpdateReturned> {
        client.send_form(
            &format!("/terminal/locations/{location}", location = location),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum UpdateReturned {
    TerminalLocation(stripe_terminal::TerminalLocationLocation),
    DeletedTerminalLocation(stripe_terminal::TerminalLocationDeletedLocation),
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListTerminalLocationLocation<'a> {
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}
impl<'a> ListTerminalLocationLocation<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> stripe::PaginationParams for ListTerminalLocationLocation<'a> {}
impl<'a> ListTerminalLocationLocation<'a> {
    /// Returns a list of `Location` objects.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_terminal::TerminalLocationLocation>> {
        client.get_query("/terminal/locations", self)
    }
    pub fn paginate(self) -> stripe::ListPaginator<stripe_terminal::TerminalLocationLocation> {
        stripe::ListPaginator::from_params("/terminal/locations", self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct DeleteTerminalLocationLocation {}
impl DeleteTerminalLocationLocation {
    pub fn new() -> Self {
        Self::default()
    }
}
impl DeleteTerminalLocationLocation {
    /// Deletes a `Location` object.
    pub fn send(
        &self,
        client: &stripe::Client,
        location: &stripe_terminal::terminal_location_location::TerminalLocationId,
    ) -> stripe::Response<stripe_terminal::TerminalLocationDeletedLocation> {
        client.send_form(
            &format!("/terminal/locations/{location}", location = location),
            self,
            http_types::Method::Delete,
        )
    }
}
