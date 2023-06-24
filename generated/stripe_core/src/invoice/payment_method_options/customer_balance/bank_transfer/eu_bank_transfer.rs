#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct EuBankTransfer {
    /// The desired country code of the bank account information.
    ///
    /// Permitted values include: `DE`, `ES`, `FR`, `IE`, or `NL`.
    pub country: EuBankTransferCountry,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for EuBankTransfer {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The desired country code of the bank account information.
///
/// Permitted values include: `DE`, `ES`, `FR`, `IE`, or `NL`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum EuBankTransferCountry {
    De,
    Es,
    Fr,
    Ie,
    Nl,
}

impl EuBankTransferCountry {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::De => "DE",
            Self::Es => "ES",
            Self::Fr => "FR",
            Self::Ie => "IE",
            Self::Nl => "NL",
        }
    }
}

impl std::str::FromStr for EuBankTransferCountry {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DE" => Ok(Self::De),
            "ES" => Ok(Self::Es),
            "FR" => Ok(Self::Fr),
            "IE" => Ok(Self::Ie),
            "NL" => Ok(Self::Nl),

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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for EuBankTransferCountry"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for EuBankTransferCountry {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<EuBankTransferCountry> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(EuBankTransferCountry::from_str(s)?);
        Ok(())
    }
}
