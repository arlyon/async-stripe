#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentIntentNextActionCashappHandleRedirectOrDisplayQrCode {
    /// The URL to the hosted Cash App Pay instructions page, which allows customers to view the QR code, and supports QR code refreshing on expiration.
    pub hosted_instructions_url: String,
    /// The url for mobile redirect based auth
    pub mobile_auth_url: String,
    pub qr_code: stripe_shared::PaymentIntentNextActionCashappQrCode,
}
