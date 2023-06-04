#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Individual {
    /// The date of birth of this cardholder.
    pub dob: Option<crate::issuing::cardholder::individual::date_of_birth::DateOfBirth>,
    /// The first name of this cardholder.
    pub first_name: String,
    /// The last name of this cardholder.
    pub last_name: String,
    /// Government-issued ID document for this cardholder.
    pub verification: Option<crate::issuing::cardholder::individual::verification::Verification>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Individual {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

pub mod date_of_birth;
pub use date_of_birth::DateOfBirth;
pub mod verification;
pub use verification::Verification;
