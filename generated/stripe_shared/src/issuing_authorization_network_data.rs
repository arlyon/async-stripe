#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct IssuingAuthorizationNetworkData {
    /// Identifier assigned to the acquirer by the card network.
    /// Sometimes this value is not provided by the network; in this case, the value will be `null`.
    pub acquiring_institution_id: Option<String>,
    /// The System Trace Audit Number (STAN) is a 6-digit identifier assigned by the acquirer.
    /// Prefer `network_data.transaction_id` if present, unless you have special requirements.
    pub system_trace_audit_number: Option<String>,
    /// Unique identifier for the authorization assigned by the card network used to match subsequent messages, disputes, and transactions.
    pub transaction_id: Option<String>,
}
