#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentIntentNextActionWechatPayRedirectToIosApp {
    /// An universal link that redirect to WeChat Pay app.
    pub native_url: String,
}
