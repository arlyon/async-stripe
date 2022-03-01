// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::config::{Client, Response};
use crate::ids::{OrderId, OrderReturnId};
use crate::params::{Expand, Expandable, List, Object, RangeQuery, Timestamp};
use crate::resources::{Currency, Order, OrderItem, Refund};

/// The resource representing a Stripe "OrderReturn".
///
/// For more details see <https://stripe.com/docs/api/order_returns/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OrderReturn {
    /// Unique identifier for the object.
    pub id: OrderReturnId,

    /// A positive integer in the smallest currency unit (that is, 100 cents for $1.00, or 1 for ¥1, Japanese Yen being a zero-decimal currency) representing the total amount for the returned line item.
    pub amount: i64,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// The items included in this order return.
    pub items: Vec<OrderItem>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The order that this return includes items from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Box<Expandable<Order>>>,

    /// The ID of the refund issued for this return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund: Option<Box<Expandable<Refund>>>,
}

impl OrderReturn {
    /// Returns a list of your order returns.
    ///
    /// The returns are returned sorted by creation date, with the most recently created return appearing first.
    pub fn list(client: &Client, params: ListOrderReturns<'_>) -> Response<List<OrderReturn>> {
        client.get_query("/order_returns", &params)
    }

    /// Retrieves the details of an existing order return.
    ///
    /// Supply the unique order ID from either an order return creation request or the order return list, and Stripe will return the corresponding order information.
    pub fn retrieve(client: &Client, id: &OrderReturnId, expand: &[&str]) -> Response<OrderReturn> {
        client.get_query(&format!("/order_returns/{}", id), &Expand { expand })
    }
}

impl Object for OrderReturn {
    type Id = OrderReturnId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "order_return"
    }
}

/// The parameters for `OrderReturn::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListOrderReturns<'a> {
    /// Date this return was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<OrderReturnId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// The order to retrieve returns for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<OrderId>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<OrderReturnId>,
}

impl<'a> ListOrderReturns<'a> {
    pub fn new() -> Self {
        ListOrderReturns {
            created: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            order: Default::default(),
            starting_after: Default::default(),
        }
    }
}
