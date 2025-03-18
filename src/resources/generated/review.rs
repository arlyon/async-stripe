// ======================================
// This file was automatically generated.
// ======================================

use crate::client::{Client, Response};
use crate::ids::ReviewId;
use crate::params::{Expand, Expandable, List, Object, Paginable, RangeQuery, Timestamp};
use crate::resources::{Charge, PaymentIntent, ReviewReason};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "RadarReview".
///
/// For more details see <https://stripe.com/docs/api/radar/reviews/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Review {
    /// Unique identifier for the object.
    pub id: ReviewId,

    /// The ZIP or postal code of the card used, if applicable.
    pub billing_zip: Option<String>,

    /// The charge associated with this review.
    pub charge: Option<Expandable<Charge>>,

    /// The reason the review was closed, or null if it has not yet been closed.
    ///
    /// One of `approved`, `refunded`, `refunded_as_fraud`, `disputed`, or `redacted`.
    pub closed_reason: Option<ReviewClosedReason>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// The IP address where the payment originated.
    pub ip_address: Option<String>,

    /// Information related to the location of the payment.
    ///
    /// Note that this information is an approximation and attempts to locate the nearest population center - it should not be used to determine a specific address.
    pub ip_address_location: Option<RadarReviewResourceLocation>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// If `true`, the review needs action.
    pub open: bool,

    /// The reason the review was opened.
    ///
    /// One of `rule` or `manual`.
    pub opened_reason: ReviewOpenedReason,

    /// The PaymentIntent ID associated with this review, if one exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<Expandable<PaymentIntent>>,

    /// The reason the review is currently open or closed.
    ///
    /// One of `rule`, `manual`, `approved`, `refunded`, `refunded_as_fraud`, `disputed`, or `redacted`.
    pub reason: ReviewReason,

    /// Information related to the browsing session of the user who initiated the payment.
    pub session: Option<RadarReviewResourceSession>,
}

impl Review {
    /// Returns a list of `Review` objects that have `open` set to `true`.
    ///
    /// The objects are sorted in descending order by creation date, with the most recently created object appearing first.
    pub fn list(client: &Client, params: &ListReviews<'_>) -> Response<List<Review>> {
        client.get_query("/reviews", params)
    }

    /// Retrieves a `Review` object.
    pub fn retrieve(client: &Client, id: &ReviewId, expand: &[&str]) -> Response<Review> {
        client.get_query(&format!("/reviews/{}", id), Expand { expand })
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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RadarReviewResourceLocation {
    /// The city where the payment originated.
    pub city: Option<String>,

    /// Two-letter ISO code representing the country where the payment originated.
    pub country: Option<String>,

    /// The geographic latitude where the payment originated.
    pub latitude: Option<f64>,

    /// The geographic longitude where the payment originated.
    pub longitude: Option<f64>,

    /// The state/county/province/region where the payment originated.
    pub region: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RadarReviewResourceSession {
    /// The browser used in this browser session (e.g., `Chrome`).
    pub browser: Option<String>,

    /// Information about the device used for the browser session (e.g., `Samsung SM-G930T`).
    pub device: Option<String>,

    /// The platform for the browser session (e.g., `Macintosh`).
    pub platform: Option<String>,

    /// The version for the browser session (e.g., `61.0.3163.100`).
    pub version: Option<String>,
}

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
impl Paginable for ListReviews<'_> {
    type O = Review;
    fn set_last(&mut self, item: Self::O) {
        self.starting_after = Some(item.id());
    }
}
/// An enum representing the possible values of an `Review`'s `closed_reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ReviewClosedReason {
    Approved,
    Disputed,
    Redacted,
    Refunded,
    RefundedAsFraud,
}

impl ReviewClosedReason {
    pub fn as_str(self) -> &'static str {
        match self {
            ReviewClosedReason::Approved => "approved",
            ReviewClosedReason::Disputed => "disputed",
            ReviewClosedReason::Redacted => "redacted",
            ReviewClosedReason::Refunded => "refunded",
            ReviewClosedReason::RefundedAsFraud => "refunded_as_fraud",
        }
    }
}

impl AsRef<str> for ReviewClosedReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ReviewClosedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for ReviewClosedReason {
    fn default() -> Self {
        Self::Approved
    }
}

/// An enum representing the possible values of an `Review`'s `opened_reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ReviewOpenedReason {
    Manual,
    Rule,
}

impl ReviewOpenedReason {
    pub fn as_str(self) -> &'static str {
        match self {
            ReviewOpenedReason::Manual => "manual",
            ReviewOpenedReason::Rule => "rule",
        }
    }
}

impl AsRef<str> for ReviewOpenedReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ReviewOpenedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for ReviewOpenedReason {
    fn default() -> Self {
        Self::Manual
    }
}
