#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct AcssDebit {
    /// Currency supported by the bank account.
pub currency: Option<AcssDebitCurrency>,
#[serde(skip_serializing_if = "Option::is_none")]
pub mandate_options: Option<stripe_core::setup_intent::payment_method_options::acss_debit::mandate_options::MandateOptions>,
    /// Bank account verification method.
#[serde(skip_serializing_if = "Option::is_none")]
pub verification_method: Option<AcssDebitVerificationMethod>,

}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for AcssDebit {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Currency supported by the bank account.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AcssDebitCurrency {
    Cad,
    Usd,
}

impl AcssDebitCurrency {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cad => "cad",
            Self::Usd => "usd",
        }
    }
}

impl std::str::FromStr for AcssDebitCurrency {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cad" => Ok(Self::Cad),
            "usd" => Ok(Self::Usd),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for AcssDebitCurrency {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AcssDebitCurrency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for AcssDebitCurrency {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AcssDebitCurrency {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for AcssDebitCurrency"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for AcssDebitCurrency {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<AcssDebitCurrency> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(AcssDebitCurrency::from_str(s)?);
        Ok(())
    }
}
/// Bank account verification method.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl AcssDebitVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Instant => "instant",
            Self::Microdeposits => "microdeposits",
        }
    }
}

impl std::str::FromStr for AcssDebitVerificationMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "automatic" => Ok(Self::Automatic),
            "instant" => Ok(Self::Instant),
            "microdeposits" => Ok(Self::Microdeposits),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for AcssDebitVerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AcssDebitVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for AcssDebitVerificationMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AcssDebitVerificationMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for AcssDebitVerificationMethod"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for AcssDebitVerificationMethod {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<AcssDebitVerificationMethod> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(AcssDebitVerificationMethod::from_str(s)?);
        Ok(())
    }
}
pub mod mandate_options;
pub use mandate_options::MandateOptions;
