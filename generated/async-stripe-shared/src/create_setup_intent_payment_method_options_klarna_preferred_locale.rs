/// Preferred language of the Klarna authorization page that the customer is redirected to
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateSetupIntentPaymentMethodOptionsKlarnaPreferredLocale {
    CsMinusCz,
    DaMinusDk,
    DeMinusAt,
    DeMinusCh,
    DeMinusDe,
    ElMinusGr,
    EnMinusAt,
    EnMinusAu,
    EnMinusBe,
    EnMinusCa,
    EnMinusCh,
    EnMinusCz,
    EnMinusDe,
    EnMinusDk,
    EnMinusEs,
    EnMinusFi,
    EnMinusFr,
    EnMinusGb,
    EnMinusGr,
    EnMinusIe,
    EnMinusIt,
    EnMinusNl,
    EnMinusNo,
    EnMinusNz,
    EnMinusPl,
    EnMinusPt,
    EnMinusRo,
    EnMinusSe,
    EnMinusUs,
    EsMinusEs,
    EsMinusUs,
    FiMinusFi,
    FrMinusBe,
    FrMinusCa,
    FrMinusCh,
    FrMinusFr,
    ItMinusCh,
    ItMinusIt,
    NbMinusNo,
    NlMinusBe,
    NlMinusNl,
    PlMinusPl,
    PtMinusPt,
    RoMinusRo,
    SvMinusFi,
    SvMinusSe,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateSetupIntentPaymentMethodOptionsKlarnaPreferredLocale {
    pub fn as_str(&self) -> &str {
        use CreateSetupIntentPaymentMethodOptionsKlarnaPreferredLocale::*;
        match self {
            CsMinusCz => "cs-CZ",
            DaMinusDk => "da-DK",
            DeMinusAt => "de-AT",
            DeMinusCh => "de-CH",
            DeMinusDe => "de-DE",
            ElMinusGr => "el-GR",
            EnMinusAt => "en-AT",
            EnMinusAu => "en-AU",
            EnMinusBe => "en-BE",
            EnMinusCa => "en-CA",
            EnMinusCh => "en-CH",
            EnMinusCz => "en-CZ",
            EnMinusDe => "en-DE",
            EnMinusDk => "en-DK",
            EnMinusEs => "en-ES",
            EnMinusFi => "en-FI",
            EnMinusFr => "en-FR",
            EnMinusGb => "en-GB",
            EnMinusGr => "en-GR",
            EnMinusIe => "en-IE",
            EnMinusIt => "en-IT",
            EnMinusNl => "en-NL",
            EnMinusNo => "en-NO",
            EnMinusNz => "en-NZ",
            EnMinusPl => "en-PL",
            EnMinusPt => "en-PT",
            EnMinusRo => "en-RO",
            EnMinusSe => "en-SE",
            EnMinusUs => "en-US",
            EsMinusEs => "es-ES",
            EsMinusUs => "es-US",
            FiMinusFi => "fi-FI",
            FrMinusBe => "fr-BE",
            FrMinusCa => "fr-CA",
            FrMinusCh => "fr-CH",
            FrMinusFr => "fr-FR",
            ItMinusCh => "it-CH",
            ItMinusIt => "it-IT",
            NbMinusNo => "nb-NO",
            NlMinusBe => "nl-BE",
            NlMinusNl => "nl-NL",
            PlMinusPl => "pl-PL",
            PtMinusPt => "pt-PT",
            RoMinusRo => "ro-RO",
            SvMinusFi => "sv-FI",
            SvMinusSe => "sv-SE",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateSetupIntentPaymentMethodOptionsKlarnaPreferredLocale {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSetupIntentPaymentMethodOptionsKlarnaPreferredLocale::*;
        match s {
            "cs-CZ" => Ok(CsMinusCz),
            "da-DK" => Ok(DaMinusDk),
            "de-AT" => Ok(DeMinusAt),
            "de-CH" => Ok(DeMinusCh),
            "de-DE" => Ok(DeMinusDe),
            "el-GR" => Ok(ElMinusGr),
            "en-AT" => Ok(EnMinusAt),
            "en-AU" => Ok(EnMinusAu),
            "en-BE" => Ok(EnMinusBe),
            "en-CA" => Ok(EnMinusCa),
            "en-CH" => Ok(EnMinusCh),
            "en-CZ" => Ok(EnMinusCz),
            "en-DE" => Ok(EnMinusDe),
            "en-DK" => Ok(EnMinusDk),
            "en-ES" => Ok(EnMinusEs),
            "en-FI" => Ok(EnMinusFi),
            "en-FR" => Ok(EnMinusFr),
            "en-GB" => Ok(EnMinusGb),
            "en-GR" => Ok(EnMinusGr),
            "en-IE" => Ok(EnMinusIe),
            "en-IT" => Ok(EnMinusIt),
            "en-NL" => Ok(EnMinusNl),
            "en-NO" => Ok(EnMinusNo),
            "en-NZ" => Ok(EnMinusNz),
            "en-PL" => Ok(EnMinusPl),
            "en-PT" => Ok(EnMinusPt),
            "en-RO" => Ok(EnMinusRo),
            "en-SE" => Ok(EnMinusSe),
            "en-US" => Ok(EnMinusUs),
            "es-ES" => Ok(EsMinusEs),
            "es-US" => Ok(EsMinusUs),
            "fi-FI" => Ok(FiMinusFi),
            "fr-BE" => Ok(FrMinusBe),
            "fr-CA" => Ok(FrMinusCa),
            "fr-CH" => Ok(FrMinusCh),
            "fr-FR" => Ok(FrMinusFr),
            "it-CH" => Ok(ItMinusCh),
            "it-IT" => Ok(ItMinusIt),
            "nb-NO" => Ok(NbMinusNo),
            "nl-BE" => Ok(NlMinusBe),
            "nl-NL" => Ok(NlMinusNl),
            "pl-PL" => Ok(PlMinusPl),
            "pt-PT" => Ok(PtMinusPt),
            "ro-RO" => Ok(RoMinusRo),
            "sv-FI" => Ok(SvMinusFi),
            "sv-SE" => Ok(SvMinusSe),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateSetupIntentPaymentMethodOptionsKlarnaPreferredLocale"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateSetupIntentPaymentMethodOptionsKlarnaPreferredLocale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreateSetupIntentPaymentMethodOptionsKlarnaPreferredLocale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSetupIntentPaymentMethodOptionsKlarnaPreferredLocale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreateSetupIntentPaymentMethodOptionsKlarnaPreferredLocale))
            .finish_non_exhaustive()
    }
}
impl serde::Serialize for CreateSetupIntentPaymentMethodOptionsKlarnaPreferredLocale {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateSetupIntentPaymentMethodOptionsKlarnaPreferredLocale {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
