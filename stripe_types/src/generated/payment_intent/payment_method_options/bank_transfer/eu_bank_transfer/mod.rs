#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct EuBankTransfer {
    /// The desired country code of the bank account information.
    ///
    /// Permitted values include: `BE`, `DE`, `ES`, `FR`, `IE`, or `NL`.
    pub country: EuBankTransferCountry,
}
/// The desired country code of the bank account information.
///
/// Permitted values include: `BE`, `DE`, `ES`, `FR`, `IE`, or `NL`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum EuBankTransferCountry {
    Be,
    De,
    Es,
    Fr,
    Ie,
    Nl,
}

impl EuBankTransferCountry {
    pub fn as_str(self) -> &'static str {
        use EuBankTransferCountry::*;
        match self {
            Be => "BE",
            De => "DE",
            Es => "ES",
            Fr => "FR",
            Ie => "IE",
            Nl => "NL",
        }
    }
}

impl std::str::FromStr for EuBankTransferCountry {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use EuBankTransferCountry::*;
        match s {
            "BE" => Ok(Be),
            "DE" => Ok(De),
            "ES" => Ok(Es),
            "FR" => Ok(Fr),
            "IE" => Ok(Ie),
            "NL" => Ok(Nl),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for EuBankTransferCountry {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for EuBankTransferCountry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for EuBankTransferCountry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for EuBankTransferCountry {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for EuBankTransferCountry"))
    }
}
