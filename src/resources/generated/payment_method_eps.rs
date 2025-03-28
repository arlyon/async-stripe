// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "payment_method_eps".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodEps {

    /// The customer's bank.
    ///
    /// Should be one of `arzte_und_apotheker_bank`, `austrian_anadi_bank_ag`, `bank_austria`, `bankhaus_carl_spangler`, `bankhaus_schelhammer_und_schattera_ag`, `bawag_psk_ag`, `bks_bank_ag`, `brull_kallmus_bank_ag`, `btv_vier_lander_bank`, `capital_bank_grawe_gruppe_ag`, `deutsche_bank_ag`, `dolomitenbank`, `easybank_ag`, `erste_bank_und_sparkassen`, `hypo_alpeadriabank_international_ag`, `hypo_noe_lb_fur_niederosterreich_u_wien`, `hypo_oberosterreich_salzburg_steiermark`, `hypo_tirol_bank_ag`, `hypo_vorarlberg_bank_ag`, `hypo_bank_burgenland_aktiengesellschaft`, `marchfelder_bank`, `oberbank_ag`, `raiffeisen_bankengruppe_osterreich`, `schoellerbank_ag`, `sparda_bank_wien`, `volksbank_gruppe`, `volkskreditbank_ag`, or `vr_bank_braunau`.
    pub bank: Option<PaymentMethodEpsBank>,
}

/// An enum representing the possible values of an `PaymentMethodEps`'s `bank` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodEpsBank {
    ArzteUndApothekerBank,
    AustrianAnadiBankAg,
    BankAustria,
    BankhausCarlSpangler,
    BankhausSchelhammerUndSchatteraAg,
    BawagPskAg,
    BksBankAg,
    BrullKallmusBankAg,
    BtvVierLanderBank,
    CapitalBankGraweGruppeAg,
    DeutscheBankAg,
    Dolomitenbank,
    EasybankAg,
    ErsteBankUndSparkassen,
    HypoAlpeadriabankInternationalAg,
    HypoBankBurgenlandAktiengesellschaft,
    HypoNoeLbFurNiederosterreichUWien,
    HypoOberosterreichSalzburgSteiermark,
    HypoTirolBankAg,
    HypoVorarlbergBankAg,
    MarchfelderBank,
    OberbankAg,
    RaiffeisenBankengruppeOsterreich,
    SchoellerbankAg,
    SpardaBankWien,
    VolksbankGruppe,
    VolkskreditbankAg,
    VrBankBraunau,
}

impl PaymentMethodEpsBank {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodEpsBank::ArzteUndApothekerBank => "arzte_und_apotheker_bank",
            PaymentMethodEpsBank::AustrianAnadiBankAg => "austrian_anadi_bank_ag",
            PaymentMethodEpsBank::BankAustria => "bank_austria",
            PaymentMethodEpsBank::BankhausCarlSpangler => "bankhaus_carl_spangler",
            PaymentMethodEpsBank::BankhausSchelhammerUndSchatteraAg => "bankhaus_schelhammer_und_schattera_ag",
            PaymentMethodEpsBank::BawagPskAg => "bawag_psk_ag",
            PaymentMethodEpsBank::BksBankAg => "bks_bank_ag",
            PaymentMethodEpsBank::BrullKallmusBankAg => "brull_kallmus_bank_ag",
            PaymentMethodEpsBank::BtvVierLanderBank => "btv_vier_lander_bank",
            PaymentMethodEpsBank::CapitalBankGraweGruppeAg => "capital_bank_grawe_gruppe_ag",
            PaymentMethodEpsBank::DeutscheBankAg => "deutsche_bank_ag",
            PaymentMethodEpsBank::Dolomitenbank => "dolomitenbank",
            PaymentMethodEpsBank::EasybankAg => "easybank_ag",
            PaymentMethodEpsBank::ErsteBankUndSparkassen => "erste_bank_und_sparkassen",
            PaymentMethodEpsBank::HypoAlpeadriabankInternationalAg => "hypo_alpeadriabank_international_ag",
            PaymentMethodEpsBank::HypoBankBurgenlandAktiengesellschaft => "hypo_bank_burgenland_aktiengesellschaft",
            PaymentMethodEpsBank::HypoNoeLbFurNiederosterreichUWien => "hypo_noe_lb_fur_niederosterreich_u_wien",
            PaymentMethodEpsBank::HypoOberosterreichSalzburgSteiermark => "hypo_oberosterreich_salzburg_steiermark",
            PaymentMethodEpsBank::HypoTirolBankAg => "hypo_tirol_bank_ag",
            PaymentMethodEpsBank::HypoVorarlbergBankAg => "hypo_vorarlberg_bank_ag",
            PaymentMethodEpsBank::MarchfelderBank => "marchfelder_bank",
            PaymentMethodEpsBank::OberbankAg => "oberbank_ag",
            PaymentMethodEpsBank::RaiffeisenBankengruppeOsterreich => "raiffeisen_bankengruppe_osterreich",
            PaymentMethodEpsBank::SchoellerbankAg => "schoellerbank_ag",
            PaymentMethodEpsBank::SpardaBankWien => "sparda_bank_wien",
            PaymentMethodEpsBank::VolksbankGruppe => "volksbank_gruppe",
            PaymentMethodEpsBank::VolkskreditbankAg => "volkskreditbank_ag",
            PaymentMethodEpsBank::VrBankBraunau => "vr_bank_braunau",
        }
    }
}

impl AsRef<str> for PaymentMethodEpsBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodEpsBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodEpsBank {
    fn default() -> Self {
        Self::ArzteUndApothekerBank
    }
}
