#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Fraudulent {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    pub additional_documentation: Option<stripe_types::Expandable<stripe_core::file::File>>,
    /// Explanation of why the cardholder is disputing this transaction.
    pub explanation: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Fraudulent {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
