#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct HostedConfirmation {
    /// A custom message to display to the customer after the flow is completed.
    pub custom_message: Option<String>,
}
