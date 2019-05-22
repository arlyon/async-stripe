// ======================================
// This file was automatically generated.
// ======================================

use crate::config::{Client, Response};
use crate::ids::TaxRateId;
use crate::params::{Expand, List, Metadata, Object, RangeQuery, Timestamp};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "TaxRate".
///
/// For more details see [https://stripe.com/docs/api/tax_rates/object](https://stripe.com/docs/api/tax_rates/object).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TaxRate {
    /// Unique identifier for the object.
    pub id: TaxRateId,

    /// Defaults to `true`.
    ///
    /// When set to `false`, this tax rate cannot be applied to objects in the API, but will still be applied to subscriptions and invoices that already have it set.
    pub active: bool,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// An arbitrary string attached to the tax rate for your internal use only.
    ///
    /// It will not be visible to your customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The display name of the tax rates as it will appear to your customer on their receipt email, PDF, and the hosted invoice page.
    pub display_name: String,

    /// This specifies if the tax rate is inclusive or exclusive.
    pub inclusive: bool,

    /// The jurisdiction for the tax rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jurisdiction: Option<String>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    /// This represents the tax rate percent out of 100.
    pub percentage: f64,
}

impl TaxRate {
    /// Returns a list of your tax rates.
    ///
    /// Tax rates are returned sorted by creation date, with the most recently created tax rates appearing first.
    pub fn list(client: &Client, params: TaxRateListParams<'_>) -> Response<List<TaxRate>> {
        client.get_query("/tax_rates", &params)
    }
}

impl Object for TaxRate {
    type Id = TaxRateId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "tax_rate"
    }
}

/// The parameters for `TaxRate::list`.
#[derive(Clone, Debug, Serialize)]
pub struct TaxRateListParams<'a> {
    /// Optional flag to filter by tax rates that are either active or not active (archived).
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,

    /// Optional range for filtering created date.
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<RangeQuery<Timestamp>>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a TaxRateId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    expand: &'a [&'a str],

    /// Optional flag to filter by tax rates that are inclusive (or those that are not inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    inclusive: Option<bool>,

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u64>,

    /// Optional range for tax rate percentage filtering.
    #[serde(skip_serializing_if = "Option::is_none")]
    percentage: Option<RangeQuery<Timestamp>>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a TaxRateId>,
}

impl<'a> TaxRateListParams<'a> {
    pub fn new() -> Self {
        TaxRateListParams {
            active: Default::default(),
            created: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            inclusive: Default::default(),
            limit: Default::default(),
            percentage: Default::default(),
            starting_after: Default::default(),
        }
    }
}
