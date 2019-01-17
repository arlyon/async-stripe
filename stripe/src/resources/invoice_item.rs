use crate::params::{Metadata, Timestamp};
use crate::resources::{Currency, Period, Plan};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe invoice item.
///
/// For more details see https://stripe.com/docs/api#invoiceitem_object.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InvoiceItem {
    pub id: String,
    pub object: String,
    pub amount: u64,
    pub currency: Currency,
    pub customer: String,
    pub date: Timestamp,
    pub description: String,
    pub discountable: String,
    pub invoice: String,
    pub livemode: bool,
    pub metadata: Metadata,
    pub period: Period,
    pub plan: Option<Plan>,
    pub proration: bool,
    pub quantity: u64,
    pub subscription: Option<String>,
    pub subscription_item: Option<String>,
}
