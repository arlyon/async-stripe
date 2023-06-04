#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Klarna {
    /// The customer's date of birth, if provided.
    pub dob: Option<crate::payment_method::klarna::date_of_birth::DateOfBirth>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Klarna {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

pub mod date_of_birth;
pub use date_of_birth::DateOfBirth;
