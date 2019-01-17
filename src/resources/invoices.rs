use crate::config::{err, ok, Client, Response};
use crate::params::{Identifiable, List, Metadata, RangeQuery, Timestamp};
use crate::resources::{Currency, Discount, Plan};
use serde_derive::{Deserialize, Serialize};

/// The set of parameters that can be used when creating or updating an invoice.
///
/// For more details see https://stripe.com/docs/api#create_invoice, https://stripe.com/docs/api#update_invoice.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InvoiceParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_percent: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forgiven: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InvoiceLineItemParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discountable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<bool>,
}

/*
#[derive(Debug, Deserialize, Serialize)]
pub struct InvoiceListLinesParams {
    #[serde(skip_serializing_if = "Option::is_none")] pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")] pub ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub starting_after: Option<String>,

    ..
}
*/

#[derive(Clone, Debug, Default, Serialize)]
pub struct InvoiceUpcomingParams<'a> {
    pub customer: &'a str, // this is a required param
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_items: Option<SubscriptionItemParams<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_prorate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_proration_date: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_tax_percent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_trial_end: Option<Timestamp>,
}

#[derive(Clone, Debug, Serialize)]
pub struct SubscriptionItemParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
}

/// Period is a structure representing a start and end dates.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Period {
    pub start: Timestamp,
    pub end: Timestamp,
}

/// The resource representing a Stripe invoice line item.
///
/// For more details see https://stripe.com/docs/api#invoice_line_item_object.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InvoiceLineItem {
    pub id: String,
    pub amount: i64,
    pub currency: Currency,
    pub description: Option<String>,
    pub discountable: bool,
    pub livemode: bool,
    pub metadata: Metadata,
    pub period: Period,
    pub plan: Option<Plan>,
    pub proration: bool,
    pub quantity: Option<u64>,
    pub subscription: Option<String>,
    pub subscription_item: Option<String>,
    #[serde(default)]
    // NOTE: Missing in response to InvoiceLineItem create
    #[serde(rename = "type")]
    pub item_type: String, // (invoiceitem, subscription)
}

impl Identifiable for InvoiceLineItem {
    fn id(&self) -> &str {
        &self.id
    }
}

/// The resource representing a Stripe invoice.
///
/// For more details see https://stripe.com/docs/api#invoice_object.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Invoice {
    pub id: Option<String>, // id field is not present when retrieving upcoming invoices
    pub amount_due: u64,
    pub application_fee: Option<u64>,
    pub attempt_count: u64,
    pub attempted: bool,
    pub charge: Option<String>,
    pub closed: bool,
    pub currency: Currency,
    pub customer: String,
    pub date: Timestamp,
    pub description: Option<String>,
    pub discount: Option<Discount>,
    pub ending_balance: Option<i64>,
    pub forgiven: bool,
    pub lines: List<InvoiceLineItem>,
    pub livemode: bool,
    pub metadata: Metadata,
    pub next_payment_attempt: Option<Timestamp>,
    pub paid: bool,
    pub period_end: Timestamp,
    pub period_start: Timestamp,
    pub receipt_number: Option<String>,
    pub starting_balance: i64,
    pub statment_descriptor: Option<String>,
    pub subscription: Option<String>,
    pub subscription_proration_date: Option<Timestamp>,
    pub subtotal: i64,
    pub tax: Option<i64>,
    pub tax_percent: Option<f64>,
    pub total: i64,
    pub webhooks_delivered_at: Option<Timestamp>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InvoiceListParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<RangeQuery<Timestamp>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<&'a str>,
}

impl Invoice {
    /// Creates a new invoice.
    ///
    /// For more details see https://stripe.com/docs/api#create_invoice.
    pub fn create(client: &Client, params: InvoiceParams<'_>) -> Response<Invoice> {
        client.post_form("/invoices", params)
    }

    /// Retrieves the details of an invoice.
    ///
    /// For more details see https://stripe.com/docs/api#retrieve_invoice.
    pub fn retrieve(client: &Client, invoice_id: &str) -> Response<Invoice> {
        client.get(&format!("/invoices/{}", invoice_id))
    }

    // TODO: Implement InvoiceListLinesParams
    // pub fn get_lines(client: &Client, invoice_id: &str, params: InvoiceListLinesParams) -> Response<List<InvoiceLineItem>> {
    //     client.get(&format!("/invoices/{}/lines", invoice_id))
    // }

    /// Retrieves the details of an upcoming invoice_id
    ///
    /// For more details see https://stripe.com/docs/api#upcoming_invoice
    pub fn upcoming(client: &Client, params: InvoiceUpcomingParams<'_>) -> Response<Invoice> {
        client.get_query("/invoices/upcoming", &params)
    }

    /// Pays an invoice.
    ///
    /// For more details see https://stripe.com/docs/api#pay_invoice.
    pub fn pay(client: &Client, invoice_id: &str) -> Response<Invoice> {
        client.post(&format!("/invoices/{}/pay", invoice_id))
    }

    /// Updates an invoice.
    ///
    /// For more details see https://stripe.com/docs/api#update_invoice.
    pub fn update(
        client: &Client,
        invoice_id: &str,
        params: InvoiceParams<'_>,
    ) -> Response<Invoice> {
        client.post_form(&format!("/invoices/{}", invoice_id), &params)
    }

    /// Lists all invoices.
    ///
    /// For more details see https://stripe.com/docs/api#list_invoices.
    pub fn list(client: &Client, params: InvoiceListParams<'_>) -> Response<List<Invoice>> {
        client.get_query("/invoices", &params)
    }
}

impl InvoiceLineItem {
    /// Creates an invoice line item.
    ///
    /// For more details see https://stripe.com/docs/api#invoice_line_item_object
    pub fn create(client: &Client, params: InvoiceLineItemParams<'_>) -> Response<InvoiceLineItem> {
        client.post_form("/invoiceitems", &params)
    }
}

// N.B. Since Invoice ID can be empty, override impl for pagination
impl List<Invoice> {
    /// Repeatedly queries Stripe for more data until all elements in list are fetched, using
    /// Stripe's default page size.
    ///
    /// Not supported by `stripe::async::Client`.
    #[cfg(not(feature = "async"))]
    pub fn get_all(self, client: &Client) -> Response<Vec<Invoice>> {
        let mut data = Vec::new();
        let mut next = self;
        loop {
            if next.has_more {
                let resp = next.next(&client)?;
                data.extend(next.data);
                next = resp;
            } else {
                data.extend(next.data);
                break;
            }
        }
        Ok(data)
    }

    /// Fetch additional page of data from stripe
    pub fn next(&self, client: &Client) -> Response<List<Invoice>> {
        use crate::error::Error;

        if let Some(last) = self.data.last() {
            if let Some(last_id) = &last.id {
                List::get_next(client, &self.url, last_id)
            } else {
                let invariant = "Cannot paginate List<Invoice>; Stripe returned Invoice with no ID";
                err(Error::Unexpected(invariant))
            }
        } else {
            ok(List {
                data: Vec::new(),
                has_more: false,
                total_count: self.total_count,
                url: self.url.clone(),
            })
        }
    }
}
