#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListClimateOrder<'a> {
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
}
impl<'a> ListClimateOrder<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ListClimateOrder<'a> {
    /// Lists all Climate order objects. The orders are returned sorted by creation date, with the
    /// most recently created orders appearing first.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_misc::ClimateOrder>> {
        client.get_query("/climate/orders", self)
    }
    pub fn paginate(self) -> stripe::ListPaginator<stripe_types::List<stripe_misc::ClimateOrder>> {
        stripe::ListPaginator::from_list_params("/climate/orders", self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveClimateOrder<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveClimateOrder<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveClimateOrder<'a> {
    /// Retrieves the details of a Climate order object with the given ID.
    pub fn send(
        &self,
        client: &stripe::Client,
        order: &stripe_misc::ClimateOrderId,
    ) -> stripe::Response<stripe_misc::ClimateOrder> {
        client.get_query(&format!("/climate/orders/{order}"), self)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateClimateOrder<'a> {
    /// Requested amount of carbon removal units. Either this or `metric_tons` must be specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// Publicly sharable reference for the end beneficiary of carbon removal.
    /// Assumed to be the Stripe account if not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary: Option<BeneficiaryParams<'a>>,
    /// Request currency for the order as a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a supported [settlement currency for your account](https://stripe.com/docs/currencies).
    /// If omitted, the account's default currency will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// Requested number of tons for the order. Either this or `amount` must be specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_tons: Option<&'a str>,
    /// Unique identifier of the Climate product.
    pub product: &'a str,
}
impl<'a> CreateClimateOrder<'a> {
    pub fn new(product: &'a str) -> Self {
        Self {
            amount: None,
            beneficiary: None,
            currency: None,
            expand: None,
            metadata: None,
            metric_tons: None,
            product,
        }
    }
}
impl<'a> CreateClimateOrder<'a> {
    /// Creates a Climate order object for a given Climate product. The order will be processed immediately
    /// after creation and payment will be deducted your Stripe balance.
    pub fn send(&self, client: &stripe::Client) -> stripe::Response<stripe_misc::ClimateOrder> {
        client.send_form("/climate/orders", self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateClimateOrder<'a> {
    /// Publicly sharable reference for the end beneficiary of carbon removal.
    /// Assumed to be the Stripe account if not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary: Option<BeneficiaryParams<'a>>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
}
impl<'a> UpdateClimateOrder<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> UpdateClimateOrder<'a> {
    /// Updates the specified order by setting the values of the parameters passed.
    pub fn send(
        &self,
        client: &stripe::Client,
        order: &stripe_misc::ClimateOrderId,
    ) -> stripe::Response<stripe_misc::ClimateOrder> {
        client.send_form(&format!("/climate/orders/{order}"), self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CancelClimateOrder<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> CancelClimateOrder<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> CancelClimateOrder<'a> {
    /// Cancels a Climate order. You can cancel an order within 30 days of creation. Stripe refunds the
    /// reservation `amount_subtotal`, but not the `amount_fees` for user-triggered cancellations. Frontier
    /// might cancel reservations if suppliers fail to deliver. If Frontier cancels the reservation, Stripe
    /// provides 90 days advance notice and refunds the `amount_total`.
    pub fn send(
        &self,
        client: &stripe::Client,
        order: &stripe_misc::ClimateOrderId,
    ) -> stripe::Response<stripe_misc::ClimateOrder> {
        client.send_form(&format!("/climate/orders/{order}/cancel"), self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct BeneficiaryParams<'a> {
    /// Publicly displayable name for the end beneficiary of carbon removal.
    pub public_name: &'a str,
}
impl<'a> BeneficiaryParams<'a> {
    pub fn new(public_name: &'a str) -> Self {
        Self { public_name }
    }
}
