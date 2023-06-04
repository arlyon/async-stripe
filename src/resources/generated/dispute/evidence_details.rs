#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct EvidenceDetails {
    /// Date by which evidence must be submitted in order to successfully challenge dispute.
    ///
    /// Will be null if the customer's bank or credit card company doesn't allow a response for this particular dispute.
    pub due_by: Option<crate::Timestamp>,
    /// Whether evidence has been staged for this dispute.
    pub has_evidence: bool,
    /// Whether the last evidence submission was submitted past the due date.
    ///
    /// Defaults to `false` if no evidence submissions have occurred.
    /// If `true`, then delivery of the latest evidence is *not* guaranteed.
    pub past_due: bool,
    /// The number of times evidence has been submitted.
    ///
    /// Typically, you may only submit evidence once.
    pub submission_count: u64,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for EvidenceDetails {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
