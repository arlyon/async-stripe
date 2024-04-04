#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentIntentNextActionWechatPayDisplayQrCode {
    /// The data being used to generate QR code
    pub data: String,
    /// The URL to the hosted WeChat Pay instructions page, which allows customers to view the WeChat Pay QR code.
    pub hosted_instructions_url: String,
    /// The base64 image data for a pre-generated QR code
    pub image_data_url: String,
    /// The image_url_png string used to render QR code
    pub image_url_png: String,
    /// The image_url_svg string used to render QR code
    pub image_url_svg: String,
}
