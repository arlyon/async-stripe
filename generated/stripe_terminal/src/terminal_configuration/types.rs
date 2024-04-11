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
    pub offline: Option<stripe_terminal::TerminalConfigurationConfigurationResourceOfflineConfig>,
    pub tipping: Option<stripe_terminal::TerminalConfigurationConfigurationResourceTipping>,
    pub verifone_p400:
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,
}
#[doc(hidden)]
pub struct TerminalConfigurationBuilder {
    bbpos_wisepos_e: Option<
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,
    >,
    id: Option<stripe_terminal::TerminalConfigurationId>,
    is_account_default: Option<Option<bool>>,
    livemode: Option<bool>,
    offline:
        Option<Option<stripe_terminal::TerminalConfigurationConfigurationResourceOfflineConfig>>,
    tipping: Option<Option<stripe_terminal::TerminalConfigurationConfigurationResourceTipping>>,
    verifone_p400: Option<
        Option<stripe_terminal::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,
    >,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
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
                "offline" => Deserialize::begin(&mut self.offline),
                "tipping" => Deserialize::begin(&mut self.tipping),
                "verifone_p400" => Deserialize::begin(&mut self.verifone_p400),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                bbpos_wisepos_e: Deserialize::default(),
                id: Deserialize::default(),
                is_account_default: Deserialize::default(),
                livemode: Deserialize::default(),
                offline: Deserialize::default(),
                tipping: Deserialize::default(),
                verifone_p400: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                bbpos_wisepos_e: self.bbpos_wisepos_e.take()?,
                id: self.id.take()?,
                is_account_default: self.is_account_default?,
                livemode: self.livemode?,
                offline: self.offline?,
                tipping: self.tipping.take()?,
                verifone_p400: self.verifone_p400.take()?,
            })
        }
    }

    impl<'a> Map for Builder<'a> {
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
                    "bbpos_wisepos_e" => b.bbpos_wisepos_e = Some(FromValueOpt::from_value(v)?),
                    "id" => b.id = Some(FromValueOpt::from_value(v)?),
                    "is_account_default" => {
                        b.is_account_default = Some(FromValueOpt::from_value(v)?)
                    }
                    "livemode" => b.livemode = Some(FromValueOpt::from_value(v)?),
                    "offline" => b.offline = Some(FromValueOpt::from_value(v)?),
                    "tipping" => b.tipping = Some(FromValueOpt::from_value(v)?),
                    "verifone_p400" => b.verifone_p400 = Some(FromValueOpt::from_value(v)?),

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
        let mut s = s.serialize_struct("TerminalConfiguration", 8)?;
        s.serialize_field("bbpos_wisepos_e", &self.bbpos_wisepos_e)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("is_account_default", &self.is_account_default)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("offline", &self.offline)?;
        s.serialize_field("tipping", &self.tipping)?;
        s.serialize_field("verifone_p400", &self.verifone_p400)?;

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
