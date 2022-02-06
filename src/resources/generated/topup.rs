// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::config::{Client, Response};
use crate::params::{Expand, List, Metadata, Object, RangeQuery, Timestamp};
use crate::Topup;

impl Topup {
    /// Returns a list of top-ups.
    pub fn list(client: &Client, params: ListTopups<'_>) -> Response<List<Topup>> {
        client.get_query("/topups", &params)
    }

    /// Retrieves the details of a top-up that has previously been created.
    ///
    /// Supply the unique top-up ID that was returned from your previous request, and Stripe will return the corresponding top-up information.
    pub fn retrieve(client: &Client, id: &TopupId, expand: &[&str]) -> Response<Topup> {
        client.get_query(&format!("/topups/{}", id), &Expand { expand })
    }

    /// Updates the metadata of a top-up.
    ///
    /// Other top-up details are not editable by design.
    pub fn update(client: &Client, id: &TopupId, params: UpdateTopup<'_>) -> Response<Topup> {
        client.post_form(&format!("/topups/{}", id), &params)
    }
}

impl Object for Topup {
    type Id = TopupId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "topup"
    }
}

// written at 597
/// The parameters for `Topup::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListTopups<'a> {
    /// A positive integer representing how much to transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<RangeQuery<Timestamp>>,

    /// A filter on the list, based on the object `created` field.
    ///
    /// The value can be a string with an integer Unix timestamp, or it can be a dictionary with a number of different query options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<TopupId>,

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
    pub starting_after: Option<TopupId>,

    /// Only return top-ups that have the given status.
    ///
    /// One of `canceled`, `failed`, `pending` or `succeeded`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TopupStatus>,
}

impl<'a> ListTopups<'a> {
    pub fn new() -> Self {
        ListTopups {
            amount: Default::default(),
            created: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
            status: Default::default(),
        }
    }
}

// written at 597
/// The parameters for `Topup::update`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct UpdateTopup<'a> {
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,

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

impl<'a> UpdateTopup<'a> {
    pub fn new() -> Self {
        UpdateTopup {
            description: Default::default(),
            expand: Default::default(),
            metadata: Default::default(),
        }
    }
}

/// An enum representing the possible values of an `ListTopups`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TopupStatus {
    Canceled,
    Failed,
    Pending,
    Succeeded,
}

impl TopupStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            TopupStatus::Canceled => "canceled",
            TopupStatus::Failed => "failed",
            TopupStatus::Pending => "pending",
            TopupStatus::Succeeded => "succeeded",
        }
    }
}

impl AsRef<str> for TopupStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TopupStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
