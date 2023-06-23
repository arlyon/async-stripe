#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Tipping {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aud: Option<
        stripe_terminal::terminal::configuration::currency_specific_config::CurrencySpecificConfig,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cad: Option<
        stripe_terminal::terminal::configuration::currency_specific_config::CurrencySpecificConfig,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chf: Option<
        stripe_terminal::terminal::configuration::currency_specific_config::CurrencySpecificConfig,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub czk: Option<
        stripe_terminal::terminal::configuration::currency_specific_config::CurrencySpecificConfig,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkk: Option<
        stripe_terminal::terminal::configuration::currency_specific_config::CurrencySpecificConfig,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eur: Option<
        stripe_terminal::terminal::configuration::currency_specific_config::CurrencySpecificConfig,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gbp: Option<
        stripe_terminal::terminal::configuration::currency_specific_config::CurrencySpecificConfig,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hkd: Option<
        stripe_terminal::terminal::configuration::currency_specific_config::CurrencySpecificConfig,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub myr: Option<
        stripe_terminal::terminal::configuration::currency_specific_config::CurrencySpecificConfig,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nok: Option<
        stripe_terminal::terminal::configuration::currency_specific_config::CurrencySpecificConfig,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nzd: Option<
        stripe_terminal::terminal::configuration::currency_specific_config::CurrencySpecificConfig,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sek: Option<
        stripe_terminal::terminal::configuration::currency_specific_config::CurrencySpecificConfig,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sgd: Option<
        stripe_terminal::terminal::configuration::currency_specific_config::CurrencySpecificConfig,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usd: Option<
        stripe_terminal::terminal::configuration::currency_specific_config::CurrencySpecificConfig,
    >,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Tipping {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
