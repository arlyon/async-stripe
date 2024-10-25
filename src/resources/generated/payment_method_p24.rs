// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "payment_method_p24".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodP24 {

    /// The customer's bank, if provided.
    pub bank: Option<PaymentMethodP24Bank>,
}

/// An enum representing the possible values of an `PaymentMethodP24`'s `bank` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodP24Bank {
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
    Velobank,
    VolkswagenBank,
}

impl PaymentMethodP24Bank {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodP24Bank::AliorBank => "alior_bank",
            PaymentMethodP24Bank::BankMillennium => "bank_millennium",
            PaymentMethodP24Bank::BankNowyBfgSa => "bank_nowy_bfg_sa",
            PaymentMethodP24Bank::BankPekaoSa => "bank_pekao_sa",
            PaymentMethodP24Bank::BankiSpbdzielcze => "banki_spbdzielcze",
            PaymentMethodP24Bank::Blik => "blik",
            PaymentMethodP24Bank::BnpParibas => "bnp_paribas",
            PaymentMethodP24Bank::Boz => "boz",
            PaymentMethodP24Bank::CitiHandlowy => "citi_handlowy",
            PaymentMethodP24Bank::CreditAgricole => "credit_agricole",
            PaymentMethodP24Bank::Envelobank => "envelobank",
            PaymentMethodP24Bank::EtransferPocztowy24 => "etransfer_pocztowy24",
            PaymentMethodP24Bank::GetinBank => "getin_bank",
            PaymentMethodP24Bank::Ideabank => "ideabank",
            PaymentMethodP24Bank::Ing => "ing",
            PaymentMethodP24Bank::Inteligo => "inteligo",
            PaymentMethodP24Bank::MbankMtransfer => "mbank_mtransfer",
            PaymentMethodP24Bank::NestPrzelew => "nest_przelew",
            PaymentMethodP24Bank::NoblePay => "noble_pay",
            PaymentMethodP24Bank::PbacZIpko => "pbac_z_ipko",
            PaymentMethodP24Bank::PlusBank => "plus_bank",
            PaymentMethodP24Bank::SantanderPrzelew24 => "santander_przelew24",
            PaymentMethodP24Bank::TmobileUsbugiBankowe => "tmobile_usbugi_bankowe",
            PaymentMethodP24Bank::ToyotaBank => "toyota_bank",
            PaymentMethodP24Bank::Velobank => "velobank",
            PaymentMethodP24Bank::VolkswagenBank => "volkswagen_bank",
        }
    }
}

impl AsRef<str> for PaymentMethodP24Bank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodP24Bank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodP24Bank {
    fn default() -> Self {
        Self::AliorBank
    }
}
