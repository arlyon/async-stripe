use serde::{Deserialize, Serialize};

use crate::client::{Client, Response};
use crate::ids::{CustomerId, InvoiceId};
use crate::resources::{Currency, InvoiceLineItem};

impl InvoiceLineItem {
    /// Creates an invoice line item.
    ///
    /// For more details see <https://stripe.com/docs/api#invoice_line_item_object>.
    pub fn create(client: &Client, params: CreateInvoiceLineItem<'_>) -> Response<InvoiceLineItem> {
        client.post_form("/invoiceitems", &params)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateInvoiceLineItem<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discountable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<InvoiceId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<bool>,
}

impl CreateInvoiceLineItem<'_> {
    pub fn new() -> Self {
        CreateInvoiceLineItem {
            amount: None,
            currency: None,
            customer: None,
            description: None,
            discountable: None,
            invoice: None,
            subscription: None,
        }
    }
}
