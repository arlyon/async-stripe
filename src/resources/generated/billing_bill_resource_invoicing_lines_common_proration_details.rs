// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "BillingBillResourceInvoicingLinesCommonProrationDetails".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BillingBillResourceInvoicingLinesCommonProrationDetails {

    /// For a credit proration `line_item`, the original debit line_items to which the credit proration applies.
    pub credited_items: Option<BillingBillResourceInvoicingLinesCommonCreditedItems>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BillingBillResourceInvoicingLinesCommonCreditedItems {

    /// Invoice containing the credited invoice line items.
    pub invoice: String,

    /// Credited invoice line items.
    pub invoice_line_items: Vec<String>,
}
