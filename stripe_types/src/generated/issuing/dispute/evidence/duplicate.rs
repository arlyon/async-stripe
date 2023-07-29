#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Duplicate {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    pub additional_documentation: Option<stripe_types::Expandable<stripe_types::file::File>>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Copy of the card statement showing that the product had already been paid for.
    pub card_statement: Option<stripe_types::Expandable<stripe_types::file::File>>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Copy of the receipt showing that the product had been paid for in cash.
    pub cash_receipt: Option<stripe_types::Expandable<stripe_types::file::File>>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Image of the front and back of the check that was used to pay for the product.
    pub check_image: Option<stripe_types::Expandable<stripe_types::file::File>>,
    /// Explanation of why the cardholder is disputing this transaction.
    pub explanation: Option<String>,
    /// Transaction (e.g., ipi_...) that the disputed transaction is a duplicate of.
    ///
    /// Of the two or more transactions that are copies of each other, this is original undisputed one.
    pub original_transaction: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Duplicate {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
