/// Result from an id_number check.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
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
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for IdNumber {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
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
        match self {
            Self::BrCpf => "br_cpf",
            Self::SgNric => "sg_nric",
            Self::UsSsn => "us_ssn",
        }
    }
}

impl std::str::FromStr for IdNumberIdNumberType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "br_cpf" => Ok(Self::BrCpf),
            "sg_nric" => Ok(Self::SgNric),
            "us_ssn" => Ok(Self::UsSsn),

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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for IdNumberIdNumberType"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for IdNumberIdNumberType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<IdNumberIdNumberType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IdNumberIdNumberType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
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
        match self {
            Self::Unverified => "unverified",
            Self::Verified => "verified",
        }
    }
}

impl std::str::FromStr for IdNumberStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "unverified" => Ok(Self::Unverified),
            "verified" => Ok(Self::Verified),

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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for IdNumberStatus"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for IdNumberStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<IdNumberStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IdNumberStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
pub mod date_of_birth;
pub use date_of_birth::DateOfBirth;
pub mod id_number_check_error;
pub use id_number_check_error::IdNumberCheckError;
