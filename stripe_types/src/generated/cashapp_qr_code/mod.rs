#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CashappQrCode {
    /// The date (unix timestamp) when the QR code expires.
    pub expires_at: stripe_types::Timestamp,
    /// The image_url_png string used to render QR code.
    pub image_url_png: String,
    /// The image_url_svg string used to render QR code.
    pub image_url_svg: String,
}
