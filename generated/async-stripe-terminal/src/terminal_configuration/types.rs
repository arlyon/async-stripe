/// A Configurations object represents how features should be configured for terminal readers.
/// For information about how to use it, see the [Terminal configurations documentation](https://docs.stripe.com/terminal/fleet/configurations-overview).
///
/// For more details see <<https://stripe.com/docs/api/terminal/configuration/object>>.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalConfiguration {
    pub bbpos_wisepad3:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,
    pub bbpos_wisepos_e:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,
    pub cellular: Option<stripe_terminal::TerminalConfigurationConfigurationResourceCellularConfig>,
    /// Unique identifier for the object.
    pub id: stripe_terminal::TerminalConfigurationId,
    /// Whether this Configuration is the default for your account
    pub is_account_default: Option<bool>,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// String indicating the name of the Configuration object, set by the user
    pub name: Option<String>,
    pub offline: Option<stripe_terminal::TerminalConfigurationConfigurationResourceOfflineConfig>,
    pub reboot_window:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceRebootWindow>,
    pub stripe_s700:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,
    pub stripe_s710:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,
    pub tipping: Option<stripe_terminal::TerminalConfigurationConfigurationResourceTipping>,
    pub verifone_m425:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,
    pub verifone_p400:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,
    pub verifone_p630:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,
    pub verifone_ux700:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,
    pub verifone_v660p:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,
    pub wifi: Option<stripe_terminal::TerminalConfigurationConfigurationResourceWifiConfig>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TerminalConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TerminalConfiguration").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TerminalConfigurationBuilder {
    bbpos_wisepad3:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,
    bbpos_wisepos_e:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,
    cellular: Option<stripe_terminal::TerminalConfigurationConfigurationResourceCellularConfig>,
    id: Option<stripe_terminal::TerminalConfigurationId>,
    is_account_default: Option<Option<bool>>,
    livemode: Option<bool>,
    name: Option<Option<String>>,
    offline: Option<stripe_terminal::TerminalConfigurationConfigurationResourceOfflineConfig>,
    reboot_window: Option<stripe_terminal::TerminalConfigurationConfigurationResourceRebootWindow>,
    stripe_s700:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,
    stripe_s710:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,
    tipping: Option<stripe_terminal::TerminalConfigurationConfigurationResourceTipping>,
    verifone_m425:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,
    verifone_p400:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,
    verifone_p630:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,
    verifone_ux700:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,
    verifone_v660p:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,
    wifi: Option<stripe_terminal::TerminalConfigurationConfigurationResourceWifiConfig>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TerminalConfiguration {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TerminalConfiguration>,
        builder: TerminalConfigurationBuilder,
    }

    impl Visitor for Place<TerminalConfiguration> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TerminalConfigurationBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TerminalConfigurationBuilder {
        type Out = TerminalConfiguration;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bbpos_wisepad3" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.bbpos_wisepad3)
                }
                "bbpos_wisepos_e" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.bbpos_wisepos_e)
                }
                "cellular" => stripe_types::miniserde_helpers::begin_nullable(&mut self.cellular),
                "id" => Deserialize::begin(&mut self.id),
                "is_account_default" => Deserialize::begin(&mut self.is_account_default),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "name" => Deserialize::begin(&mut self.name),
                "offline" => stripe_types::miniserde_helpers::begin_nullable(&mut self.offline),
                "reboot_window" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.reboot_window)
                }
                "stripe_s700" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.stripe_s700)
                }
                "stripe_s710" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.stripe_s710)
                }
                "tipping" => stripe_types::miniserde_helpers::begin_nullable(&mut self.tipping),
                "verifone_m425" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.verifone_m425)
                }
                "verifone_p400" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.verifone_p400)
                }
                "verifone_p630" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.verifone_p630)
                }
                "verifone_ux700" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.verifone_ux700)
                }
                "verifone_v660p" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.verifone_v660p)
                }
                "wifi" => stripe_types::miniserde_helpers::begin_nullable(&mut self.wifi),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                bbpos_wisepad3: None,
                bbpos_wisepos_e: None,
                cellular: None,
                id: None,
                is_account_default: Some(None),
                livemode: None,
                name: Some(None),
                offline: None,
                reboot_window: None,
                stripe_s700: None,
                stripe_s710: None,
                tipping: None,
                verifone_m425: None,
                verifone_p400: None,
                verifone_p630: None,
                verifone_ux700: None,
                verifone_v660p: None,
                wifi: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self {
                    bbpos_wisepad3,
                    bbpos_wisepos_e,
                    cellular,
                    id: Some(id),
                    is_account_default: Some(is_account_default),
                    livemode: Some(livemode),
                    name: Some(name),
                    offline,
                    reboot_window,
                    stripe_s700,
                    stripe_s710,
                    tipping,
                    verifone_m425,
                    verifone_p400,
                    verifone_p630,
                    verifone_ux700,
                    verifone_v660p,
                    wifi,
                } => Some(Self::Out {
                    bbpos_wisepad3,
                    bbpos_wisepos_e,
                    cellular,
                    id,
                    is_account_default,
                    livemode,
                    name,
                    offline,
                    reboot_window,
                    stripe_s700,
                    stripe_s710,
                    tipping,
                    verifone_m425,
                    verifone_p400,
                    verifone_p630,
                    verifone_ux700,
                    verifone_v660p,
                    wifi,
                }),
                builder => {
                    *self = builder;
                    None
                }
            }
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for TerminalConfiguration {
        type Builder = TerminalConfigurationBuilder;
    }

    impl FromValueOpt for TerminalConfiguration {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for TerminalConfiguration {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("TerminalConfiguration", 19)?;
        s.serialize_field("bbpos_wisepad3", &self.bbpos_wisepad3)?;
        s.serialize_field("bbpos_wisepos_e", &self.bbpos_wisepos_e)?;
        s.serialize_field("cellular", &self.cellular)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("is_account_default", &self.is_account_default)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("offline", &self.offline)?;
        s.serialize_field("reboot_window", &self.reboot_window)?;
        s.serialize_field("stripe_s700", &self.stripe_s700)?;
        s.serialize_field("stripe_s710", &self.stripe_s710)?;
        s.serialize_field("tipping", &self.tipping)?;
        s.serialize_field("verifone_m425", &self.verifone_m425)?;
        s.serialize_field("verifone_p400", &self.verifone_p400)?;
        s.serialize_field("verifone_p630", &self.verifone_p630)?;
        s.serialize_field("verifone_ux700", &self.verifone_ux700)?;
        s.serialize_field("verifone_v660p", &self.verifone_v660p)?;
        s.serialize_field("wifi", &self.wifi)?;

        s.serialize_field("object", "terminal.configuration")?;
        s.end()
    }
}
impl stripe_types::Object for TerminalConfiguration {
    type Id = stripe_terminal::TerminalConfigurationId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(TerminalConfigurationId);
