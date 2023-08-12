#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListPaymentSource<'a> {
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
    /// Filter sources according to a particular object type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<&'a str>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
}
impl<'a> ListPaymentSource<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ListPaymentSource<'a> {
    /// List sources for a specified customer.
    pub fn send(
        &self,
        client: &stripe::Client,
        customer: &stripe_types::customer::CustomerId,
    ) -> stripe::Response<stripe_types::List<stripe_types::PaymentSource>> {
        client.get_query(&format!("/customers/{customer}/sources", customer = customer), self)
    }
    pub fn paginate(
        self,
        customer: &stripe_types::customer::CustomerId,
    ) -> stripe::ListPaginator<stripe_types::PaymentSource> {
        stripe::ListPaginator::from_params(
            &format!("/customers/{customer}/sources", customer = customer),
            self,
        )
    }
}
impl<'a> stripe::PaginationParams for ListPaymentSource<'a> {}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrievePaymentSource<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrievePaymentSource<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrievePaymentSource<'a> {
    /// Retrieve a specified source for a given customer.
    pub fn send(
        &self,
        client: &stripe::Client,
        customer: &stripe_types::customer::CustomerId,
        id: &str,
    ) -> stripe::Response<stripe_types::PaymentSource> {
        client.get_query(
            &format!("/customers/{customer}/sources/{id}", customer = customer, id = id),
            self,
        )
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentSource<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// Please refer to full [documentation](https://stripe.com/docs/api) instead.
    pub source: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate: Option<bool>,
}
impl<'a> CreatePaymentSource<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            expand: Default::default(),
            metadata: Default::default(),
            source,
            validate: Default::default(),
        }
    }
}
impl<'a> CreatePaymentSource<'a> {
    /// When you create a new credit card, you must specify a customer or recipient on which to create it.
    ///
    /// If the card’s owner has no default card, then the new card will become the default.
    /// However, if the owner already has a default, then it will not change.
    /// To change the default, you should [update the customer](https://stripe.com/docs/api#update_customer) to have a new `default_source`.
    pub fn send(
        &self,
        client: &stripe::Client,
        customer: &stripe_types::customer::CustomerId,
    ) -> stripe::Response<stripe_types::PaymentSource> {
        client.send_form(
            &format!("/customers/{customer}/sources", customer = customer),
            self,
            http_types::Method::Post,
        )
    }
}
