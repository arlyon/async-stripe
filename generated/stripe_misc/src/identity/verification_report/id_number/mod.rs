/// Result from an id_number check.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IdNumber {
    /// Date of birth.
pub dob: Option<stripe_misc::identity::verification_report::id_number::date_of_birth::DateOfBirth>,
    /// Details on the verification error.
    ///
    /// Present when status is `unverified`.
pub error: Option<stripe_misc::identity::verification_report::id_number::id_number_check_error::IdNumberCheckError>,
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
/// Type of ID number.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum IdNumberIdNumberType {
    BrCpf,
    SgNric,
    UsSsn,
}

impl IdNumberIdNumberType {
    pub fn as_str(self) -> &'static str {
        use IdNumberIdNumberType::*;
        match self {
            BrCpf => "br_cpf",
            SgNric => "sg_nric",
            UsSsn => "us_ssn",
        }
    }
}

impl std::str::FromStr for IdNumberIdNumberType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IdNumberIdNumberType::*;
        match s {
            "br_cpf" => Ok(BrCpf),
            "sg_nric" => Ok(SgNric),
            "us_ssn" => Ok(UsSsn),
            _ => Err(()),
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
impl serde::Serialize for IdNumberIdNumberType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IdNumberIdNumberType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for IdNumberIdNumberType"))
    }
}
/// Status of this `id_number` check.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum IdNumberStatus {
    Unverified,
    Verified,
}

impl IdNumberStatus {
    pub fn as_str(self) -> &'static str {
        use IdNumberStatus::*;
        match self {
            Unverified => "unverified",
            Verified => "verified",
        }
    }
}

impl std::str::FromStr for IdNumberStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IdNumberStatus::*;
        match s {
            "unverified" => Ok(Unverified),
            "verified" => Ok(Verified),
            _ => Err(()),
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
impl serde::Serialize for IdNumberStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IdNumberStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for IdNumberStatus"))
    }
}
pub mod date_of_birth;
pub use date_of_birth::DateOfBirth;
pub mod id_number_check_error;
pub use id_number_check_error::IdNumberCheckError;
