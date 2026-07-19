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
    pub verifone_p400:
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
    bbpos_wisepad3: Option<
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,
    >,
    bbpos_wisepos_e: Option<
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,
    >,
    cellular:
        Option<Option<stripe_terminal::TerminalConfigurationConfigurationResourceCellularConfig>>,
    id: Option<stripe_terminal::TerminalConfigurationId>,
    is_account_default: Option<Option<bool>>,
    livemode: Option<bool>,
    name: Option<Option<String>>,
    offline:
        Option<Option<stripe_terminal::TerminalConfigurationConfigurationResourceOfflineConfig>>,
    reboot_window:
        Option<Option<stripe_terminal::TerminalConfigurationConfigurationResourceRebootWindow>>,
    stripe_s700: Option<
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,
    >,
    stripe_s710: Option<
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,
    >,
    tipping: Option<Option<stripe_terminal::TerminalConfigurationConfigurationResourceTipping>>,
    verifone_p400: Option<
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,
    >,
    wifi: Option<Option<stripe_terminal::TerminalConfigurationConfigurationResourceWifiConfig>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: TerminalConfigurationBuilder {
                    bbpos_wisepad3: Deserialize::default(),
                    bbpos_wisepos_e: Deserialize::default(),
                    cellular: Deserialize::default(),
                    id: Deserialize::default(),
                    is_account_default: Deserialize::default(),
                    livemode: Deserialize::default(),
                    name: Deserialize::default(),
                    offline: Deserialize::default(),
                    reboot_window: Deserialize::default(),
                    stripe_s700: Deserialize::default(),
                    stripe_s710: Deserialize::default(),
                    tipping: Deserialize::default(),
                    verifone_p400: Deserialize::default(),
                    wifi: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bbpos_wisepad3" => Deserialize::begin(&mut self.builder.bbpos_wisepad3),
                "bbpos_wisepos_e" => Deserialize::begin(&mut self.builder.bbpos_wisepos_e),
                "cellular" => Deserialize::begin(&mut self.builder.cellular),
                "id" => Deserialize::begin(&mut self.builder.id),
                "is_account_default" => Deserialize::begin(&mut self.builder.is_account_default),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "name" => Deserialize::begin(&mut self.builder.name),
                "offline" => Deserialize::begin(&mut self.builder.offline),
                "reboot_window" => Deserialize::begin(&mut self.builder.reboot_window),
                "stripe_s700" => Deserialize::begin(&mut self.builder.stripe_s700),
                "stripe_s710" => Deserialize::begin(&mut self.builder.stripe_s710),
                "tipping" => Deserialize::begin(&mut self.builder.tipping),
                "verifone_p400" => Deserialize::begin(&mut self.builder.verifone_p400),
                "wifi" => Deserialize::begin(&mut self.builder.wifi),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(bbpos_wisepad3),
                Some(bbpos_wisepos_e),
                Some(cellular),
                Some(id),
                Some(is_account_default),
                Some(livemode),
                Some(name),
                Some(offline),
                Some(reboot_window),
                Some(stripe_s700),
                Some(stripe_s710),
                Some(tipping),
                Some(verifone_p400),
                Some(wifi),
            ) = (
                self.builder.bbpos_wisepad3.take(),
                self.builder.bbpos_wisepos_e.take(),
                self.builder.cellular,
                self.builder.id.take(),
                self.builder.is_account_default,
                self.builder.livemode,
                self.builder.name.take(),
                self.builder.offline,
                self.builder.reboot_window,
                self.builder.stripe_s700.take(),
                self.builder.stripe_s710.take(),
                self.builder.tipping.take(),
                self.builder.verifone_p400.take(),
                self.builder.wifi.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(TerminalConfiguration {
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
                verifone_p400,
                wifi,
            });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for TerminalConfiguration {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("TerminalConfiguration", 15)?;
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
        s.serialize_field("verifone_p400", &self.verifone_p400)?;
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
