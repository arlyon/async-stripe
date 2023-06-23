#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Verification {
    /// A document showing address, either a passport, local ID card, or utility bill from a well-known utility company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_document:
        Option<stripe_core::person::verification_document::VerificationDocument>,
    /// A user-displayable string describing the verification state for the person.
    ///
    /// For example, this may say "Provided identity information could not be verified".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// One of `document_address_mismatch`, `document_dob_mismatch`, `document_duplicate_type`, `document_id_number_mismatch`, `document_name_mismatch`, `document_nationality_mismatch`, `failed_keyed_identity`, or `failed_other`.
    ///
    /// A machine-readable code specifying the verification state for the person.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<stripe_core::person::verification_document::VerificationDocument>,
    /// The state of verification for the person.
    ///
    /// Possible values are `unverified`, `pending`, or `verified`.
    pub status: String,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Verification {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
