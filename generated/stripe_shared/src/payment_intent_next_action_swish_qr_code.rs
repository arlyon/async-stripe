#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentIntentNextActionSwishQrCode {
    /// The raw data string used to generate QR code, it should be used together with QR code library.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// The image_url_png string used to render QR code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url_png: Option<String>,
    /// The image_url_svg string used to render QR code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url_svg: Option<String>,
}
