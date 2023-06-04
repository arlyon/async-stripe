/// Result from an id_number check.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct IdNumber {
    /// Date of birth.
    pub dob: Option<crate::identity::verification_report::id_number::date_of_birth::DateOfBirth>,
    /// Details on the verification error.
    ///
    /// Present when status is `unverified`.
    pub error: Option<
        crate::identity::verification_report::id_number::id_number_check_error::IdNumberCheckError,
    >,
    /// First name.
    pub first_name: Option<String>,
    /// ID number.
    pub id_number: Option<String>,
    /// Type of ID number.
    pub id_number_type: Option<IdNumberIdNumberType>,
    /// Last name.
    pub last_name: Option<String>,
    /// Status of this `id_number` check.
    pub status: IdNumberStatus,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for IdNumber {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Type of ID number.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum IdNumberIdNumberType {
    BrCpf,
    SgNric,
    UsSsn,
}

impl IdNumberIdNumberType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BrCpf => "br_cpf",
            Self::SgNric => "sg_nric",
            Self::UsSsn => "us_ssn",
        }
    }
}

impl AsRef<str> for IdNumberIdNumberType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IdNumberIdNumberType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Status of this `id_number` check.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum IdNumberStatus {
    Unverified,
    Verified,
}

impl IdNumberStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Unverified => "unverified",
            Self::Verified => "verified",
        }
    }
}

impl AsRef<str> for IdNumberStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IdNumberStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
pub mod date_of_birth;
pub use date_of_birth::DateOfBirth;
pub mod id_number_check_error;
pub use id_number_check_error::IdNumberCheckError;
