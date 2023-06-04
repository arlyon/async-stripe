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
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
