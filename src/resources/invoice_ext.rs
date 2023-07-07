use serde::Serialize;

use crate::client::{Client, Response};
use crate::ids::{CouponId, CustomerId, InvoiceId, PlanId, SubscriptionId, SubscriptionItemId};
use crate::params::{Metadata, Timestamp};
use crate::resources::{CollectionMethod, Invoice};

#[deprecated(since = "0.12.0")]
pub type InvoiceCollectionMethod = CollectionMethod;

impl Invoice {
    /// Retrieves the details of an upcoming invoice_id
    ///
    /// For more details see <https://stripe.com/docs/api#upcoming_invoice>.
    pub fn upcoming(client: &Client, params: RetrieveUpcomingInvoice) -> Response<Invoice> {
        client.get_query("/invoices/upcoming", &params)
    }

    /// Pays an invoice.
    ///
    /// For more details see <https://stripe.com/docs/api#pay_invoice.>.
    pub fn pay(client: &Client, invoice_id: &InvoiceId) -> Response<Invoice> {
        client.post(&format!("/invoices/{}/pay", invoice_id))
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct RetrieveUpcomingInvoice {
    pub customer: CustomerId, // this is a required param
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<CouponId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<SubscriptionId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_items: Option<Vec<UpdateSubscriptionItems>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_proration_behavior: Option<SubscriptionProrationBehavior>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_proration_date: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_start_date: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_trial_end: Option<Timestamp>,
}

impl RetrieveUpcomingInvoice {
    pub fn new(customer: CustomerId) -> Self {
        RetrieveUpcomingInvoice {
            customer,
            coupon: None,
            subscription: None,
            subscription_items: None,
            subscription_proration_behavior: None,
            subscription_proration_date: None,
            subscription_start_date: None,
            subscription_trial_end: None,
        }
    }
}
