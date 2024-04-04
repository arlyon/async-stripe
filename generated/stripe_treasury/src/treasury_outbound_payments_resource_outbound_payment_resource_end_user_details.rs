#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TreasuryOutboundPaymentsResourceOutboundPaymentResourceEndUserDetails {
    /// IP address of the user initiating the OutboundPayment.
    /// Set if `present` is set to `true`.
    /// IP address collection is required for risk and compliance reasons.
    /// This will be used to help determine if the OutboundPayment is authorized or should be blocked.
    pub ip_address: Option<String>,
    /// `true` if the OutboundPayment creation request is being made on behalf of an end user by a platform.
    /// Otherwise, `false`.
    pub present: bool,
}
