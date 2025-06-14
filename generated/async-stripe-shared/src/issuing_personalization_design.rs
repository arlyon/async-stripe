/// A Personalization Design is a logical grouping of a Physical Bundle, card logo, and carrier text that represents a product line.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingPersonalizationDesign {
    /// The file for the card logo to use with physical bundles that support card logos.
    /// Must have a `purpose` value of `issuing_logo`.
    pub card_logo: Option<stripe_types::Expandable<stripe_shared::File>>,
    /// Hash containing carrier text, for use with physical bundles that support carrier text.
    pub carrier_text: Option<stripe_shared::IssuingPersonalizationDesignCarrierText>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Unique identifier for the object.
    pub id: stripe_shared::IssuingPersonalizationDesignId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// A lookup key used to retrieve personalization designs dynamically from a static string.
    /// This may be up to 200 characters.
    pub lookup_key: Option<String>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// Friendly display name.
    pub name: Option<String>,
    /// The physical bundle object belonging to this personalization design.
    pub physical_bundle: stripe_types::Expandable<stripe_shared::IssuingPhysicalBundle>,
    pub preferences: stripe_shared::IssuingPersonalizationDesignPreferences,
    pub rejection_reasons: stripe_shared::IssuingPersonalizationDesignRejectionReasons,
    /// Whether this personalization design can be used to create cards.
    pub status: stripe_shared::IssuingPersonalizationDesignStatus,
}
#[doc(hidden)]
pub struct IssuingPersonalizationDesignBuilder {
    card_logo: Option<Option<stripe_types::Expandable<stripe_shared::File>>>,
    carrier_text: Option<Option<stripe_shared::IssuingPersonalizationDesignCarrierText>>,
    created: Option<stripe_types::Timestamp>,
    id: Option<stripe_shared::IssuingPersonalizationDesignId>,
    livemode: Option<bool>,
    lookup_key: Option<Option<String>>,
    metadata: Option<std::collections::HashMap<String, String>>,
    name: Option<Option<String>>,
    physical_bundle: Option<stripe_types::Expandable<stripe_shared::IssuingPhysicalBundle>>,
    preferences: Option<stripe_shared::IssuingPersonalizationDesignPreferences>,
    rejection_reasons: Option<stripe_shared::IssuingPersonalizationDesignRejectionReasons>,
    status: Option<stripe_shared::IssuingPersonalizationDesignStatus>,
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

    impl Deserialize for IssuingPersonalizationDesign {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingPersonalizationDesign>,
        builder: IssuingPersonalizationDesignBuilder,
    }

    impl Visitor for Place<IssuingPersonalizationDesign> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingPersonalizationDesignBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingPersonalizationDesignBuilder {
        type Out = IssuingPersonalizationDesign;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "card_logo" => Deserialize::begin(&mut self.card_logo),
                "carrier_text" => Deserialize::begin(&mut self.carrier_text),
                "created" => Deserialize::begin(&mut self.created),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "lookup_key" => Deserialize::begin(&mut self.lookup_key),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "name" => Deserialize::begin(&mut self.name),
                "physical_bundle" => Deserialize::begin(&mut self.physical_bundle),
                "preferences" => Deserialize::begin(&mut self.preferences),
                "rejection_reasons" => Deserialize::begin(&mut self.rejection_reasons),
                "status" => Deserialize::begin(&mut self.status),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                card_logo: Deserialize::default(),
                carrier_text: Deserialize::default(),
                created: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                lookup_key: Deserialize::default(),
                metadata: Deserialize::default(),
                name: Deserialize::default(),
                physical_bundle: Deserialize::default(),
                preferences: Deserialize::default(),
                rejection_reasons: Deserialize::default(),
                status: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(card_logo),
                Some(carrier_text),
                Some(created),
                Some(id),
                Some(livemode),
                Some(lookup_key),
                Some(metadata),
                Some(name),
                Some(physical_bundle),
                Some(preferences),
                Some(rejection_reasons),
                Some(status),
            ) = (
                self.card_logo.take(),
                self.carrier_text.take(),
                self.created,
                self.id.take(),
                self.livemode,
                self.lookup_key.take(),
                self.metadata.take(),
                self.name.take(),
                self.physical_bundle.take(),
                self.preferences,
                self.rejection_reasons.take(),
                self.status,
            )
            else {
                return None;
            };
            Some(Self::Out {
                card_logo,
                carrier_text,
                created,
                id,
                livemode,
                lookup_key,
                metadata,
                name,
                physical_bundle,
                preferences,
                rejection_reasons,
                status,
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

    impl ObjectDeser for IssuingPersonalizationDesign {
        type Builder = IssuingPersonalizationDesignBuilder;
    }

    impl FromValueOpt for IssuingPersonalizationDesign {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingPersonalizationDesignBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "card_logo" => b.card_logo = FromValueOpt::from_value(v),
                    "carrier_text" => b.carrier_text = FromValueOpt::from_value(v),
                    "created" => b.created = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "lookup_key" => b.lookup_key = FromValueOpt::from_value(v),
                    "metadata" => b.metadata = FromValueOpt::from_value(v),
                    "name" => b.name = FromValueOpt::from_value(v),
                    "physical_bundle" => b.physical_bundle = FromValueOpt::from_value(v),
                    "preferences" => b.preferences = FromValueOpt::from_value(v),
                    "rejection_reasons" => b.rejection_reasons = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingPersonalizationDesign {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("IssuingPersonalizationDesign", 13)?;
        s.serialize_field("card_logo", &self.card_logo)?;
        s.serialize_field("carrier_text", &self.carrier_text)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("lookup_key", &self.lookup_key)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("physical_bundle", &self.physical_bundle)?;
        s.serialize_field("preferences", &self.preferences)?;
        s.serialize_field("rejection_reasons", &self.rejection_reasons)?;
        s.serialize_field("status", &self.status)?;

        s.serialize_field("object", "issuing.personalization_design")?;
        s.end()
    }
}
impl stripe_types::Object for IssuingPersonalizationDesign {
    type Id = stripe_shared::IssuingPersonalizationDesignId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(IssuingPersonalizationDesignId);
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingPersonalizationDesignStatus {
    Active,
    Inactive,
    Rejected,
    Review,
}
impl IssuingPersonalizationDesignStatus {
    pub fn as_str(self) -> &'static str {
        use IssuingPersonalizationDesignStatus::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Rejected => "rejected",
            Review => "review",
        }
    }
}

impl std::str::FromStr for IssuingPersonalizationDesignStatus {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingPersonalizationDesignStatus::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "rejected" => Ok(Rejected),
            "review" => Ok(Review),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for IssuingPersonalizationDesignStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingPersonalizationDesignStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingPersonalizationDesignStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for IssuingPersonalizationDesignStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IssuingPersonalizationDesignStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(IssuingPersonalizationDesignStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingPersonalizationDesignStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingPersonalizationDesignStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for IssuingPersonalizationDesignStatus")
        })
    }
}
