use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

/// Deletes the feature attachment to a product
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct DeleteProductFeature {
    id: String,
    product: stripe_shared::ProductId,
}
impl DeleteProductFeature {
    /// Construct a new `DeleteProductFeature`.
    pub fn new(id: impl Into<String>, product: impl Into<stripe_shared::ProductId>) -> Self {
        Self { id: id.into(), product: product.into() }
    }
}
impl DeleteProductFeature {
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

impl StripeRequest for DeleteProductFeature {
    type Output = stripe_product::DeletedProductFeature;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        let product = &self.product;
        RequestBuilder::new(StripeMethod::Delete, format!("/products/{product}/features/{id}"))
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct ListProductProductFeatureBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
impl ListProductProductFeatureBuilder {
    fn new() -> Self {
        Self { ending_before: None, expand: None, limit: None, starting_after: None }
    }
}
/// Retrieve a list of features for a product
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct ListProductProductFeature {
    inner: ListProductProductFeatureBuilder,
    product: stripe_shared::ProductId,
}
impl ListProductProductFeature {
    /// Construct a new `ListProductProductFeature`.
    pub fn new(product: impl Into<stripe_shared::ProductId>) -> Self {
        Self { product: product.into(), inner: ListProductProductFeatureBuilder::new() }
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: impl Into<String>) -> Self {
        self.inner.ending_before = Some(ending_before.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: impl Into<i64>) -> Self {
        self.inner.limit = Some(limit.into());
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
}
impl ListProductProductFeature {
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
        let product = &self.product;

        stripe_client_core::ListPaginator::new_list(
            format!("/products/{product}/features"),
            &self.inner,
        )
    }
}

impl StripeRequest for ListProductProductFeature {
    type Output = stripe_types::List<stripe_product::ProductFeature>;

    fn build(&self) -> RequestBuilder {
        let product = &self.product;
        RequestBuilder::new(StripeMethod::Get, format!("/products/{product}/features"))
            .query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct RetrieveProductFeatureBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveProductFeatureBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a product_feature, which represents a feature attachment to a product
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct RetrieveProductFeature {
    inner: RetrieveProductFeatureBuilder,
    id: String,
    product: stripe_shared::ProductId,
}
impl RetrieveProductFeature {
    /// Construct a new `RetrieveProductFeature`.
    pub fn new(id: impl Into<String>, product: impl Into<stripe_shared::ProductId>) -> Self {
        Self { id: id.into(), product: product.into(), inner: RetrieveProductFeatureBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveProductFeature {
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

impl StripeRequest for RetrieveProductFeature {
    type Output = stripe_product::ProductFeature;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        let product = &self.product;
        RequestBuilder::new(StripeMethod::Get, format!("/products/{product}/features/{id}"))
            .query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct CreateProductProductFeatureBuilder {
    entitlement_feature: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl CreateProductProductFeatureBuilder {
    fn new(entitlement_feature: impl Into<String>) -> Self {
        Self { entitlement_feature: entitlement_feature.into(), expand: None }
    }
}
/// Creates a product_feature, which represents a feature attachment to a product
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateProductProductFeature {
    inner: CreateProductProductFeatureBuilder,
    product: stripe_shared::ProductId,
}
impl CreateProductProductFeature {
    /// Construct a new `CreateProductProductFeature`.
    pub fn new(
        product: impl Into<stripe_shared::ProductId>,
        entitlement_feature: impl Into<String>,
    ) -> Self {
        Self {
            product: product.into(),
            inner: CreateProductProductFeatureBuilder::new(entitlement_feature.into()),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl CreateProductProductFeature {
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

impl StripeRequest for CreateProductProductFeature {
    type Output = stripe_product::ProductFeature;

    fn build(&self) -> RequestBuilder {
        let product = &self.product;
        RequestBuilder::new(StripeMethod::Post, format!("/products/{product}/features"))
            .form(&self.inner)
    }
}
