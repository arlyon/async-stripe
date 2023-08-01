#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Multibanco {
    /// Entity number associated with this Multibanco payment.
    pub entity: Option<String>,
    /// Reference number associated with this Multibanco payment.
    pub reference: Option<String>,
}
