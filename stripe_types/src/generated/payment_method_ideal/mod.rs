#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodIdeal {
    /// The customer's bank, if provided.
    ///
    /// Can be one of `abn_amro`, `asn_bank`, `bunq`, `handelsbanken`, `ing`, `knab`, `moneyou`, `rabobank`, `regiobank`, `revolut`, `sns_bank`, `triodos_bank`, `van_lanschot`, or `yoursafe`.
    pub bank: Option<PaymentMethodIdealBank>,
    /// The Bank Identifier Code of the customer's bank, if the bank was provided.
    pub bic: Option<PaymentMethodIdealBic>,
}
/// The customer's bank, if provided.
///
/// Can be one of `abn_amro`, `asn_bank`, `bunq`, `handelsbanken`, `ing`, `knab`, `moneyou`, `rabobank`, `regiobank`, `revolut`, `sns_bank`, `triodos_bank`, `van_lanschot`, or `yoursafe`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodIdealBank {
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
    Yoursafe,
}

impl PaymentMethodIdealBank {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodIdealBank::*;
        match self {
            AbnAmro => "abn_amro",
            AsnBank => "asn_bank",
            Bunq => "bunq",
            Handelsbanken => "handelsbanken",
            Ing => "ing",
            Knab => "knab",
            Moneyou => "moneyou",
            Rabobank => "rabobank",
            Regiobank => "regiobank",
            Revolut => "revolut",
            SnsBank => "sns_bank",
            TriodosBank => "triodos_bank",
            VanLanschot => "van_lanschot",
            Yoursafe => "yoursafe",
        }
    }
}

impl std::str::FromStr for PaymentMethodIdealBank {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodIdealBank::*;
        match s {
            "abn_amro" => Ok(AbnAmro),
            "asn_bank" => Ok(AsnBank),
            "bunq" => Ok(Bunq),
            "handelsbanken" => Ok(Handelsbanken),
            "ing" => Ok(Ing),
            "knab" => Ok(Knab),
            "moneyou" => Ok(Moneyou),
            "rabobank" => Ok(Rabobank),
            "regiobank" => Ok(Regiobank),
            "revolut" => Ok(Revolut),
            "sns_bank" => Ok(SnsBank),
            "triodos_bank" => Ok(TriodosBank),
            "van_lanschot" => Ok(VanLanschot),
            "yoursafe" => Ok(Yoursafe),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentMethodIdealBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodIdealBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodIdealBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentMethodIdealBank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentMethodIdealBank {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for PaymentMethodIdealBank"))
    }
}
/// The Bank Identifier Code of the customer's bank, if the bank was provided.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodIdealBic {
    Abnanl2a,
    Asnbnl21,
    Bitsnl2a,
    Bunqnl2a,
    Fvlbnl22,
    Handnl2a,
    Ingbnl2a,
    Knabnl2h,
    Moyonl21,
    Rabonl2u,
    Rbrbnl21,
    Revoie23,
    Revolt21,
    Snsbnl2a,
    Trionl2u,
}

impl PaymentMethodIdealBic {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodIdealBic::*;
        match self {
            Abnanl2a => "ABNANL2A",
            Asnbnl21 => "ASNBNL21",
            Bitsnl2a => "BITSNL2A",
            Bunqnl2a => "BUNQNL2A",
            Fvlbnl22 => "FVLBNL22",
            Handnl2a => "HANDNL2A",
            Ingbnl2a => "INGBNL2A",
            Knabnl2h => "KNABNL2H",
            Moyonl21 => "MOYONL21",
            Rabonl2u => "RABONL2U",
            Rbrbnl21 => "RBRBNL21",
            Revoie23 => "REVOIE23",
            Revolt21 => "REVOLT21",
            Snsbnl2a => "SNSBNL2A",
            Trionl2u => "TRIONL2U",
        }
    }
}

impl std::str::FromStr for PaymentMethodIdealBic {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodIdealBic::*;
        match s {
            "ABNANL2A" => Ok(Abnanl2a),
            "ASNBNL21" => Ok(Asnbnl21),
            "BITSNL2A" => Ok(Bitsnl2a),
            "BUNQNL2A" => Ok(Bunqnl2a),
            "FVLBNL22" => Ok(Fvlbnl22),
            "HANDNL2A" => Ok(Handnl2a),
            "INGBNL2A" => Ok(Ingbnl2a),
            "KNABNL2H" => Ok(Knabnl2h),
            "MOYONL21" => Ok(Moyonl21),
            "RABONL2U" => Ok(Rabonl2u),
            "RBRBNL21" => Ok(Rbrbnl21),
            "REVOIE23" => Ok(Revoie23),
            "REVOLT21" => Ok(Revolt21),
            "SNSBNL2A" => Ok(Snsbnl2a),
            "TRIONL2U" => Ok(Trionl2u),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentMethodIdealBic {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodIdealBic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodIdealBic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentMethodIdealBic {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentMethodIdealBic {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for PaymentMethodIdealBic"))
    }
}
