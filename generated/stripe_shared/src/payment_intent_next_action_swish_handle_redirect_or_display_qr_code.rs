#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentIntentNextActionSwishHandleRedirectOrDisplayQrCode {
    /// The URL to the hosted Swish instructions page, which allows customers to view the QR code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_instructions_url: Option<String>,
    /// The url for mobile redirect based auth
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_auth_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qr_code: Option<stripe_shared::PaymentIntentNextActionSwishQrCode>,
}
