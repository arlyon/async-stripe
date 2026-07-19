/// A Personalization Design is a logical grouping of a Physical Bundle, card logo, and carrier text that represents a product line.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// A lookup key used to retrieve personalization designs dynamically from a static string.
    /// This may be up to 200 characters.
    pub lookup_key: Option<String>,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingPersonalizationDesign {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingPersonalizationDesign").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: IssuingPersonalizationDesignBuilder {
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
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "card_logo" => Deserialize::begin(&mut self.builder.card_logo),
                "carrier_text" => Deserialize::begin(&mut self.builder.carrier_text),
                "created" => Deserialize::begin(&mut self.builder.created),
                "id" => Deserialize::begin(&mut self.builder.id),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "lookup_key" => Deserialize::begin(&mut self.builder.lookup_key),
                "metadata" => Deserialize::begin(&mut self.builder.metadata),
                "name" => Deserialize::begin(&mut self.builder.name),
                "physical_bundle" => Deserialize::begin(&mut self.builder.physical_bundle),
                "preferences" => Deserialize::begin(&mut self.builder.preferences),
                "rejection_reasons" => Deserialize::begin(&mut self.builder.rejection_reasons),
                "status" => Deserialize::begin(&mut self.builder.status),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
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
                self.builder.card_logo.take(),
                self.builder.carrier_text.take(),
                self.builder.created,
                self.builder.id.take(),
                self.builder.livemode,
                self.builder.lookup_key.take(),
                self.builder.metadata.take(),
                self.builder.name.take(),
                self.builder.physical_bundle.take(),
                self.builder.preferences,
                self.builder.rejection_reasons.take(),
                self.builder.status.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(IssuingPersonalizationDesign {
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
            });
            Ok(())
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum IssuingPersonalizationDesignStatus {
    Active,
    Inactive,
    Rejected,
    Review,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl IssuingPersonalizationDesignStatus {
    pub fn as_str(&self) -> &str {
        use IssuingPersonalizationDesignStatus::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Rejected => "rejected",
            Review => "review",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for IssuingPersonalizationDesignStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingPersonalizationDesignStatus::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "rejected" => Ok(Rejected),
            "review" => Ok(Review),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "IssuingPersonalizationDesignStatus"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for IssuingPersonalizationDesignStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for IssuingPersonalizationDesignStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingPersonalizationDesignStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(IssuingPersonalizationDesignStatus)).finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for IssuingPersonalizationDesignStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<IssuingPersonalizationDesignStatus> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingPersonalizationDesignStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingPersonalizationDesignStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
