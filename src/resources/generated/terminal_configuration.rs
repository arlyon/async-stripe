// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

use crate::ids::TerminalConfigurationId;
use crate::params::{Expandable, Object};
use crate::resources::File;

/// The resource representing a Stripe "TerminalConfigurationConfiguration".
///
/// For more details see <https://stripe.com/docs/api/terminal/configuration/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TerminalConfiguration {
    /// Unique identifier for the object.
    pub id: TerminalConfigurationId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbpos_wisepos_e: Option<TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,

    // Always true for a deleted object
    #[serde(default)]
    pub deleted: bool,

    /// Whether this Configuration is the default for your account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_account_default: Option<bool>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub livemode: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<TerminalConfigurationConfigurationResourceOfflineConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tipping: Option<TerminalConfigurationConfigurationResourceTipping>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub verifone_p400: Option<TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,
}

impl Object for TerminalConfiguration {
    type Id = TerminalConfigurationId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "terminal.configuration"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig {
    /// A File ID representing an image you would like displayed on the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splashscreen: Option<Expandable<File>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TerminalConfigurationConfigurationResourceOfflineConfig {
    /// Determines whether to allow transactions to be collected while reader is offline.
    ///
    /// Defaults to false.
    pub enabled: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TerminalConfigurationConfigurationResourceTipping {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aud: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cad: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub chf: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub czk: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkk: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub eur: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gbp: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hkd: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub myr: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nok: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nzd: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sek: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sgd: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub usd: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TerminalConfigurationConfigurationResourceCurrencySpecificConfig {
    /// Fixed amounts displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,

    /// Percentages displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,

    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}
