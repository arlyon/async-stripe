#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PromptpayDisplayQrCode {
    /// The raw data string used to generate QR code, it should be used together with QR code library.
    pub data: String,
    /// The URL to the hosted PromptPay instructions page, which allows customers to view the PromptPay QR code.
    pub hosted_instructions_url: String,
    /// The PNG path used to render the QR code, can be used as the source in an HTML img tag.
    pub image_url_png: String,
    /// The SVG path used to render the QR code, can be used as the source in an HTML img tag.
    pub image_url_svg: String,
}
