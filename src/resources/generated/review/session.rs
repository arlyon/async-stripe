#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Session {
    /// The browser used in this browser session (e.g., `Chrome`).
    pub browser: Option<String>,
    /// Information about the device used for the browser session (e.g., `Samsung SM-G930T`).
    pub device: Option<String>,
    /// The platform for the browser session (e.g., `Macintosh`).
    pub platform: Option<String>,
    /// The version for the browser session (e.g., `61.0.3163.100`).
    pub version: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Session {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
