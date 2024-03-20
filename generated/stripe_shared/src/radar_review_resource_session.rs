#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct RadarReviewResourceSession {
    /// The browser used in this browser session (e.g., `Chrome`).
    pub browser: Option<String>,
    /// Information about the device used for the browser session (e.g., `Samsung SM-G930T`).
    pub device: Option<String>,
    /// The platform for the browser session (e.g., `Macintosh`).
    pub platform: Option<String>,
    /// The version for the browser session (e.g., `61.0.3163.100`).
    pub version: Option<String>,
}
