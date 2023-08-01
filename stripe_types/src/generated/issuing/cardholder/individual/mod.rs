#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Individual {
    /// Information related to the card_issuing program for this cardholder.
    pub card_issuing:
        Option<stripe_types::issuing::cardholder::individual::card_issuing::CardIssuing>,
    /// The date of birth of this cardholder.
    pub dob: Option<stripe_types::issuing::cardholder::individual::date_of_birth::DateOfBirth>,
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
    pub verification:
        Option<stripe_types::issuing::cardholder::individual::verification::Verification>,
}
pub mod card_issuing;
pub use card_issuing::CardIssuing;
pub mod date_of_birth;
pub use date_of_birth::DateOfBirth;
pub mod verification;
pub use verification::Verification;
