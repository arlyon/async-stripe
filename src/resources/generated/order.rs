// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::config::{Client, Response};
use crate::ids::{CouponId, CustomerId, OrderId};
use crate::params::{Expand, Expandable, List, Metadata, Object, RangeQuery, Timestamp};
use crate::resources::{
    Charge, Currency, Customer, OrderItem, OrderReturn, OrderStatusFilter, Shipping,
};

/// The resource representing a Stripe "Order".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Order {
    /// Unique identifier for the object.
    pub id: OrderId,

    /// A positive integer in the smallest currency unit (that is, 100 cents for $1.00, or 1 for ¥1, Japanese Yen being a zero-decimal currency) representing the total amount for the order.
    pub amount: i64,

    /// The total amount that was returned to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_returned: Option<Box<i64>>,

    /// ID of the Connect Application that created the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<Box<String>>,

    /// A fee in cents that will be applied to the order and transferred to the application owner’s Stripe account.
    ///
    /// The request must be made with an OAuth key or the Stripe-Account header in order to take an application fee.
    /// For more information, see the application fees documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee: Option<Box<i64>>,

    /// The ID of the payment used to pay for the order.
    ///
    /// Present if the order status is `paid`, `fulfilled`, or `refunded`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<Box<Expandable<Charge>>>,

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
    pub customer: Option<Box<Expandable<Customer>>>,

    /// The email address of the customer placing the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Box<String>>,

    /// External coupon code to load for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_coupon_code: Option<Box<String>>,

    /// List of items constituting the order.
    ///
    /// An order can have up to 25 items.
    pub items: Vec<OrderItem>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(default)]
    pub metadata: Metadata,

    /// A list of returns that have taken place for this order.
    #[serde(default)]
    pub returns: List<OrderReturn>,

    /// The shipping method that is currently selected for this order, if any.
    ///
    /// If present, it is equal to one of the `id`s of shipping methods in the `shipping_methods` array.
    /// At order creation time, if there are multiple shipping methods, Stripe will automatically selected the first method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_shipping_method: Option<Box<String>>,

    /// The shipping address for the order.
    ///
    /// Present if the order is for goods to be shipped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<Box<Shipping>>,

    /// A list of supported shipping methods for this order.
    ///
    /// The desired shipping method can be specified either by updating the order, or when paying it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_methods: Option<Box<Vec<ShippingMethod>>>,

    /// Current order status.
    ///
    /// One of `created`, `paid`, `canceled`, `fulfilled`, or `returned`.
    /// More details in the [Orders Guide](https://stripe.com/docs/orders/guide#understanding-order-statuses).
    pub status: OrderStatus,

    /// The timestamps at which the order status was updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_transitions: Option<Box<StatusTransitions>>,

    /// Time at which the object was last updated.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<Box<Timestamp>>,

    /// The user's order ID if it is different from the Stripe order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upstream_id: Option<Box<String>>,
}

impl Order {
    /// Returns a list of your orders.
    ///
    /// The orders are returned sorted by creation date, with the most recently created orders appearing first.
    pub fn list(client: &Client, params: ListOrders<'_>) -> Response<List<Order>> {
        client.get_query("/orders", &params)
    }

    /// Creates a new order object.
    pub fn create(client: &Client, params: CreateOrder<'_>) -> Response<Order> {
        client.post_form("/orders", &params)
    }

    /// Retrieves the details of an existing order.
    ///
    /// Supply the unique order ID from either an order creation request or the order list, and Stripe will return the corresponding order information.
    pub fn retrieve(client: &Client, id: &OrderId, expand: &[&str]) -> Response<Order> {
        client.get_query(&format!("/orders/{}", id), &Expand { expand })
    }

