// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::config::{Client, Response};
use crate::params::{Deleted, Expand, List, Metadata, Object, RangeQuery, Timestamp};
use crate::resources::Currency;
use crate::Coupon;

impl Coupon {
    /// Returns a list of your coupons.
    pub fn list(client: &Client, params: ListCoupons<'_>) -> Response<List<Coupon>> {
        client.get_query("/coupons", &params)
    }

    /// You can create coupons easily via the [coupon management](https://dashboard.stripe.com/coupons) page of the Stripe dashboard.
    ///
    /// Coupon creation is also accessible via the API if you need to create coupons on the fly.  A coupon has either a `percent_off` or an `amount_off` and `currency`.
    /// If you set an `amount_off`, that amount will be subtracted from any invoice’s subtotal.
    /// For example, an invoice with a subtotal of $100 will have a final total of $0 if a coupon with an `amount_off` of 20000 is applied to it and an invoice with a subtotal of $300 will have a final total of $100 if a coupon with an `amount_off` of 20000 is applied to it.
    pub fn create(client: &Client, params: CreateCoupon<'_>) -> Response<Coupon> {
        client.post_form("/coupons", &params)
    }

    /// Retrieves the coupon with the given ID.
    pub fn retrieve(client: &Client, id: &CouponId, expand: &[&str]) -> Response<Coupon> {
        client.get_query(&format!("/coupons/{}", id), &Expand { expand })
    }

    /// Updates the metadata of a coupon.
    ///
    /// Other coupon details (currency, duration, amount_off) are, by design, not editable.
    pub fn update(client: &Client, id: &CouponId, params: UpdateCoupon<'_>) -> Response<Coupon> {
        client.post_form(&format!("/coupons/{}", id), &params)
    }

    /// You can delete coupons via the [coupon management](https://dashboard.stripe.com/coupons) page of the Stripe dashboard.
    ///
    /// However, deleting a coupon does not affect any customers who have already applied the coupon; it means that new customers can’t redeem the coupon.
    /// You can also delete coupons via the API.
    pub fn delete(client: &Client, id: &CouponId) -> Response<Deleted<CouponId>> {
        client.delete(&format!("/coupons/{}", id))
    }
}

impl Object for Coupon {
    type Id = CouponId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "coupon"
    }
}

// written at 597
/// The parameters for `Coupon::create`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct CreateCoupon<'a> {
    /// A positive integer representing the amount to subtract from an invoice total (required if `percent_off` is not passed).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_off: Option<i64>,

    /// A hash containing directions for what this Coupon will apply discounts to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applies_to: Option<Box<CreateCouponAppliesTo>>,

    /// Three-letter [ISO code for the currency](https://stripe.com/docs/currencies) of the `amount_off` parameter (required if `amount_off` is passed).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// Specifies how long the discount will be in effect if used on a subscription.
    ///
    /// Can be `forever`, `once`, or `repeating`.
    /// Defaults to `once`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<CouponDuration>,

    /// Required only if `duration` is `repeating`, in which case it must be a positive integer that specifies the number of months the discount will be in effect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_months: Option<i64>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Unique string of your choice that will be used to identify this coupon when applying it to a customer.
    ///
    /// If you don't want to specify a particular code, you can leave the ID blank and we'll generate a random code for you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<&'a str>,

    /// A positive integer specifying the number of times the coupon can be redeemed before it's no longer valid.
    ///
    /// For example, you might have a 50% off coupon that the first 20 readers of your blog can use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_redemptions: Option<i64>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Name of the coupon displayed to customers on, for instance invoices, or receipts.
    ///
    /// By default the `id` is shown if `name` is not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,

    /// A positive float larger than 0, and smaller or equal to 100, that represents the discount the coupon will apply (required if `amount_off` is not passed).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_off: Option<f64>,

    /// Unix timestamp specifying the last time at which the coupon can be redeemed.
    ///
    /// After the redeem_by date, the coupon can no longer be applied to new customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redeem_by: Option<Timestamp>,
}

impl<'a> CreateCoupon<'a> {
    pub fn new() -> Self {
        CreateCoupon {
            amount_off: Default::default(),
            applies_to: Default::default(),
            currency: Default::default(),
            duration: Default::default(),
            duration_in_months: Default::default(),
            expand: Default::default(),
            id: Default::default(),
            max_redemptions: Default::default(),
            metadata: Default::default(),
            name: Default::default(),
            percent_off: Default::default(),
            redeem_by: Default::default(),
        }
    }
}

// written at 597
/// The parameters for `Coupon::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListCoupons<'a> {
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
    pub ending_before: Option<CouponId>,

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
    pub starting_after: Option<CouponId>,
}

impl<'a> ListCoupons<'a> {
    pub fn new() -> Self {
        ListCoupons {
            created: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
        }
    }
}

// written at 597
/// The parameters for `Coupon::update`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct UpdateCoupon<'a> {
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

    /// Name of the coupon displayed to customers on, for instance invoices, or receipts.
    ///
    /// By default the `id` is shown if `name` is not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
}

impl<'a> UpdateCoupon<'a> {
    pub fn new() -> Self {
        UpdateCoupon {
            expand: Default::default(),
            metadata: Default::default(),
            name: Default::default(),
        }
    }
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateCouponAppliesTo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub products: Option<Box<Vec<String>>>,
}

/// An enum representing the possible values of an `CreateCoupon`'s `duration` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CouponDuration {
    Forever,
    Once,
    Repeating,
}

impl CouponDuration {
    pub fn as_str(self) -> &'static str {
        match self {
            CouponDuration::Forever => "forever",
            CouponDuration::Once => "once",
            CouponDuration::Repeating => "repeating",
        }
    }
}

impl AsRef<str> for CouponDuration {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CouponDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
