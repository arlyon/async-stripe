#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Tipping {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aud:
        Option<crate::terminal::configuration::currency_specific_config::CurrencySpecificConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cad:
        Option<crate::terminal::configuration::currency_specific_config::CurrencySpecificConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chf:
        Option<crate::terminal::configuration::currency_specific_config::CurrencySpecificConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub czk:
        Option<crate::terminal::configuration::currency_specific_config::CurrencySpecificConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkk:
        Option<crate::terminal::configuration::currency_specific_config::CurrencySpecificConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eur:
        Option<crate::terminal::configuration::currency_specific_config::CurrencySpecificConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gbp:
        Option<crate::terminal::configuration::currency_specific_config::CurrencySpecificConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hkd:
        Option<crate::terminal::configuration::currency_specific_config::CurrencySpecificConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub myr:
        Option<crate::terminal::configuration::currency_specific_config::CurrencySpecificConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nok:
        Option<crate::terminal::configuration::currency_specific_config::CurrencySpecificConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nzd:
        Option<crate::terminal::configuration::currency_specific_config::CurrencySpecificConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sek:
        Option<crate::terminal::configuration::currency_specific_config::CurrencySpecificConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sgd:
        Option<crate::terminal::configuration::currency_specific_config::CurrencySpecificConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usd:
        Option<crate::terminal::configuration::currency_specific_config::CurrencySpecificConfig>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Tipping {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
