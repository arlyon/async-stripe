/// Result from an id_number check.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GelatoIdNumberReport {
    /// Date of birth.
    pub dob: Option<stripe_misc::GelatoDataIdNumberReportDate>,
    /// Details on the verification error.
    ///
    /// Present when status is `unverified`.
    pub error: Option<stripe_misc::GelatoIdNumberReportError>,
    /// First name.
    pub first_name: Option<String>,
    /// ID number.
    pub id_number: Option<String>,
    /// Type of ID number.
    pub id_number_type: Option<GelatoIdNumberReportIdNumberType>,
    /// Last name.
    pub last_name: Option<String>,
    /// Status of this `id_number` check.
    pub status: GelatoIdNumberReportStatus,
}
/// Type of ID number.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GelatoIdNumberReportIdNumberType {
    BrCpf,
    SgNric,
    UsSsn,
}

impl GelatoIdNumberReportIdNumberType {
    pub fn as_str(self) -> &'static str {
        use GelatoIdNumberReportIdNumberType::*;
        match self {
            BrCpf => "br_cpf",
            SgNric => "sg_nric",
            UsSsn => "us_ssn",
        }
    }
}

impl std::str::FromStr for GelatoIdNumberReportIdNumberType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GelatoIdNumberReportIdNumberType::*;
        match s {
            "br_cpf" => Ok(BrCpf),
            "sg_nric" => Ok(SgNric),
            "us_ssn" => Ok(UsSsn),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for GelatoIdNumberReportIdNumberType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GelatoIdNumberReportIdNumberType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for GelatoIdNumberReportIdNumberType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for GelatoIdNumberReportIdNumberType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GelatoIdNumberReportIdNumberType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for GelatoIdNumberReportIdNumberType")
        })
    }
}
/// Status of this `id_number` check.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GelatoIdNumberReportStatus {
    Unverified,
    Verified,
}

impl GelatoIdNumberReportStatus {
    pub fn as_str(self) -> &'static str {
        use GelatoIdNumberReportStatus::*;
        match self {
            Unverified => "unverified",
            Verified => "verified",
        }
    }
}

impl std::str::FromStr for GelatoIdNumberReportStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GelatoIdNumberReportStatus::*;
        match s {
            "unverified" => Ok(Unverified),
            "verified" => Ok(Verified),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for GelatoIdNumberReportStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GelatoIdNumberReportStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for GelatoIdNumberReportStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for GelatoIdNumberReportStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GelatoIdNumberReportStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for GelatoIdNumberReportStatus"))
    }
}
