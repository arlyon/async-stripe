use stripe::{Client, Response};

impl stripe_core::file_link::FileLink {
    /// Retrieves the file link with the given ID.
    pub fn retrieve(
        client: &Client,
        link: &str,
        params: RetrieveFileLink,
    ) -> Response<stripe_core::file_link::FileLink> {
        client.get_query(&format!("/file_links/{link}", link = link), params)
    }
    /// Creates a new file link object.
    pub fn create(
        client: &Client,
        params: CreateFileLink,
    ) -> Response<stripe_core::file_link::FileLink> {
        client.send_form("/file_links", params, http_types::Method::Post)
    }
    /// Updates an existing file link object.
    ///
    /// Expired links can no longer be updated.
    pub fn update(
        client: &Client,
        link: &str,
        params: UpdateFileLink,
    ) -> Response<stripe_core::file_link::FileLink> {
        client.send_form(
            &format!("/file_links/{link}", link = link),
            params,
            http_types::Method::Post,
        )
    }
    /// Returns a list of file links.
    pub fn list(
        client: &Client,
        params: ListFileLink,
    ) -> Response<stripe_types::List<stripe_core::file_link::FileLink>> {
        client.get_query("/file_links", params)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveFileLink<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveFileLink<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateFileLink<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A future timestamp after which the link will no longer be usable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<stripe_types::Timestamp>,
    /// The ID of the file.
    ///
    /// The file's `purpose` must be one of the following: `business_icon`, `business_logo`, `customer_signature`, `dispute_evidence`, `finance_report_run`, `identity_document_downloadable`, `pci_document`, `selfie`, `sigma_scheduled_query`, `tax_document_user_upload`, or `terminal_reader_splashscreen`.
    pub file: &'a str,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a stripe_types::Metadata>,
}
impl<'a> CreateFileLink<'a> {
    pub fn new(file: &'a str) -> Self {
        Self {
            expand: Default::default(),
            expires_at: Default::default(),
            file,
            metadata: Default::default(),
        }
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateFileLink<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A future timestamp after which the link will no longer be usable, or `now` to expire the link immediately.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<UpdateFileLinkExpiresAt>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a stripe_types::Metadata>,
}
impl<'a> UpdateFileLink<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// A future timestamp after which the link will no longer be usable, or `now` to expire the link immediately.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum UpdateFileLinkExpiresAt {
    Now,
    Timestamp(stripe_types::Timestamp),
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListFileLink<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Filter links by their expiration status.
    ///
    /// By default, all links are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired: Option<bool>,
    /// Only return links for the given file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<&'a str>,
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
impl<'a> ListFileLink<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
