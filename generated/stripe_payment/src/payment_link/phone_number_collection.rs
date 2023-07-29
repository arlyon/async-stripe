#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PhoneNumberCollection {
    /// If `true`, a phone number will be collected during checkout.
    pub enabled: bool,
}
