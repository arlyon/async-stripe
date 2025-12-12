/// A Location represents a grouping of readers.
///
/// Related guide: [Fleet management](https://docs.stripe.com/terminal/fleet/locations)
///
/// For more details see <<https://stripe.com/docs/api/terminal/locations/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalLocation {
    pub address: stripe_shared::Address,
    pub address_kana: Option<stripe_shared::LegalEntityJapanAddress>,
    pub address_kanji: Option<stripe_shared::LegalEntityJapanAddress>,
    /// The ID of a configuration that will be used to customize all readers in this location.
    pub configuration_overrides: Option<String>,
    /// The display name of the location.
    pub display_name: String,
    /// The Kana variation of the display name of the location.
    pub display_name_kana: Option<String>,
    /// The Kanji variation of the display name of the location.
    pub display_name_kanji: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_terminal::TerminalLocationId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// The phone number of the location.
    pub phone: Option<String>,
}
#[doc(hidden)]
pub struct TerminalLocationBuilder {
    address: Option<stripe_shared::Address>,
    address_kana: Option<Option<stripe_shared::LegalEntityJapanAddress>>,
    address_kanji: Option<Option<stripe_shared::LegalEntityJapanAddress>>,
    configuration_overrides: Option<Option<String>>,
    display_name: Option<String>,
    display_name_kana: Option<Option<String>>,
    display_name_kanji: Option<Option<String>>,
    id: Option<stripe_terminal::TerminalLocationId>,
    livemode: Option<bool>,
    metadata: Option<std::collections::HashMap<String, String>>,
    phone: Option<Option<String>>,
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

    impl Deserialize for TerminalLocation {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TerminalLocation>,
        builder: TerminalLocationBuilder,
    }

    impl Visitor for Place<TerminalLocation> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TerminalLocationBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TerminalLocationBuilder {
        type Out = TerminalLocation;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "address" => Deserialize::begin(&mut self.address),
                "address_kana" => Deserialize::begin(&mut self.address_kana),
                "address_kanji" => Deserialize::begin(&mut self.address_kanji),
                "configuration_overrides" => Deserialize::begin(&mut self.configuration_overrides),
                "display_name" => Deserialize::begin(&mut self.display_name),
                "display_name_kana" => Deserialize::begin(&mut self.display_name_kana),
                "display_name_kanji" => Deserialize::begin(&mut self.display_name_kanji),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "phone" => Deserialize::begin(&mut self.phone),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                address: Deserialize::default(),
                address_kana: Deserialize::default(),
                address_kanji: Deserialize::default(),
                configuration_overrides: Deserialize::default(),
                display_name: Deserialize::default(),
                display_name_kana: Deserialize::default(),
                display_name_kanji: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
                phone: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(address),
                Some(address_kana),
                Some(address_kanji),
                Some(configuration_overrides),
                Some(display_name),
                Some(display_name_kana),
                Some(display_name_kanji),
                Some(id),
                Some(livemode),
                Some(metadata),
                Some(phone),
            ) = (
                self.address.take(),
                self.address_kana.take(),
                self.address_kanji.take(),
                self.configuration_overrides.take(),
                self.display_name.take(),
                self.display_name_kana.take(),
                self.display_name_kanji.take(),
                self.id.take(),
                self.livemode,
                self.metadata.take(),
                self.phone.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                address,
                address_kana,
                address_kanji,
                configuration_overrides,
                display_name,
                display_name_kana,
                display_name_kanji,
                id,
                livemode,
                metadata,
                phone,
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

    impl ObjectDeser for TerminalLocation {
        type Builder = TerminalLocationBuilder;
    }

    impl FromValueOpt for TerminalLocation {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TerminalLocationBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "address" => b.address = FromValueOpt::from_value(v),
                    "address_kana" => b.address_kana = FromValueOpt::from_value(v),
                    "address_kanji" => b.address_kanji = FromValueOpt::from_value(v),
                    "configuration_overrides" => {
                        b.configuration_overrides = FromValueOpt::from_value(v)
                    }
                    "display_name" => b.display_name = FromValueOpt::from_value(v),
                    "display_name_kana" => b.display_name_kana = FromValueOpt::from_value(v),
                    "display_name_kanji" => b.display_name_kanji = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "metadata" => b.metadata = FromValueOpt::from_value(v),
                    "phone" => b.phone = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for TerminalLocation {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("TerminalLocation", 12)?;
        s.serialize_field("address", &self.address)?;
        s.serialize_field("address_kana", &self.address_kana)?;
        s.serialize_field("address_kanji", &self.address_kanji)?;
        s.serialize_field("configuration_overrides", &self.configuration_overrides)?;
        s.serialize_field("display_name", &self.display_name)?;
        s.serialize_field("display_name_kana", &self.display_name_kana)?;
        s.serialize_field("display_name_kanji", &self.display_name_kanji)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("phone", &self.phone)?;

        s.serialize_field("object", "terminal.location")?;
        s.end()
    }
}
impl stripe_types::Object for TerminalLocation {
    type Id = stripe_terminal::TerminalLocationId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(TerminalLocationId);
