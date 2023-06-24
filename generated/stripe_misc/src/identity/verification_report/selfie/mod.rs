/// Result from a selfie check.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Selfie {
    /// ID of the [File](https://stripe.com/docs/api/files) holding the image of the identity document used in this check.
    pub document: Option<String>,
    /// Details on the verification error.
    ///
    /// Present when status is `unverified`.
    pub error: Option<
        stripe_misc::identity::verification_report::selfie::selfie_check_error::SelfieCheckError,
    >,
    /// ID of the [File](https://stripe.com/docs/api/files) holding the image of the selfie used in this check.
    pub selfie: Option<String>,
    /// Status of this `selfie` check.
    pub status: SelfieStatus,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Selfie {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Status of this `selfie` check.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SelfieStatus {
    Unverified,
    Verified,
}

impl SelfieStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Unverified => "unverified",
            Self::Verified => "verified",
        }
    }
}

impl std::str::FromStr for SelfieStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "unverified" => Ok(Self::Unverified),
            "verified" => Ok(Self::Verified),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for SelfieStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SelfieStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for SelfieStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SelfieStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for SelfieStatus"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SelfieStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<SelfieStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SelfieStatus::from_str(s)?);
        Ok(())
    }
}
pub mod selfie_check_error;
pub use selfie_check_error::SelfieCheckError;
