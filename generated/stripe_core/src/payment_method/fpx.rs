#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Fpx {
    /// Account holder type, if provided.
    ///
    /// Can be one of `individual` or `company`.
    pub account_holder_type: Option<FpxAccountHolderType>,
    /// The customer's bank, if provided.
    ///
    /// Can be one of `affin_bank`, `agrobank`, `alliance_bank`, `ambank`, `bank_islam`, `bank_muamalat`, `bank_rakyat`, `bsn`, `cimb`, `hong_leong_bank`, `hsbc`, `kfh`, `maybank2u`, `ocbc`, `public_bank`, `rhb`, `standard_chartered`, `uob`, `deutsche_bank`, `maybank2e`, `pb_enterprise`, or `bank_of_china`.
    pub bank: FpxBank,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Fpx {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Account holder type, if provided.
///
/// Can be one of `individual` or `company`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FpxAccountHolderType {
    Company,
    Individual,
}

impl FpxAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Company => "company",
            Self::Individual => "individual",
        }
    }
}

impl std::str::FromStr for FpxAccountHolderType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "company" => Ok(Self::Company),
            "individual" => Ok(Self::Individual),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for FpxAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FpxAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for FpxAccountHolderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FpxAccountHolderType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for FpxAccountHolderType"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for FpxAccountHolderType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<FpxAccountHolderType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(FpxAccountHolderType::from_str(s)?);
        Ok(())
    }
}
/// The customer's bank, if provided.
///
/// Can be one of `affin_bank`, `agrobank`, `alliance_bank`, `ambank`, `bank_islam`, `bank_muamalat`, `bank_rakyat`, `bsn`, `cimb`, `hong_leong_bank`, `hsbc`, `kfh`, `maybank2u`, `ocbc`, `public_bank`, `rhb`, `standard_chartered`, `uob`, `deutsche_bank`, `maybank2e`, `pb_enterprise`, or `bank_of_china`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FpxBank {
    AffinBank,
    Agrobank,
    AllianceBank,
    Ambank,
    BankIslam,
    BankMuamalat,
    BankOfChina,
    BankRakyat,
    Bsn,
    Cimb,
    DeutscheBank,
    HongLeongBank,
    Hsbc,
    Kfh,
    Maybank2e,
    Maybank2u,
    Ocbc,
    PbEnterprise,
    PublicBank,
    Rhb,
    StandardChartered,
    Uob,
}

impl FpxBank {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AffinBank => "affin_bank",
            Self::Agrobank => "agrobank",
            Self::AllianceBank => "alliance_bank",
            Self::Ambank => "ambank",
            Self::BankIslam => "bank_islam",
            Self::BankMuamalat => "bank_muamalat",
            Self::BankOfChina => "bank_of_china",
            Self::BankRakyat => "bank_rakyat",
            Self::Bsn => "bsn",
            Self::Cimb => "cimb",
            Self::DeutscheBank => "deutsche_bank",
            Self::HongLeongBank => "hong_leong_bank",
            Self::Hsbc => "hsbc",
            Self::Kfh => "kfh",
            Self::Maybank2e => "maybank2e",
            Self::Maybank2u => "maybank2u",
            Self::Ocbc => "ocbc",
            Self::PbEnterprise => "pb_enterprise",
            Self::PublicBank => "public_bank",
            Self::Rhb => "rhb",
            Self::StandardChartered => "standard_chartered",
            Self::Uob => "uob",
        }
    }
}

impl std::str::FromStr for FpxBank {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "affin_bank" => Ok(Self::AffinBank),
            "agrobank" => Ok(Self::Agrobank),
            "alliance_bank" => Ok(Self::AllianceBank),
            "ambank" => Ok(Self::Ambank),
            "bank_islam" => Ok(Self::BankIslam),
            "bank_muamalat" => Ok(Self::BankMuamalat),
            "bank_of_china" => Ok(Self::BankOfChina),
            "bank_rakyat" => Ok(Self::BankRakyat),
            "bsn" => Ok(Self::Bsn),
            "cimb" => Ok(Self::Cimb),
            "deutsche_bank" => Ok(Self::DeutscheBank),
            "hong_leong_bank" => Ok(Self::HongLeongBank),
            "hsbc" => Ok(Self::Hsbc),
            "kfh" => Ok(Self::Kfh),
            "maybank2e" => Ok(Self::Maybank2e),
            "maybank2u" => Ok(Self::Maybank2u),
            "ocbc" => Ok(Self::Ocbc),
            "pb_enterprise" => Ok(Self::PbEnterprise),
            "public_bank" => Ok(Self::PublicBank),
            "rhb" => Ok(Self::Rhb),
            "standard_chartered" => Ok(Self::StandardChartered),
            "uob" => Ok(Self::Uob),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for FpxBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FpxBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for FpxBank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FpxBank {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for FpxBank"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for FpxBank {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<FpxBank> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(FpxBank::from_str(s)?);
        Ok(())
    }
}
