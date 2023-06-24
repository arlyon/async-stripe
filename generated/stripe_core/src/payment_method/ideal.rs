#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Ideal {
    /// The customer's bank, if provided.
    ///
    /// Can be one of `abn_amro`, `asn_bank`, `bunq`, `handelsbanken`, `ing`, `knab`, `moneyou`, `rabobank`, `regiobank`, `revolut`, `sns_bank`, `triodos_bank`, or `van_lanschot`.
    pub bank: Option<IdealBank>,
    /// The Bank Identifier Code of the customer's bank, if the bank was provided.
    pub bic: Option<IdealBic>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Ideal {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The customer's bank, if provided.
///
/// Can be one of `abn_amro`, `asn_bank`, `bunq`, `handelsbanken`, `ing`, `knab`, `moneyou`, `rabobank`, `regiobank`, `revolut`, `sns_bank`, `triodos_bank`, or `van_lanschot`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum IdealBank {
    AbnAmro,
    AsnBank,
    Bunq,
    Handelsbanken,
    Ing,
    Knab,
    Moneyou,
    Rabobank,
    Regiobank,
    Revolut,
    SnsBank,
    TriodosBank,
    VanLanschot,
}

impl IdealBank {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AbnAmro => "abn_amro",
            Self::AsnBank => "asn_bank",
            Self::Bunq => "bunq",
            Self::Handelsbanken => "handelsbanken",
            Self::Ing => "ing",
            Self::Knab => "knab",
            Self::Moneyou => "moneyou",
            Self::Rabobank => "rabobank",
            Self::Regiobank => "regiobank",
            Self::Revolut => "revolut",
            Self::SnsBank => "sns_bank",
            Self::TriodosBank => "triodos_bank",
            Self::VanLanschot => "van_lanschot",
        }
    }
}

impl std::str::FromStr for IdealBank {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "abn_amro" => Ok(Self::AbnAmro),
            "asn_bank" => Ok(Self::AsnBank),
            "bunq" => Ok(Self::Bunq),
            "handelsbanken" => Ok(Self::Handelsbanken),
            "ing" => Ok(Self::Ing),
            "knab" => Ok(Self::Knab),
            "moneyou" => Ok(Self::Moneyou),
            "rabobank" => Ok(Self::Rabobank),
            "regiobank" => Ok(Self::Regiobank),
            "revolut" => Ok(Self::Revolut),
            "sns_bank" => Ok(Self::SnsBank),
            "triodos_bank" => Ok(Self::TriodosBank),
            "van_lanschot" => Ok(Self::VanLanschot),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for IdealBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IdealBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for IdealBank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IdealBank {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for IdealBank"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for IdealBank {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<IdealBank> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IdealBank::from_str(s)?);
        Ok(())
    }
}
/// The Bank Identifier Code of the customer's bank, if the bank was provided.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum IdealBic {
    Abnanl2a,
    Asnbnl21,
    Bunqnl2a,
    Fvlbnl22,
    Handnl2a,
    Ingbnl2a,
    Knabnl2h,
    Moyonl21,
    Rabonl2u,
    Rbrbnl21,
    Revolt21,
    Snsbnl2a,
    Trionl2u,
}

impl IdealBic {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Abnanl2a => "ABNANL2A",
            Self::Asnbnl21 => "ASNBNL21",
            Self::Bunqnl2a => "BUNQNL2A",
            Self::Fvlbnl22 => "FVLBNL22",
            Self::Handnl2a => "HANDNL2A",
            Self::Ingbnl2a => "INGBNL2A",
            Self::Knabnl2h => "KNABNL2H",
            Self::Moyonl21 => "MOYONL21",
            Self::Rabonl2u => "RABONL2U",
            Self::Rbrbnl21 => "RBRBNL21",
            Self::Revolt21 => "REVOLT21",
            Self::Snsbnl2a => "SNSBNL2A",
            Self::Trionl2u => "TRIONL2U",
        }
    }
}

impl std::str::FromStr for IdealBic {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ABNANL2A" => Ok(Self::Abnanl2a),
            "ASNBNL21" => Ok(Self::Asnbnl21),
            "BUNQNL2A" => Ok(Self::Bunqnl2a),
            "FVLBNL22" => Ok(Self::Fvlbnl22),
            "HANDNL2A" => Ok(Self::Handnl2a),
            "INGBNL2A" => Ok(Self::Ingbnl2a),
            "KNABNL2H" => Ok(Self::Knabnl2h),
            "MOYONL21" => Ok(Self::Moyonl21),
            "RABONL2U" => Ok(Self::Rabonl2u),
            "RBRBNL21" => Ok(Self::Rbrbnl21),
            "REVOLT21" => Ok(Self::Revolt21),
            "SNSBNL2A" => Ok(Self::Snsbnl2a),
            "TRIONL2U" => Ok(Self::Trionl2u),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for IdealBic {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IdealBic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for IdealBic {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IdealBic {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for IdealBic"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for IdealBic {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<IdealBic> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IdealBic::from_str(s)?);
        Ok(())
    }
}
