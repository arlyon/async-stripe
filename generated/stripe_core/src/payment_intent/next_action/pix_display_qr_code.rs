#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PixDisplayQrCode {
    /// The raw data string used to generate QR code, it should be used together with QR code library.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// The date (unix timestamp) when the PIX expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    /// The URL to the hosted pix instructions page, which allows customers to view the pix QR code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_instructions_url: Option<String>,
    /// The image_url_png string used to render png QR code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url_png: Option<String>,
    /// The image_url_svg string used to render svg QR code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url_svg: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PixDisplayQrCode {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
