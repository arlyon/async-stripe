// ======================================
// This file was automatically generated.
// ======================================

use crate::client::{Client, Response};
use crate::ids::{CouponId, CustomerId, PromotionCodeId};
use crate::params::{
    CurrencyMap, Expand, Expandable, List, Metadata, Object, Paginable, RangeQuery, Timestamp,
};
use crate::resources::{Coupon, Currency, Customer};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "PromotionCode".
///
/// For more details see <https://stripe.com/docs/api/promotion_codes/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PromotionCode {
    /// Unique identifier for the object.
    pub id: PromotionCodeId,

    /// Whether the promotion code is currently active.
    ///
    /// A promotion code is only active if the coupon is also valid.
    pub active: bool,

    /// The customer-facing code.
    ///
    /// Regardless of case, this code must be unique across all active promotion codes for each customer.
    pub code: String,

    pub coupon: Coupon,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// The customer that this promotion code can be used by.
    pub customer: Option<Expandable<Customer>>,

    /// Date at which the promotion code can no longer be redeemed.
    pub expires_at: Option<Timestamp>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Maximum number of times this promotion code can be redeemed.
    pub max_redemptions: Option<i64>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<Metadata>,

    pub restrictions: PromotionCodesResourceRestrictions,

    /// Number of times this promotion code has been used.
    pub times_redeemed: i64,
}

impl PromotionCode {
    /// Returns a list of your promotion codes.
    pub fn list(client: &Client, params: &ListPromotionCodes<'_>) -> Response<List<PromotionCode>> {
        client.get_query("/promotion_codes", params)
    }

    /// Retrieves the promotion code with the given ID.
    ///
    /// In order to retrieve a promotion code by the customer-facing `code` use [list](https://stripe.com/docs/api/promotion_codes/list) with the desired `code`.
    pub fn retrieve(
        client: &Client,
        id: &PromotionCodeId,
        expand: &[&str],
    ) -> Response<PromotionCode> {
        client.get_query(&format!("/promotion_codes/{}", id), Expand { expand })
    }

    /// Updates the specified promotion code by setting the values of the parameters passed.
    ///
    /// Most fields are, by design, not editable.
    pub fn update(
        client: &Client,
        id: &PromotionCodeId,
        params: UpdatePromotionCode<'_>,
    ) -> Response<PromotionCode> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form(&format!("/promotion_codes/{}", id), &params)
    }
}

impl Object for PromotionCode {
    type Id = PromotionCodeId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "promotion_code"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PromotionCodesResourceRestrictions {
    /// Promotion code restrictions defined in each available currency option.
    ///
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options: Option<CurrencyMap<PromotionCodeCurrencyOption>>,

    /// A Boolean indicating if the Promotion Code should only be redeemed for Customers without any successful payments or invoices.
    pub first_time_transaction: bool,

    /// Minimum amount required to redeem this Promotion Code into a Coupon (e.g., a purchase must be $100 or more to work).
    pub minimum_amount: Option<i64>,

    /// Three-letter [ISO code](https://stripe.com/docs/currencies) for minimum_amount.
    pub minimum_amount_currency: Option<Currency>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PromotionCodeCurrencyOption {
    /// Minimum amount required to redeem this Promotion Code into a Coupon (e.g., a purchase must be $100 or more to work).
    pub minimum_amount: i64,
}

/// The parameters for `PromotionCode::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListPromotionCodes<'a> {
    /// Filter promotion codes by whether they are active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    /// Only return promotion codes that have this case-insensitive code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<&'a str>,

    /// Only return promotion codes for this coupon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<CouponId>,

    /// A filter on the list, based on the object `created` field.
    ///
    /// The value can be a string with an integer Unix timestamp, or it can be a dictionary with a number of different query options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,

    /// Only return promotion codes that are restricted to this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<PromotionCodeId>,

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
    pub starting_after: Option<PromotionCodeId>,
}

impl<'a> ListPromotionCodes<'a> {
    pub fn new() -> Self {
        ListPromotionCodes {
            active: Default::default(),
            code: Default::default(),
            coupon: Default::default(),
            created: Default::default(),
            customer: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
        }
    }
}
impl Paginable for ListPromotionCodes<'_> {
    type O = PromotionCode;
    fn set_last(&mut self, item: Self::O) {
        self.starting_after = Some(item.id());
    }
}
/// The parameters for `PromotionCode::update`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct UpdatePromotionCode<'a> {
    /// Whether the promotion code is currently active.
    ///
    /// A promotion code can only be reactivated when the coupon is still valid and the promotion code is otherwise redeemable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

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

    /// Settings that restrict the redemption of the promotion code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<UpdatePromotionCodeRestrictions>,
}

impl<'a> UpdatePromotionCode<'a> {
    pub fn new() -> Self {
        UpdatePromotionCode {
            active: Default::default(),
            expand: Default::default(),
            metadata: Default::default(),
            restrictions: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePromotionCodeRestrictions {
    /// Promotion codes defined in each available currency option.
    ///
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options: Option<CurrencyMap<UpdatePromotionCodeRestrictionsCurrencyOptions>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePromotionCodeRestrictionsCurrencyOptions {
    /// Minimum amount required to redeem this Promotion Code into a Coupon (e.g., a purchase must be $100 or more to work).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_amount: Option<i64>,
}
