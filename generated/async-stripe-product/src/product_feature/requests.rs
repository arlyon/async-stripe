use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

/// Deletes the feature attachment to a product
#[derive(Clone, Debug, serde::Serialize)]
pub struct DeleteProductFeature<'a> {
    id: &'a str,
    product: &'a stripe_shared::ProductId,
}
impl<'a> DeleteProductFeature<'a> {
    /// Construct a new `DeleteProductFeature`.
    pub fn new(id: &'a str, product: &'a stripe_shared::ProductId) -> Self {
        Self { id, product }
    }
}
impl DeleteProductFeature<'_> {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for DeleteProductFeature<'_> {
    type Output = stripe_product::DeletedProductFeature;

    fn build(&self) -> RequestBuilder {
        let id = self.id;
        let product = self.product;
        RequestBuilder::new(StripeMethod::Delete, format!("/products/{product}/features/{id}"))
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListProductProductFeatureBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
}
impl<'a> ListProductProductFeatureBuilder<'a> {
    fn new() -> Self {
        Self { ending_before: None, expand: None, limit: None, starting_after: None }
    }
}
/// Retrieve a list of features for a product
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListProductProductFeature<'a> {
    inner: ListProductProductFeatureBuilder<'a>,
    product: &'a stripe_shared::ProductId,
}
impl<'a> ListProductProductFeature<'a> {
    /// Construct a new `ListProductProductFeature`.
    pub fn new(product: &'a stripe_shared::ProductId) -> Self {
        Self { product, inner: ListProductProductFeatureBuilder::new() }
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: &'a str) -> Self {
        self.inner.ending_before = Some(ending_before);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: i64) -> Self {
        self.inner.limit = Some(limit);
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: &'a str) -> Self {
        self.inner.starting_after = Some(starting_after);
        self
    }
}
impl ListProductProductFeature<'_> {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    pub fn paginate(
        &self,
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_product::ProductFeature>> {
        let product = self.product;

        stripe_client_core::ListPaginator::new_list(
            format!("/products/{product}/features"),
            self.inner,
        )
    }
}

impl StripeRequest for ListProductProductFeature<'_> {
    type Output = stripe_types::List<stripe_product::ProductFeature>;

    fn build(&self) -> RequestBuilder {
        let product = self.product;
        RequestBuilder::new(StripeMethod::Get, format!("/products/{product}/features"))
            .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveProductFeatureBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveProductFeatureBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a product_feature, which represents a feature attachment to a product
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveProductFeature<'a> {
    inner: RetrieveProductFeatureBuilder<'a>,
    id: &'a str,
    product: &'a stripe_shared::ProductId,
}
impl<'a> RetrieveProductFeature<'a> {
    /// Construct a new `RetrieveProductFeature`.
    pub fn new(id: &'a str, product: &'a stripe_shared::ProductId) -> Self {
        Self { id, product, inner: RetrieveProductFeatureBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveProductFeature<'_> {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for RetrieveProductFeature<'_> {
    type Output = stripe_product::ProductFeature;

    fn build(&self) -> RequestBuilder {
        let id = self.id;
        let product = self.product;
        RequestBuilder::new(StripeMethod::Get, format!("/products/{product}/features/{id}"))
            .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreateProductProductFeatureBuilder<'a> {
    entitlement_feature: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> CreateProductProductFeatureBuilder<'a> {
    fn new(entitlement_feature: &'a str) -> Self {
        Self { entitlement_feature, expand: None }
    }
}
/// Creates a product_feature, which represents a feature attachment to a product
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateProductProductFeature<'a> {
    inner: CreateProductProductFeatureBuilder<'a>,
    product: &'a stripe_shared::ProductId,
}
impl<'a> CreateProductProductFeature<'a> {
    /// Construct a new `CreateProductProductFeature`.
    pub fn new(product: &'a stripe_shared::ProductId, entitlement_feature: &'a str) -> Self {
        Self { product, inner: CreateProductProductFeatureBuilder::new(entitlement_feature) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl CreateProductProductFeature<'_> {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for CreateProductProductFeature<'_> {
    type Output = stripe_product::ProductFeature;

    fn build(&self) -> RequestBuilder {
        let product = self.product;
        RequestBuilder::new(StripeMethod::Post, format!("/products/{product}/features"))
            .form(&self.inner)
    }
}
