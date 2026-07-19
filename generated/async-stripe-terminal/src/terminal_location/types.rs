/// A Location represents a grouping of readers.
///
/// Related guide: [Fleet management](https://docs.stripe.com/terminal/fleet/locations)
///
/// For more details see <<https://stripe.com/docs/api/terminal/locations/object>>.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// The phone number of the location.
    pub phone: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TerminalLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TerminalLocation").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: TerminalLocationBuilder {
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
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "address" => Deserialize::begin(&mut self.builder.address),
                "address_kana" => Deserialize::begin(&mut self.builder.address_kana),
                "address_kanji" => Deserialize::begin(&mut self.builder.address_kanji),
                "configuration_overrides" => {
                    Deserialize::begin(&mut self.builder.configuration_overrides)
                }
                "display_name" => Deserialize::begin(&mut self.builder.display_name),
                "display_name_kana" => Deserialize::begin(&mut self.builder.display_name_kana),
                "display_name_kanji" => Deserialize::begin(&mut self.builder.display_name_kanji),
                "id" => Deserialize::begin(&mut self.builder.id),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "metadata" => Deserialize::begin(&mut self.builder.metadata),
                "phone" => Deserialize::begin(&mut self.builder.phone),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
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
                self.builder.address.take(),
                self.builder.address_kana.take(),
                self.builder.address_kanji.take(),
                self.builder.configuration_overrides.take(),
                self.builder.display_name.take(),
                self.builder.display_name_kana.take(),
                self.builder.display_name_kanji.take(),
                self.builder.id.take(),
                self.builder.livemode,
                self.builder.metadata.take(),
                self.builder.phone.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(TerminalLocation {
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
            });
            Ok(())
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
