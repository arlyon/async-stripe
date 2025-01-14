// ======================================
// This file was automatically generated.
// ======================================

use crate::client::{Client, Response};
use crate::ids::TerminalLocationId;
use crate::params::{Expand, List, Metadata, Object, Paginable};
use crate::resources::Address;
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "TerminalLocationLocation".
///
/// For more details see <https://stripe.com/docs/api/terminal/locations/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TerminalLocation {
    /// Unique identifier for the object.
    pub id: TerminalLocationId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    /// The ID of a configuration that will be used to customize all readers in this location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_overrides: Option<String>,

    // Always true for a deleted object
    #[serde(default)]
    pub deleted: bool,

    /// The display name of the location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub livemode: Option<bool>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

impl TerminalLocation {
    /// Returns a list of `Location` objects.
    pub fn list(
        client: &Client,
        params: &ListTerminalLocations<'_>,
    ) -> Response<List<TerminalLocation>> {
        client.get_query("/terminal/locations", params)
    }

    /// Creates a new `Location` object.
    /// For further details, including which address fields are required in each country, see the [Manage locations](https://stripe.com/docs/terminal/fleet/locations) guide.
    pub fn create(
        client: &Client,
        params: CreateTerminalLocation<'_>,
    ) -> Response<TerminalLocation> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form("/terminal/locations", &params)
    }
}

impl Object for TerminalLocation {
    type Id = TerminalLocationId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "terminal.location"
    }
}

/// The parameters for `TerminalLocation::create`.
#[derive(Clone, Debug, Serialize)]
pub struct CreateTerminalLocation<'a> {
    /// The full address of the location.
    pub address: CreateTerminalLocationAddress,

    /// The ID of a configuration that will be used to customize all readers in this location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_overrides: Option<&'a str>,

    /// A name for the location.
    pub display_name: &'a str,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

impl<'a> CreateTerminalLocation<'a> {
    pub fn new(address: CreateTerminalLocationAddress, display_name: &'a str) -> Self {
        CreateTerminalLocation {
            address,
            configuration_overrides: Default::default(),
            display_name,
            expand: Default::default(),
            metadata: Default::default(),
        }
    }
}

/// The parameters for `TerminalLocation::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListTerminalLocations<'a> {
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<TerminalLocationId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<TerminalLocationId>,
}

impl<'a> ListTerminalLocations<'a> {
    pub fn new() -> Self {
        ListTerminalLocations {
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
        }
    }
}
impl Paginable for ListTerminalLocations<'_> {
    type O = TerminalLocation;
    fn set_last(&mut self, item: Self::O) {
        self.starting_after = Some(item.id());
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTerminalLocationAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: String,

    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,

    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,

    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
