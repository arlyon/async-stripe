#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct NextActionRedirectToUrl {
    /// If the customer does not exit their browser while authenticating, they will be redirected to this specified URL after completion.
    pub return_url: Option<String>,
    /// The URL you must redirect your customer to in order to authenticate.
    pub url: Option<String>,
}
