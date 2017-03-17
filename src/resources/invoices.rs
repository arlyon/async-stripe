use error::Error;
use client::Client;
use params::{List, Metadata, Timestamp};
use resources::{Currency, Discount, Plan};

#[derive(Default, Serialize)]
pub struct InvoiceParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")] pub application_fee: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")] pub customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")] pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")] pub statement_descriptor: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")] pub subscription: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")] pub tax_percent: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")] pub closed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")] pub forgiven: Option<bool>,
}

/*
#[derive(Serialize)]
pub struct InvoiceListLinesParams {
    #[serde(skip_serializing_if = "Option::is_none")] pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")] pub ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub starting_after: Option<String>,

    ..
}
*/

#[derive(Debug, Deserialize)]
pub struct Period {
    pub start: Timestamp,
    pub end: Timestamp,
}

#[derive(Debug, Deserialize)]
pub struct InvoiceLine {
    pub id: String,
    pub amount: u64,
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
    #[serde(rename = "type")] pub item_type: String, // (invoiceitem, subscription)
}

#[derive(Debug, Deserialize)]
pub struct Invoice {
    pub id: String,
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
    pub lines: List<InvoiceLine>,
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

impl Invoice {
    pub fn create(c: &Client, params: InvoiceParams) -> Result<Invoice, Error> {
        return c.post("/invoices", params);
    }

    pub fn get(c: &Client, invoice_id: &str) -> Result<Invoice, Error> {
        return c.get(&format!("/invoices/{}", invoice_id));
    }

    // TODO: Implement InvoiceListLinesParams
    // pub fn get_lines(c: &Client, invoice_id: &str, params: InvoiceListLinesParams) -> Result<List<InvoiceLine>, Error> {
    //     return c.get(&format!("/invoices/{}/lines", invoice_id));
    // }

    // TODO: Implement InvoiceUpcomingParams
    // pub fn get_upcoming(c: &Client, params: InvoiceUpcomingParams) -> Result<Invoice, Error> {
    //     return c.get(&format!("/invoices/upcoming?customer={}", invoice_id));
    // }

    pub fn pay(c: &Client, invoice_id: &str) -> Result<Invoice, Error> {
        return c.post(&format!("/invoices/{}/pay", invoice_id), ());
    }

    // TODO: Implement InvoiceListParams
    // pub fn list(c: &Client, params: InvoiceListParams) -> Result<List<InvoiceLine>, Error> {
    //     return c.get(&format!("/invoices/{}/lines", invoice_id));
    // }
}
