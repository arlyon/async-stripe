#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Individual {
    /// The date of birth of this cardholder.
    pub dob: Option<stripe_types::issuing::cardholder::individual::date_of_birth::DateOfBirth>,
    /// The first name of this cardholder.
    pub first_name: String,
    /// The last name of this cardholder.
    pub last_name: String,
    /// Government-issued ID document for this cardholder.
    pub verification:
        Option<stripe_types::issuing::cardholder::individual::verification::Verification>,
}
pub mod date_of_birth;
pub use date_of_birth::DateOfBirth;
pub mod verification;
pub use verification::Verification;
