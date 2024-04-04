#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct RefundDestinationDetailsGeneric {
    /// The reference assigned to the refund.
    pub reference: Option<String>,
    /// Status of the reference on the refund. This can be `pending`, `available` or `unavailable`.
    pub reference_status: Option<String>,
}
