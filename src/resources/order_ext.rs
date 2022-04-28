use serde::{Deserialize, Serialize};

/// An enum representing the possible values of an `ListOrders`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatusFilter {
    Created,
    Fulfilled,
    Paid,
    Refunded,
}

impl OrderStatusFilter {
    pub fn as_str(self) -> &'static str {
        match self {
            OrderStatusFilter::Created => "created",
            OrderStatusFilter::Fulfilled => "fulfilled",
            OrderStatusFilter::Paid => "paid",
            OrderStatusFilter::Refunded => "refunded",
        }
    }
}

impl AsRef<str> for OrderStatusFilter {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OrderStatusFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
