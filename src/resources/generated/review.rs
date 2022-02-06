// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::config::{Client, Response};
use crate::params::{Expand, List, Object, RangeQuery, Timestamp};
use crate::Review;

impl Review {
    /// Returns a list of `Review` objects that have `open` set to `true`.
    ///
    /// The objects are sorted in descending order by creation date, with the most recently created object appearing first.
    pub fn list(client: &Client, params: ListReviews<'_>) -> Response<List<Review>> {
        client.get_query("/reviews", &params)
    }

    /// Retrieves a `Review` object.
    pub fn retrieve(client: &Client, id: &ReviewId, expand: &[&str]) -> Response<Review> {
        client.get_query(&format!("/reviews/{}", id), &Expand { expand })
    }
}

impl Object for Review {
    type Id = ReviewId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "review"
    }
}

// written at 597
/// The parameters for `Review::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListReviews<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<ReviewId>,

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
    pub starting_after: Option<ReviewId>,
}

impl<'a> ListReviews<'a> {
    pub fn new() -> Self {
        ListReviews {
            created: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
        }
    }
}
