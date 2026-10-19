/// [Preferred locale](https://docs.stripe.com/payments/paypal/supported-locales) of the PayPal checkout page that the customer is redirected to.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PreferredLocale {
    CsMinusCz,
    DaMinusDk,
    DeMinusAt,
    DeMinusDe,
    DeMinusLu,
    ElMinusGr,
    EnMinusGb,
    EnMinusUs,
    EsMinusEs,
    FiMinusFi,
    FrMinusBe,
    FrMinusFr,
    FrMinusLu,
    HuMinusHu,
    ItMinusIt,
    NlMinusBe,
    NlMinusNl,
    PlMinusPl,
    PtMinusPt,
    SkMinusSk,
    SvMinusSe,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PreferredLocale {
    pub fn as_str(&self) -> &str {
        use PreferredLocale::*;
        match self {
            CsMinusCz => "cs-CZ",
            DaMinusDk => "da-DK",
            DeMinusAt => "de-AT",
            DeMinusDe => "de-DE",
            DeMinusLu => "de-LU",
            ElMinusGr => "el-GR",
            EnMinusGb => "en-GB",
            EnMinusUs => "en-US",
            EsMinusEs => "es-ES",
            FiMinusFi => "fi-FI",
            FrMinusBe => "fr-BE",
            FrMinusFr => "fr-FR",
            FrMinusLu => "fr-LU",
            HuMinusHu => "hu-HU",
            ItMinusIt => "it-IT",
            NlMinusBe => "nl-BE",
            NlMinusNl => "nl-NL",
            PlMinusPl => "pl-PL",
            PtMinusPt => "pt-PT",
            SkMinusSk => "sk-SK",
            SvMinusSe => "sv-SE",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PreferredLocale {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PreferredLocale::*;
        match s {
            "cs-CZ" => Ok(CsMinusCz),
            "da-DK" => Ok(DaMinusDk),
            "de-AT" => Ok(DeMinusAt),
            "de-DE" => Ok(DeMinusDe),
            "de-LU" => Ok(DeMinusLu),
            "el-GR" => Ok(ElMinusGr),
            "en-GB" => Ok(EnMinusGb),
            "en-US" => Ok(EnMinusUs),
            "es-ES" => Ok(EsMinusEs),
            "fi-FI" => Ok(FiMinusFi),
            "fr-BE" => Ok(FrMinusBe),
            "fr-FR" => Ok(FrMinusFr),
            "fr-LU" => Ok(FrMinusLu),
            "hu-HU" => Ok(HuMinusHu),
            "it-IT" => Ok(ItMinusIt),
            "nl-BE" => Ok(NlMinusBe),
            "nl-NL" => Ok(NlMinusNl),
            "pl-PL" => Ok(PlMinusPl),
            "pt-PT" => Ok(PtMinusPt),
            "sk-SK" => Ok(SkMinusSk),
            "sv-SE" => Ok(SvMinusSe),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "PreferredLocale");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PreferredLocale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PreferredLocale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PreferredLocale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PreferredLocale)).finish_non_exhaustive()
    }
}
impl serde::Serialize for PreferredLocale {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PreferredLocale {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
