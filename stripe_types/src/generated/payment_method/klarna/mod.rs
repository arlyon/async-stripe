#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Klarna {
    /// The customer's date of birth, if provided.
    pub dob: Option<stripe_types::payment_method::klarna::date_of_birth::DateOfBirth>,
}
pub mod date_of_birth;
pub use date_of_birth::DateOfBirth;
