// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::config::{Client, Response};
use crate::ids::ReviewId;
use crate::params::{Expand, Expandable, List, Object, RangeQuery, Timestamp};
use crate::resources::{Charge, PaymentIntent, ReviewReason};

/// The resource representing a Stripe "RadarReview".
///
/// For more details see <https://stripe.com/docs/api/reviews/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Review {
    /// Unique identifier for the object.
    pub id: ReviewId,

    /// The ZIP or postal code of the card used, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_zip: Option<Box<String>>,

    /// The charge associated with this review.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<Box<Expandable<Charge>>>,

    /// The reason the review was closed, or null if it has not yet been closed.
    ///
    /// One of `approved`, `refunded`, `refunded_as_fraud`, `disputed`, or `redacted`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_reason: Option<Box<ReviewClosedReason>>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// The IP address where the payment originated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<Box<String>>,

    /// Information related to the location of the payment.
    ///
    /// Note that this information is an approximation and attempts to locate the nearest population center - it should not be used to determine a specific address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_location: Option<Box<RadarReviewResourceLocation>>,

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
    pub payment_intent: Option<Box<Expandable<PaymentIntent>>>,

    /// The reason the review is currently open or closed.
    ///
    /// One of `rule`, `manual`, `approved`, `refunded`, `refunded_as_fraud`, `disputed`, or `redacted`.
    pub reason: ReviewReason,

    /// Information related to the browsing session of the user who initiated the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<Box<RadarReviewResourceSession>>,
}

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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RadarReviewResourceLocation {
    /// The city where the payment originated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<Box<String>>,

    /// Two-letter ISO code representing the country where the payment originated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Box<String>>,

    /// The geographic latitude where the payment originated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<Box<f64>>,

    /// The geographic longitude where the payment originated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<Box<f64>>,

    /// The state/county/province/region where the payment originated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<Box<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RadarReviewResourceSession {
    /// The browser used in this browser session (e.g., `Chrome`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser: Option<Box<String>>,

    /// Information about the device used for the browser session (e.g., `Samsung SM-G930T`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Box<String>>,

    /// The platform for the browser session (e.g., `Macintosh`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<Box<String>>,

    /// The version for the browser session (e.g., `61.0.3163.100`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<Box<String>>,
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
