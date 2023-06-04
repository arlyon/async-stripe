#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PromptpayDisplayQrCode {
    /// The raw data string used to generate QR code, it should be used together with QR code library.
    pub data: String,
    /// The URL to the hosted PromptPay instructions page, which allows customers to view the PromptPay QR code.
    pub hosted_instructions_url: String,
    /// The image_url_png string used to render QR code, can be used as <img src="…" />.
    pub image_url_png: String,
    /// The image_url_svg string used to render QR code, can be used as <img src="…" />.
    pub image_url_svg: String,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PromptpayDisplayQrCode {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
