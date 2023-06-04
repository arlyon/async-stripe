use crate::{Client, Response};

impl crate::invoice_line_item::InvoiceLineItem {
    /// When retrieving an invoice, you’ll get a **lines** property containing the total count of line items and the first handful of those items.
    ///
    /// There is also a URL where you can retrieve the full (paginated) list of line items.
    pub fn list(
        client: &Client,
        invoice: &crate::invoice::InvoiceId,
        params: ListInvoiceLineItem,
    ) -> Response<crate::List<crate::invoice_line_item::InvoiceLineItem>> {
        client.get_query(&format!("/invoices/{invoice}/lines", invoice = invoice), params)
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListInvoiceLineItem<'a> {
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}
impl<'a> ListInvoiceLineItem<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
