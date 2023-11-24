#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListCreditNoteLineItem<'a> {
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
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
    pub starting_after: Option<&'a str>,
}
impl<'a> ListCreditNoteLineItem<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ListCreditNoteLineItem<'a> {
    /// When retrieving a credit note, you’ll get a **lines** property containing the the first handful of those items.
    ///
    /// There is also a URL where you can retrieve the full (paginated) list of line items.
    pub fn send(
        &self,
        client: &stripe::Client,
        credit_note: &stripe_types::credit_note::CreditNoteId,
    ) -> stripe::Response<stripe_types::List<stripe_types::CreditNoteLineItem>> {
        client.get_query(&format!("/credit_notes/{credit_note}/lines"), self)
    }
    pub fn paginate(
        self,
        credit_note: &stripe_types::credit_note::CreditNoteId,
    ) -> stripe::ListPaginator<stripe_types::CreditNoteLineItem> {
        stripe::ListPaginator::from_params(&format!("/credit_notes/{credit_note}/lines"), self)
    }
}
impl<'a> stripe::PaginationParams for ListCreditNoteLineItem<'a> {}
