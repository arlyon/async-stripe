#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct VerifiedOutputs {
    /// The user's verified address.
    pub address: Option<stripe_types::address::Address>,
    /// The user’s verified date of birth.
    pub dob: Option<
        stripe_misc::identity::verification_session::verified_outputs::date_of_birth::DateOfBirth,
    >,
    /// The user's verified first name.
    pub first_name: Option<String>,
    /// The user's verified id number.
    pub id_number: Option<String>,
    /// The user's verified id number type.
    pub id_number_type: Option<VerifiedOutputsIdNumberType>,
    /// The user's verified last name.
    pub last_name: Option<String>,
}
/// The user's verified id number type.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for VerifiedOutputsIdNumberType {
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
impl serde::Serialize for VerifiedOutputsIdNumberType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for VerifiedOutputsIdNumberType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for VerifiedOutputsIdNumberType"))
    }
}
pub mod date_of_birth;
pub use date_of_birth::DateOfBirth;
