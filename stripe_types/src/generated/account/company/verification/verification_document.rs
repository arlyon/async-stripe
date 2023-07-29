#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct VerificationDocument {
    /// The back of a document returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `additional_verification`.
    pub back: Option<stripe_types::Expandable<stripe_types::file::File>>,
    /// A user-displayable string describing the verification state of this document.
    pub details: Option<String>,
    /// One of `document_corrupt`, `document_expired`, `document_failed_copy`, `document_failed_greyscale`, `document_failed_other`, `document_failed_test_mode`, `document_fraudulent`, `document_incomplete`, `document_invalid`, `document_manipulated`, `document_not_readable`, `document_not_uploaded`, `document_type_not_supported`, or `document_too_large`.
    ///
    /// A machine-readable code specifying the verification state for this document.
    pub details_code: Option<String>,
    /// The front of a document returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `additional_verification`.
    pub front: Option<stripe_types::Expandable<stripe_types::file::File>>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for VerificationDocument {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
