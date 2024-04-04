#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListFile<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
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
    /// Filter queries by the file purpose.
    /// If you don't provide a purpose, the queries return unfiltered files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<stripe_shared::FilePurpose>,
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
}
impl<'a> ListFile<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ListFile<'a> {
    /// Returns a list of the files that your account has access to.
    /// Stripe sorts and returns the files by their creation dates, placing the most recently created files at the top.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_shared::File>> {
        client.get_query("/files", self)
    }
    pub fn paginate(self) -> stripe::ListPaginator<stripe_types::List<stripe_shared::File>> {
        stripe::ListPaginator::from_list_params("/files", self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveFile<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveFile<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveFile<'a> {
    /// Retrieves the details of an existing file object.
    /// After you supply a unique file ID, Stripe returns the corresponding file object.
    /// Learn how to [access file contents](https://stripe.com/docs/file-upload#download-file-contents).
    pub fn send(
        &self,
        client: &stripe::Client,
        file: &stripe_shared::FileId,
    ) -> stripe::Response<stripe_shared::File> {
        client.get_query(&format!("/files/{file}"), self)
    }
}
