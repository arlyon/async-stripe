#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Eps {
    /// The customer's bank.
    ///
    /// Should be one of `arzte_und_apotheker_bank`, `austrian_anadi_bank_ag`, `bank_austria`, `bankhaus_carl_spangler`, `bankhaus_schelhammer_und_schattera_ag`, `bawag_psk_ag`, `bks_bank_ag`, `brull_kallmus_bank_ag`, `btv_vier_lander_bank`, `capital_bank_grawe_gruppe_ag`, `deutsche_bank_ag`, `dolomitenbank`, `easybank_ag`, `erste_bank_und_sparkassen`, `hypo_alpeadriabank_international_ag`, `hypo_noe_lb_fur_niederosterreich_u_wien`, `hypo_oberosterreich_salzburg_steiermark`, `hypo_tirol_bank_ag`, `hypo_vorarlberg_bank_ag`, `hypo_bank_burgenland_aktiengesellschaft`, `marchfelder_bank`, `oberbank_ag`, `raiffeisen_bankengruppe_osterreich`, `schoellerbank_ag`, `sparda_bank_wien`, `volksbank_gruppe`, `volkskreditbank_ag`, or `vr_bank_braunau`.
    pub bank: Option<EpsBank>,
    /// Owner's verified full name.
    ///
    /// Values are verified or provided by EPS directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated. EPS rarely provides this information so the attribute is usually empty.
    pub verified_name: Option<String>,
}
/// The customer's bank.
///
/// Should be one of `arzte_und_apotheker_bank`, `austrian_anadi_bank_ag`, `bank_austria`, `bankhaus_carl_spangler`, `bankhaus_schelhammer_und_schattera_ag`, `bawag_psk_ag`, `bks_bank_ag`, `brull_kallmus_bank_ag`, `btv_vier_lander_bank`, `capital_bank_grawe_gruppe_ag`, `deutsche_bank_ag`, `dolomitenbank`, `easybank_ag`, `erste_bank_und_sparkassen`, `hypo_alpeadriabank_international_ag`, `hypo_noe_lb_fur_niederosterreich_u_wien`, `hypo_oberosterreich_salzburg_steiermark`, `hypo_tirol_bank_ag`, `hypo_vorarlberg_bank_ag`, `hypo_bank_burgenland_aktiengesellschaft`, `marchfelder_bank`, `oberbank_ag`, `raiffeisen_bankengruppe_osterreich`, `schoellerbank_ag`, `sparda_bank_wien`, `volksbank_gruppe`, `volkskreditbank_ag`, or `vr_bank_braunau`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum EpsBank {
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

impl EpsBank {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ArzteUndApothekerBank => "arzte_und_apotheker_bank",
            Self::AustrianAnadiBankAg => "austrian_anadi_bank_ag",
            Self::BankAustria => "bank_austria",
            Self::BankhausCarlSpangler => "bankhaus_carl_spangler",
            Self::BankhausSchelhammerUndSchatteraAg => "bankhaus_schelhammer_und_schattera_ag",
            Self::BawagPskAg => "bawag_psk_ag",
            Self::BksBankAg => "bks_bank_ag",
            Self::BrullKallmusBankAg => "brull_kallmus_bank_ag",
            Self::BtvVierLanderBank => "btv_vier_lander_bank",
            Self::CapitalBankGraweGruppeAg => "capital_bank_grawe_gruppe_ag",
            Self::DeutscheBankAg => "deutsche_bank_ag",
            Self::Dolomitenbank => "dolomitenbank",
            Self::EasybankAg => "easybank_ag",
            Self::ErsteBankUndSparkassen => "erste_bank_und_sparkassen",
            Self::HypoAlpeadriabankInternationalAg => "hypo_alpeadriabank_international_ag",
            Self::HypoBankBurgenlandAktiengesellschaft => "hypo_bank_burgenland_aktiengesellschaft",
            Self::HypoNoeLbFurNiederosterreichUWien => "hypo_noe_lb_fur_niederosterreich_u_wien",
            Self::HypoOberosterreichSalzburgSteiermark => "hypo_oberosterreich_salzburg_steiermark",
            Self::HypoTirolBankAg => "hypo_tirol_bank_ag",
            Self::HypoVorarlbergBankAg => "hypo_vorarlberg_bank_ag",
            Self::MarchfelderBank => "marchfelder_bank",
            Self::OberbankAg => "oberbank_ag",
            Self::RaiffeisenBankengruppeOsterreich => "raiffeisen_bankengruppe_osterreich",
            Self::SchoellerbankAg => "schoellerbank_ag",
            Self::SpardaBankWien => "sparda_bank_wien",
            Self::VolksbankGruppe => "volksbank_gruppe",
            Self::VolkskreditbankAg => "volkskreditbank_ag",
            Self::VrBankBraunau => "vr_bank_braunau",
        }
    }
}

impl std::str::FromStr for EpsBank {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "arzte_und_apotheker_bank" => Ok(Self::ArzteUndApothekerBank),
            "austrian_anadi_bank_ag" => Ok(Self::AustrianAnadiBankAg),
            "bank_austria" => Ok(Self::BankAustria),
            "bankhaus_carl_spangler" => Ok(Self::BankhausCarlSpangler),
            "bankhaus_schelhammer_und_schattera_ag" => Ok(Self::BankhausSchelhammerUndSchatteraAg),
            "bawag_psk_ag" => Ok(Self::BawagPskAg),
            "bks_bank_ag" => Ok(Self::BksBankAg),
            "brull_kallmus_bank_ag" => Ok(Self::BrullKallmusBankAg),
            "btv_vier_lander_bank" => Ok(Self::BtvVierLanderBank),
            "capital_bank_grawe_gruppe_ag" => Ok(Self::CapitalBankGraweGruppeAg),
            "deutsche_bank_ag" => Ok(Self::DeutscheBankAg),
            "dolomitenbank" => Ok(Self::Dolomitenbank),
            "easybank_ag" => Ok(Self::EasybankAg),
            "erste_bank_und_sparkassen" => Ok(Self::ErsteBankUndSparkassen),
            "hypo_alpeadriabank_international_ag" => Ok(Self::HypoAlpeadriabankInternationalAg),
            "hypo_bank_burgenland_aktiengesellschaft" => {
                Ok(Self::HypoBankBurgenlandAktiengesellschaft)
            }
            "hypo_noe_lb_fur_niederosterreich_u_wien" => {
                Ok(Self::HypoNoeLbFurNiederosterreichUWien)
            }
            "hypo_oberosterreich_salzburg_steiermark" => {
                Ok(Self::HypoOberosterreichSalzburgSteiermark)
            }
            "hypo_tirol_bank_ag" => Ok(Self::HypoTirolBankAg),
            "hypo_vorarlberg_bank_ag" => Ok(Self::HypoVorarlbergBankAg),
            "marchfelder_bank" => Ok(Self::MarchfelderBank),
            "oberbank_ag" => Ok(Self::OberbankAg),
            "raiffeisen_bankengruppe_osterreich" => Ok(Self::RaiffeisenBankengruppeOsterreich),
            "schoellerbank_ag" => Ok(Self::SchoellerbankAg),
            "sparda_bank_wien" => Ok(Self::SpardaBankWien),
            "volksbank_gruppe" => Ok(Self::VolksbankGruppe),
            "volkskreditbank_ag" => Ok(Self::VolkskreditbankAg),
            "vr_bank_braunau" => Ok(Self::VrBankBraunau),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for EpsBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for EpsBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for EpsBank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for EpsBank {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for EpsBank"))
    }
}
