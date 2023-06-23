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
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
/// The customer's bank, if provided.
///
/// Can be one of `affin_bank`, `agrobank`, `alliance_bank`, `ambank`, `bank_islam`, `bank_muamalat`, `bank_rakyat`, `bsn`, `cimb`, `hong_leong_bank`, `hsbc`, `kfh`, `maybank2u`, `ocbc`, `public_bank`, `rhb`, `standard_chartered`, `uob`, `deutsche_bank`, `maybank2e`, `pb_enterprise`, or `bank_of_china`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
