
/// Retrieves a Mandate object.
pub fn retrieve(
    client: &stripe::Client,
    mandate: &stripe_types::mandate::MandateId,
    params: RetrieveMandate,
) -> stripe::Response<stripe_types::mandate::Mandate> {
    client.get_query(&format!("/mandates/{mandate}", mandate = mandate), params)
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveMandate<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveMandate<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
