/// The resource representing a Stripe order.
///
/// For more details see https://stripe.com/docs/api#invoice_object.
#[derive(Debug, Deserialize)]
pub struct Order {
    pub id: Option<String>, // id field is not present when retrieving upcoming invoices
    pub amount_due: u64,
    pub application_fee: Option<u64>,
    pub attempt_count: u64,
    pub attempted: bool,
    pub charge: Option<String>,
    pub closed: bool,
    // pub currency: Currency,
    // pub customer: String,
    // pub date: Timestamp,
    // pub description: Option<String>,
    // pub discount: Option<Discount>,
    // pub ending_balance: Option<i64>,
    // pub forgiven: bool,
    // pub lines: List<InvoiceItem>,
    // pub livemode: bool,
    // pub metadata: Metadata,
    // pub next_payment_attempt: Option<Timestamp>,
    // pub paid: bool,
    // pub period_end: Timestamp,
    // pub period_start: Timestamp,
    // pub receipt_number: Option<String>,
    // pub starting_balance: i64,
    // pub statment_descriptor: Option<String>,
    // pub subscription: Option<String>,
    // pub subscription_proration_date: Option<Timestamp>,
    // pub subtotal: i64,
    // pub tax: Option<i64>,
    // pub tax_percent: Option<f64>,
    // pub total: i64,
    // pub webhooks_delivered_at: Option<Timestamp>,
}
