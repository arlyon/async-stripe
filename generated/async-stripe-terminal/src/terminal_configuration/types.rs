/// A Configurations object represents how features should be configured for terminal readers.
///
/// For more details see <<https://stripe.com/docs/api/terminal/configuration/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalConfiguration {
    pub bbpos_wisepos_e:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,
    /// Unique identifier for the object.
    pub id: stripe_terminal::TerminalConfigurationId,
    /// Whether this Configuration is the default for your account
    pub is_account_default: Option<bool>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// String indicating the name of the Configuration object, set by the user
    pub name: Option<String>,
    pub offline: Option<stripe_terminal::TerminalConfigurationConfigurationResourceOfflineConfig>,
    pub reboot_window:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceRebootWindow>,
    pub stripe_s700:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,
    pub tipping: Option<stripe_terminal::TerminalConfigurationConfigurationResourceTipping>,
    pub verifone_p400:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,
    pub wifi: Option<stripe_terminal::TerminalConfigurationConfigurationResourceWifiConfig>,
}
#[doc(hidden)]
pub struct TerminalConfigurationBuilder {
    bbpos_wisepos_e: Option<
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,
    >,
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
    tipping: Option<Option<stripe_terminal::TerminalConfigurationConfigurationResourceTipping>>,
    verifone_p400: Option<
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,
    >,
    wifi: Option<Option<stripe_terminal::TerminalConfigurationConfigurationResourceWifiConfig>>,
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
    use miniserde::{make_place, Deserialize, Result};
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
                "bbpos_wisepos_e" => Deserialize::begin(&mut self.bbpos_wisepos_e),
                "id" => Deserialize::begin(&mut self.id),
                "is_account_default" => Deserialize::begin(&mut self.is_account_default),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "name" => Deserialize::begin(&mut self.name),
                "offline" => Deserialize::begin(&mut self.offline),
                "reboot_window" => Deserialize::begin(&mut self.reboot_window),
                "stripe_s700" => Deserialize::begin(&mut self.stripe_s700),
                "tipping" => Deserialize::begin(&mut self.tipping),
                "verifone_p400" => Deserialize::begin(&mut self.verifone_p400),
                "wifi" => Deserialize::begin(&mut self.wifi),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                bbpos_wisepos_e: Deserialize::default(),
                id: Deserialize::default(),
                is_account_default: Deserialize::default(),
                livemode: Deserialize::default(),
                name: Deserialize::default(),
                offline: Deserialize::default(),
                reboot_window: Deserialize::default(),
                stripe_s700: Deserialize::default(),
                tipping: Deserialize::default(),
                verifone_p400: Deserialize::default(),
                wifi: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(bbpos_wisepos_e),
                Some(id),
                Some(is_account_default),
                Some(livemode),
                Some(name),
                Some(offline),
                Some(reboot_window),
                Some(stripe_s700),
                Some(tipping),
                Some(verifone_p400),
                Some(wifi),
            ) = (
                self.bbpos_wisepos_e.take(),
                self.id.take(),
                self.is_account_default,
                self.livemode,
                self.name.take(),
                self.offline,
                self.reboot_window,
                self.stripe_s700.take(),
                self.tipping.take(),
                self.verifone_p400.take(),
                self.wifi.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                bbpos_wisepos_e,
                id,
                is_account_default,
                livemode,
                name,
                offline,
                reboot_window,
                stripe_s700,
                tipping,
                verifone_p400,
                wifi,
            })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for TerminalConfiguration {
        type Builder = TerminalConfigurationBuilder;
    }

    impl FromValueOpt for TerminalConfiguration {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TerminalConfigurationBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "bbpos_wisepos_e" => b.bbpos_wisepos_e = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "is_account_default" => b.is_account_default = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "name" => b.name = FromValueOpt::from_value(v),
                    "offline" => b.offline = FromValueOpt::from_value(v),
                    "reboot_window" => b.reboot_window = FromValueOpt::from_value(v),
                    "stripe_s700" => b.stripe_s700 = FromValueOpt::from_value(v),
                    "tipping" => b.tipping = FromValueOpt::from_value(v),
                    "verifone_p400" => b.verifone_p400 = FromValueOpt::from_value(v),
                    "wifi" => b.wifi = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for TerminalConfiguration {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("TerminalConfiguration", 12)?;
        s.serialize_field("bbpos_wisepos_e", &self.bbpos_wisepos_e)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("is_account_default", &self.is_account_default)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("offline", &self.offline)?;
        s.serialize_field("reboot_window", &self.reboot_window)?;
        s.serialize_field("stripe_s700", &self.stripe_s700)?;
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
