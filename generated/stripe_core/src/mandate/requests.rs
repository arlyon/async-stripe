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
impl<'a> RetrieveMandate<'a> {
    /// Retrieves a Mandate object.
    pub fn send(
        &self,
        client: &stripe::Client,
        mandate: &stripe_shared::MandateId,
    ) -> stripe::Response<stripe_shared::Mandate> {
        client.get_query(&format!("/mandates/{mandate}"), self)
    }
}
