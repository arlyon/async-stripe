#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct IssuingCardholderIndividual {
    /// Information related to the card_issuing program for this cardholder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<stripe_types::IssuingCardholderCardIssuing>,
    /// The date of birth of this cardholder.
    pub dob: Option<stripe_types::IssuingCardholderIndividualDob>,
    /// The first name of this cardholder.
    ///
    /// Required before activating Cards.
    /// This field cannot contain any numbers, special characters (except periods, commas, hyphens, spaces and apostrophes) or non-latin letters.
    pub first_name: Option<String>,
    /// The last name of this cardholder.
    ///
    /// Required before activating Cards.
    /// This field cannot contain any numbers, special characters (except periods, commas, hyphens, spaces and apostrophes) or non-latin letters.
    pub last_name: Option<String>,
    /// Government-issued ID document for this cardholder.
    pub verification: Option<stripe_types::IssuingCardholderVerification>,
}
