// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{IssuingPersonalizationDesignId};
use crate::params::{Expandable, Metadata, Object, Timestamp};
use crate::resources::{File, IssuingPhysicalBundle};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "IssuingPersonalizationDesign".
///
/// For more details see <https://stripe.com/docs/api/issuing/personalization_designs/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingPersonalizationDesign {
    /// Unique identifier for the object.
    pub id: IssuingPersonalizationDesignId,

    /// The file for the card logo to use with physical bundles that support card logos.
    ///
    /// Must have a `purpose` value of `issuing_logo`.
    pub card_logo: Option<Expandable<File>>,

    /// Hash containing carrier text, for use with physical bundles that support carrier text.
    pub carrier_text: Option<IssuingPersonalizationDesignCarrierText>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// A lookup key used to retrieve personalization designs dynamically from a static string.
    ///
    /// This may be up to 200 characters.
    pub lookup_key: Option<String>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    /// Friendly display name.
    pub name: Option<String>,

    /// The physical bundle object belonging to this personalization design.
    pub physical_bundle: Expandable<IssuingPhysicalBundle>,

    pub preferences: IssuingPersonalizationDesignPreferences,

    pub rejection_reasons: IssuingPersonalizationDesignRejectionReasons,

    /// Whether this personalization design can be used to create cards.
    pub status: IssuingPersonalizationDesignStatus,
}

impl Object for IssuingPersonalizationDesign {
    type Id = IssuingPersonalizationDesignId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "issuing.personalization_design"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingPersonalizationDesignCarrierText {

    /// The footer body text of the carrier letter.
    pub footer_body: Option<String>,

    /// The footer title text of the carrier letter.
    pub footer_title: Option<String>,

    /// The header body text of the carrier letter.
    pub header_body: Option<String>,

    /// The header title text of the carrier letter.
    pub header_title: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingPersonalizationDesignPreferences {

    /// Whether we use this personalization design to create cards when one isn't specified.
    ///
    /// A connected account uses the Connect platform's default design if no personalization design is set as the default design.
    pub is_default: bool,

    /// Whether this personalization design is used to create cards when one is not specified and a default for this connected account does not exist.
    pub is_platform_default: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingPersonalizationDesignRejectionReasons {

    /// The reason(s) the card logo was rejected.
    pub card_logo: Option<Vec<IssuingPersonalizationDesignRejectionReasonsCardLogo>>,

    /// The reason(s) the carrier text was rejected.
    pub carrier_text: Option<Vec<IssuingPersonalizationDesignRejectionReasonsCarrierText>>,
}

/// An enum representing the possible values of an `IssuingPersonalizationDesignRejectionReasons`'s `card_logo` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingPersonalizationDesignRejectionReasonsCardLogo {
    GeographicLocation,
    Inappropriate,
    NetworkName,
    NonBinaryImage,
    NonFiatCurrency,
    Other,
    OtherEntity,
    PromotionalMaterial,
}

impl IssuingPersonalizationDesignRejectionReasonsCardLogo {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingPersonalizationDesignRejectionReasonsCardLogo::GeographicLocation => "geographic_location",
            IssuingPersonalizationDesignRejectionReasonsCardLogo::Inappropriate => "inappropriate",
            IssuingPersonalizationDesignRejectionReasonsCardLogo::NetworkName => "network_name",
            IssuingPersonalizationDesignRejectionReasonsCardLogo::NonBinaryImage => "non_binary_image",
            IssuingPersonalizationDesignRejectionReasonsCardLogo::NonFiatCurrency => "non_fiat_currency",
            IssuingPersonalizationDesignRejectionReasonsCardLogo::Other => "other",
            IssuingPersonalizationDesignRejectionReasonsCardLogo::OtherEntity => "other_entity",
            IssuingPersonalizationDesignRejectionReasonsCardLogo::PromotionalMaterial => "promotional_material",
        }
    }
}

impl AsRef<str> for IssuingPersonalizationDesignRejectionReasonsCardLogo {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingPersonalizationDesignRejectionReasonsCardLogo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for IssuingPersonalizationDesignRejectionReasonsCardLogo {
    fn default() -> Self {
        Self::GeographicLocation
    }
}

/// An enum representing the possible values of an `IssuingPersonalizationDesignRejectionReasons`'s `carrier_text` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingPersonalizationDesignRejectionReasonsCarrierText {
    GeographicLocation,
    Inappropriate,
    NetworkName,
    NonFiatCurrency,
    Other,
    OtherEntity,
    PromotionalMaterial,
}

impl IssuingPersonalizationDesignRejectionReasonsCarrierText {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingPersonalizationDesignRejectionReasonsCarrierText::GeographicLocation => "geographic_location",
            IssuingPersonalizationDesignRejectionReasonsCarrierText::Inappropriate => "inappropriate",
            IssuingPersonalizationDesignRejectionReasonsCarrierText::NetworkName => "network_name",
            IssuingPersonalizationDesignRejectionReasonsCarrierText::NonFiatCurrency => "non_fiat_currency",
            IssuingPersonalizationDesignRejectionReasonsCarrierText::Other => "other",
            IssuingPersonalizationDesignRejectionReasonsCarrierText::OtherEntity => "other_entity",
            IssuingPersonalizationDesignRejectionReasonsCarrierText::PromotionalMaterial => "promotional_material",
        }
    }
}

impl AsRef<str> for IssuingPersonalizationDesignRejectionReasonsCarrierText {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingPersonalizationDesignRejectionReasonsCarrierText {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for IssuingPersonalizationDesignRejectionReasonsCarrierText {
    fn default() -> Self {
        Self::GeographicLocation
    }
}

/// An enum representing the possible values of an `IssuingPersonalizationDesign`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingPersonalizationDesignStatus {
    Active,
    Inactive,
    Rejected,
    Review,
}

impl IssuingPersonalizationDesignStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingPersonalizationDesignStatus::Active => "active",
            IssuingPersonalizationDesignStatus::Inactive => "inactive",
            IssuingPersonalizationDesignStatus::Rejected => "rejected",
            IssuingPersonalizationDesignStatus::Review => "review",
        }
    }
}

impl AsRef<str> for IssuingPersonalizationDesignStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingPersonalizationDesignStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for IssuingPersonalizationDesignStatus {
    fn default() -> Self {
        Self::Active
    }
}