    /// Updates the specific order by setting the values of the parameters passed.
    ///
    /// Any parameters not provided will be left unchanged.
    pub fn update(client: &Client, id: &OrderId, params: UpdateOrder<'_>) -> Response<Order> {
        client.post_form(&format!("/orders/{}", id), &params)
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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
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
    pub delivery_estimate: Option<Box<DeliveryEstimate>>,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: String,

    /// Unique identifier for the object.
    pub id: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DeliveryEstimate {
    /// If `type` is `"exact"`, `date` will be the expected delivery date in the format YYYY-MM-DD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Box<String>>,

    /// If `type` is `"range"`, `earliest` will be be the earliest delivery date in the format YYYY-MM-DD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub earliest: Option<Box<String>>,

    /// If `type` is `"range"`, `latest` will be the latest delivery date in the format YYYY-MM-DD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest: Option<Box<String>>,

    /// The type of estimate.
    ///
    /// Must be either `"range"` or `"exact"`.
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct StatusTransitions {
    /// The time that the order was canceled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled: Option<Box<Timestamp>>,

    /// The time that the order was fulfilled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfiled: Option<Box<Timestamp>>,

    /// The time that the order was paid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid: Option<Box<Timestamp>>,

    /// The time that the order was returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned: Option<Box<Timestamp>>,
}

/// The parameters for `Order::create`.
#[derive(Clone, Debug, Serialize)]
pub struct CreateOrder<'a> {
    /// A coupon code that represents a discount to be applied to this order.
    ///
    /// Must be one-time duration and in same currency as the order.
    /// An order can have multiple coupons.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<CouponId>,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// The ID of an existing customer to use for this order.
    ///
    /// If provided, the customer email and shipping address will be used to create the order.
    /// Subsequently, the customer will also be charged to pay the order.
    /// If `email` or `shipping` are also provided, they will override the values retrieved from the customer object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// The email address of the customer placing the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// List of items constituting the order.
    ///
    /// An order can have up to 25 items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<Vec<OrderItemParams>>>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Shipping address for the order.
    ///
    /// Required if any of the SKUs are for products that have `shippable` set to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<Box<CreateOrderShipping>>,
}

impl<'a> CreateOrder<'a> {
    pub fn new(currency: Currency) -> Self {
        CreateOrder {
            coupon: Default::default(),
            currency,
            customer: Default::default(),
            email: Default::default(),
            expand: Default::default(),
            items: Default::default(),
            metadata: Default::default(),
            shipping: Default::default(),
        }
    }
}

/// The parameters for `Order::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListOrders<'a> {
    /// Date this order was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,

    /// Only return orders for the given customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<OrderId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Only return orders with the given IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Box<Vec<String>>>,

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
    pub starting_after: Option<OrderId>,

    /// Only return orders that have the given status.
    ///
    /// One of `created`, `paid`, `fulfilled`, or `refunded`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OrderStatusFilter>,

    /// Filter orders based on when they were paid, fulfilled, canceled, or returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_transitions: Option<Box<ListOrdersStatusTransitions>>,

    /// Only return orders with the given upstream order IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upstream_ids: Option<Box<Vec<String>>>,
}

impl<'a> ListOrders<'a> {
    pub fn new() -> Self {
        ListOrders {
            created: Default::default(),
            customer: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            ids: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
            status: Default::default(),
            status_transitions: Default::default(),
            upstream_ids: Default::default(),
        }
    }
}

/// The parameters for `Order::update`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct UpdateOrder<'a> {
    /// A coupon code that represents a discount to be applied to this order.
    ///
    /// Must be one-time duration and in same currency as the order.
    /// An order can have multiple coupons.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<CouponId>,

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

    /// The shipping method to select for fulfilling this order.
    ///
    /// If specified, must be one of the `id`s of a shipping method in the `shipping_methods` array.
    /// If specified, will overwrite the existing selected shipping method, updating `items` as necessary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_shipping_method: Option<&'a str>,

    /// Tracking information once the order has been fulfilled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<Box<UpdateOrderShipping>>,

    /// Current order status.
    ///
    /// One of `created`, `paid`, `canceled`, `fulfilled`, or `returned`.
    /// More detail in the [Orders Guide](https://stripe.com/docs/orders/guide#understanding-order-statuses).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OrderStatus>,
}

impl<'a> UpdateOrder<'a> {
    pub fn new() -> Self {
        UpdateOrder {
            coupon: Default::default(),
            expand: Default::default(),
            metadata: Default::default(),
            selected_shipping_method: Default::default(),
            shipping: Default::default(),
            status: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateOrderShipping {
    pub address: CreateOrderShippingAddress,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<Box<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ListOrdersStatusTransitions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled: Option<Box<RangeQuery<Timestamp>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfilled: Option<Box<RangeQuery<Timestamp>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid: Option<Box<RangeQuery<Timestamp>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned: Option<Box<RangeQuery<Timestamp>>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OrderItemParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Box<i64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<Box<u64>>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<Box<OrderItemParamsType>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateOrderShipping {
    pub carrier: String,

    pub tracking_number: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateOrderShippingAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<Box<String>>,
}

/// An enum representing the possible values of an `OrderItemParams`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OrderItemParamsType {
    Discount,
    Shipping,
    Sku,
    Tax,
}

impl OrderItemParamsType {
    pub fn as_str(self) -> &'static str {
        match self {
            OrderItemParamsType::Discount => "discount",
            OrderItemParamsType::Shipping => "shipping",
            OrderItemParamsType::Sku => "sku",
            OrderItemParamsType::Tax => "tax",
        }
    }
}

impl AsRef<str> for OrderItemParamsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OrderItemParamsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for OrderItemParamsType {
    fn default() -> Self {
        Self::Discount
    }
}

/// An enum representing the possible values of an `UpdateOrder`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    Canceled,
    Created,
    Fulfilled,
    Paid,
    Returned,
}

impl OrderStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            OrderStatus::Canceled => "canceled",
            OrderStatus::Created => "created",
            OrderStatus::Fulfilled => "fulfilled",
            OrderStatus::Paid => "paid",
            OrderStatus::Returned => "returned",
        }
    }
}

impl AsRef<str> for OrderStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for OrderStatus {
    fn default() -> Self {
        Self::Canceled
    }
}
