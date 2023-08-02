#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Individual {
    /// Information related to the card_issuing program for this cardholder.
    pub card_issuing: Option<stripe_types::card_issuing::CardIssuing>,
    /// The date of birth of this cardholder.
    pub dob: Option<stripe_types::date_of_birth::DateOfBirth>,
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
    pub verification: Option<stripe_types::verification::Verification>,
}
