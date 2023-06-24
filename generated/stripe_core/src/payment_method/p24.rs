#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct P24 {
    /// The customer's bank, if provided.
    pub bank: Option<P24Bank>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for P24 {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The customer's bank, if provided.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum P24Bank {
    AliorBank,
    BankMillennium,
    BankNowyBfgSa,
    BankPekaoSa,
    BankiSpbdzielcze,
    Blik,
    BnpParibas,
    Boz,
    CitiHandlowy,
    CreditAgricole,
    Envelobank,
    EtransferPocztowy24,
    GetinBank,
    Ideabank,
    Ing,
    Inteligo,
    MbankMtransfer,
    NestPrzelew,
    NoblePay,
    PbacZIpko,
    PlusBank,
    SantanderPrzelew24,
    TmobileUsbugiBankowe,
    ToyotaBank,
    VolkswagenBank,
}

impl P24Bank {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AliorBank => "alior_bank",
            Self::BankMillennium => "bank_millennium",
            Self::BankNowyBfgSa => "bank_nowy_bfg_sa",
            Self::BankPekaoSa => "bank_pekao_sa",
            Self::BankiSpbdzielcze => "banki_spbdzielcze",
            Self::Blik => "blik",
            Self::BnpParibas => "bnp_paribas",
            Self::Boz => "boz",
            Self::CitiHandlowy => "citi_handlowy",
            Self::CreditAgricole => "credit_agricole",
            Self::Envelobank => "envelobank",
            Self::EtransferPocztowy24 => "etransfer_pocztowy24",
            Self::GetinBank => "getin_bank",
            Self::Ideabank => "ideabank",
            Self::Ing => "ing",
            Self::Inteligo => "inteligo",
            Self::MbankMtransfer => "mbank_mtransfer",
            Self::NestPrzelew => "nest_przelew",
            Self::NoblePay => "noble_pay",
            Self::PbacZIpko => "pbac_z_ipko",
            Self::PlusBank => "plus_bank",
            Self::SantanderPrzelew24 => "santander_przelew24",
            Self::TmobileUsbugiBankowe => "tmobile_usbugi_bankowe",
            Self::ToyotaBank => "toyota_bank",
            Self::VolkswagenBank => "volkswagen_bank",
        }
    }
}

impl std::str::FromStr for P24Bank {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "alior_bank" => Ok(Self::AliorBank),
            "bank_millennium" => Ok(Self::BankMillennium),
            "bank_nowy_bfg_sa" => Ok(Self::BankNowyBfgSa),
            "bank_pekao_sa" => Ok(Self::BankPekaoSa),
            "banki_spbdzielcze" => Ok(Self::BankiSpbdzielcze),
            "blik" => Ok(Self::Blik),
            "bnp_paribas" => Ok(Self::BnpParibas),
            "boz" => Ok(Self::Boz),
            "citi_handlowy" => Ok(Self::CitiHandlowy),
            "credit_agricole" => Ok(Self::CreditAgricole),
            "envelobank" => Ok(Self::Envelobank),
            "etransfer_pocztowy24" => Ok(Self::EtransferPocztowy24),
            "getin_bank" => Ok(Self::GetinBank),
            "ideabank" => Ok(Self::Ideabank),
            "ing" => Ok(Self::Ing),
            "inteligo" => Ok(Self::Inteligo),
            "mbank_mtransfer" => Ok(Self::MbankMtransfer),
            "nest_przelew" => Ok(Self::NestPrzelew),
            "noble_pay" => Ok(Self::NoblePay),
            "pbac_z_ipko" => Ok(Self::PbacZIpko),
            "plus_bank" => Ok(Self::PlusBank),
            "santander_przelew24" => Ok(Self::SantanderPrzelew24),
            "tmobile_usbugi_bankowe" => Ok(Self::TmobileUsbugiBankowe),
            "toyota_bank" => Ok(Self::ToyotaBank),
            "volkswagen_bank" => Ok(Self::VolkswagenBank),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for P24Bank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for P24Bank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for P24Bank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for P24Bank {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for P24Bank"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for P24Bank {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<P24Bank> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(P24Bank::from_str(s)?);
        Ok(())
    }
}
