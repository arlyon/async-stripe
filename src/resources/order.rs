// ======================================
// This file was automatically generated.
// ======================================

use crate::config::{Client, Response};
use crate::ids::{CustomerId, OrderId};
use crate::params::{Expand, Expandable, List, Metadata, Object, RangeQuery, Timestamp};
use crate::resources::{Charge, Currency, Customer, OrderItem, OrderReturn, Shipping};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "Order".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Order {
    /// Unique identifier for the object.
    pub id: OrderId,

    /// A positive integer in the smallest currency unit (that is, 100 cents for $1.00, or 1 for ¥1, Japanese Yen being a zero-decimal currency) representing the total amount for the order.
    pub amount: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_returned: Option<i64>,

    /// ID of the Connect Application that created the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee: Option<i64>,

    /// The ID of the payment used to pay for the order.
    ///
    /// Present if the order status is `paid`, `fulfilled`, or `refunded`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<Expandable<Charge>>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// The customer used for the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Expandable<Customer>>,

    /// The email address of the customer placing the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_coupon_code: Option<String>,

    /// List of items constituting the order.
    ///
    /// An order can have up to 25 items.
    pub items: Vec<OrderItem>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    #[serde(default)]
    pub returns: List<OrderReturn>,

    /// The shipping method that is currently selected for this order, if any.
    ///
    /// If present, it is equal to one of the `id`s of shipping methods in the `shipping_methods` array.
    /// At order creation time, if there are multiple shipping methods, Stripe will automatically selected the first method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_shipping_method: Option<String>,

    /// The shipping address for the order.
    ///
    /// Present if the order is for goods to be shipped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<Shipping>,

    /// A list of supported shipping methods for this order.
    ///
    /// The desired shipping method can be specified either by updating the order, or when paying it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_methods: Option<Vec<ShippingMethod>>,

    /// Current order status.
    ///
    /// One of `created`, `paid`, `canceled`, `fulfilled`, or `returned`.
    /// More details in the [Orders Guide](https://stripe.com/docs/orders/guide#understanding-order-statuses).
    pub status: String,

    /// The timestamps at which the order status was updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_transitions: Option<StatusTransitions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<Timestamp>,

    /// The user's order ID if it is different from the Stripe order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upstream_id: Option<String>,
}

impl Order {
    /// Returns a list of your orders.
    ///
    /// The orders are returned sorted by creation date, with the most recently created orders appearing first.
    pub fn list(client: &Client, params: OrderListParams<'_>) -> Response<List<Order>> {
        client.get_query("/orders", &params)
    }

    /// Retrieves the details of an existing order.
    ///
    /// Supply the unique order ID from either an order creation request or the order list, and Stripe will return the corresponding order information.
    pub fn retrieve(client: &Client, id: &OrderId, expand: &[&str]) -> Response<Order> {
        client.get_query(&format!("/orders/{}", id), &Expand { expand })
    }
}

impl Object for Order {
    type Id = OrderId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "order"
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ShippingMethod {
    /// A positive integer in the smallest currency unit (that is, 100 cents for $1.00, or 1 for ¥1, Japanese Yen being a zero-decimal currency) representing the total amount for the line item.
    pub amount: i64,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// The estimated delivery date for the given shipping method.
    ///
    /// Can be either a specific date or a range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_estimate: Option<DeliveryEstimate>,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: String,

    /// Unique identifier for the object.
    pub id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeliveryEstimate {
    /// If `type` is `"exact"`, `date` will be the expected delivery date in the format YYYY-MM-DD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,

    /// If `type` is `"range"`, `earliest` will be be the earliest delivery date in the format YYYY-MM-DD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub earliest: Option<String>,

    /// If `type` is `"range"`, `latest` will be the latest delivery date in the format YYYY-MM-DD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest: Option<String>,

    /// The type of estimate.
    ///
    /// Must be either `"range"` or `"exact"`.
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StatusTransitions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled: Option<Timestamp>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfiled: Option<Timestamp>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid: Option<Timestamp>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned: Option<Timestamp>,
}

/// The parameters for `Order::list`.
#[derive(Clone, Debug, Serialize)]
pub struct OrderListParams<'a> {
    /// Date this order was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<RangeQuery<Timestamp>>,

    /// Only return orders for the given customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<CustomerId>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a OrderId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u64>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a OrderId>,
}

impl<'a> OrderListParams<'a> {
    pub fn new() -> Self {
        OrderListParams {
            created: Default::default(),
            customer: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
        }
    }
}
