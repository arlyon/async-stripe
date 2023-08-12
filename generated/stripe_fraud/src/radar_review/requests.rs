#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListRadarReview<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
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
impl<'a> ListRadarReview<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ListRadarReview<'a> {
    /// Returns a list of `Review` objects that have `open` set to `true`.
    ///
    /// The objects are sorted in descending order by creation date, with the most recently created object appearing first.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_types::RadarReview>> {
        client.get_query("/reviews", self)
    }
    pub fn paginate(self) -> stripe::ListPaginator<stripe_types::RadarReview> {
        stripe::ListPaginator::from_params("/reviews", self)
    }
}
impl<'a> stripe::PaginationParams for ListRadarReview<'a> {}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveRadarReview<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveRadarReview<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveRadarReview<'a> {
    /// Retrieves a `Review` object.
    pub fn send(
        &self,
        client: &stripe::Client,
        review: &stripe_types::radar_review::ReviewId,
    ) -> stripe::Response<stripe_types::RadarReview> {
        client.get_query(&format!("/reviews/{review}", review = review), self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ApproveRadarReview<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> ApproveRadarReview<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ApproveRadarReview<'a> {
    /// Approves a `Review` object, closing it and removing it from the list of reviews.
    pub fn send(
        &self,
        client: &stripe::Client,
        review: &stripe_types::radar_review::ReviewId,
    ) -> stripe::Response<stripe_types::RadarReview> {
        client.send_form(
            &format!("/reviews/{review}/approve", review = review),
            self,
            http_types::Method::Post,
        )
    }
}
