#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct WechatPayDisplayQrCode {
    /// The data being used to generate QR code.
    pub data: String,
    /// The base64 image data for a pre-generated QR code.
    pub image_data_url: String,
    /// The image_url_png string used to render QR code.
    pub image_url_png: String,
    /// The image_url_svg string used to render QR code.
    pub image_url_svg: String,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for WechatPayDisplayQrCode {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}