#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct GelatoVerifiedOutputs {
    /// The user's verified address.
    pub address: Option<stripe_types::Address>,
    /// The user’s verified date of birth.
    pub dob: Option<stripe_misc::GelatoDataVerifiedOutputsDate>,
    /// The user's verified first name.
    pub first_name: Option<String>,
    /// The user's verified id number.
    pub id_number: Option<String>,
    /// The user's verified id number type.
    pub id_number_type: Option<GelatoVerifiedOutputsIdNumberType>,
    /// The user's verified last name.
    pub last_name: Option<String>,
}
/// The user's verified id number type.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GelatoVerifiedOutputsIdNumberType {
    BrCpf,
    SgNric,
    UsSsn,
}

impl GelatoVerifiedOutputsIdNumberType {
    pub fn as_str(self) -> &'static str {
        use GelatoVerifiedOutputsIdNumberType::*;
        match self {
            BrCpf => "br_cpf",
            SgNric => "sg_nric",
            UsSsn => "us_ssn",
        }
    }
}

impl std::str::FromStr for GelatoVerifiedOutputsIdNumberType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GelatoVerifiedOutputsIdNumberType::*;
        match s {
            "br_cpf" => Ok(BrCpf),
            "sg_nric" => Ok(SgNric),
            "us_ssn" => Ok(UsSsn),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for GelatoVerifiedOutputsIdNumberType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GelatoVerifiedOutputsIdNumberType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for GelatoVerifiedOutputsIdNumberType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for GelatoVerifiedOutputsIdNumberType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GelatoVerifiedOutputsIdNumberType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for GelatoVerifiedOutputsIdNumberType"))
    }
}
