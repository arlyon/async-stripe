#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct TerminalConfigurationConfigurationResourceTipping {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aud:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cad:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chf:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub czk:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkk:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eur:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gbp:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hkd:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub myr:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nok:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nzd:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sek:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sgd:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usd:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
}
