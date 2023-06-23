#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ServiceNotAsDescribed {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    pub additional_documentation: Option<stripe_types::Expandable<stripe_core::file::File>>,
    /// Date when order was canceled.
    pub canceled_at: Option<stripe_types::Timestamp>,
    /// Reason for canceling the order.
    pub cancellation_reason: Option<String>,
    /// Explanation of why the cardholder is disputing this transaction.
    pub explanation: Option<String>,
    /// Date when the product was received.
    pub received_at: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ServiceNotAsDescribed {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
