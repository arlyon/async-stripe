#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct VerifiedOutputs {
    /// The user's verified address.
    pub address: Option<crate::address::Address>,
    /// The user’s verified date of birth.
    pub dob:
        Option<crate::identity::verification_session::verified_outputs::date_of_birth::DateOfBirth>,
    /// The user's verified first name.
    pub first_name: Option<String>,
    /// The user's verified id number.
    pub id_number: Option<String>,
    /// The user's verified id number type.
    pub id_number_type: Option<VerifiedOutputsIdNumberType>,
    /// The user's verified last name.
    pub last_name: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for VerifiedOutputs {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The user's verified id number type.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum VerifiedOutputsIdNumberType {
    BrCpf,
    SgNric,
    UsSsn,
}

impl VerifiedOutputsIdNumberType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BrCpf => "br_cpf",
            Self::SgNric => "sg_nric",
            Self::UsSsn => "us_ssn",
        }
    }
}

impl AsRef<str> for VerifiedOutputsIdNumberType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for VerifiedOutputsIdNumberType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
pub mod date_of_birth;
pub use date_of_birth::DateOfBirth;
