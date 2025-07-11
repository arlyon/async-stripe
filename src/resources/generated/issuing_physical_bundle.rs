// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{IssuingPhysicalBundleId};
use crate::params::{Object};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "IssuingPhysicalBundle".
///
/// For more details see <https://stripe.com/docs/api/issuing/physical_bundles/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingPhysicalBundle {
    /// Unique identifier for the object.
    pub id: IssuingPhysicalBundleId,

    pub features: IssuingPhysicalBundleFeatures,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Friendly display name.
    pub name: String,

    /// Whether this physical bundle can be used to create cards.
    pub status: IssuingPhysicalBundleStatus,

    /// Whether this physical bundle is a standard Stripe offering or custom-made for you.
    #[serde(rename = "type")]
    pub type_: IssuingPhysicalBundleType,
}

impl Object for IssuingPhysicalBundle {
    type Id = IssuingPhysicalBundleId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "issuing.physical_bundle"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingPhysicalBundleFeatures {

    /// The policy for how to use card logo images in a card design with this physical bundle.
    pub card_logo: IssuingPhysicalBundleFeaturesCardLogo,

    /// The policy for how to use carrier letter text in a card design with this physical bundle.
    pub carrier_text: IssuingPhysicalBundleFeaturesCarrierText,

    /// The policy for how to use a second line on a card with this physical bundle.
    pub second_line: IssuingPhysicalBundleFeaturesSecondLine,
}

/// An enum representing the possible values of an `IssuingPhysicalBundleFeatures`'s `card_logo` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingPhysicalBundleFeaturesCardLogo {
    Optional,
    Required,
    Unsupported,
}

impl IssuingPhysicalBundleFeaturesCardLogo {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingPhysicalBundleFeaturesCardLogo::Optional => "optional",
            IssuingPhysicalBundleFeaturesCardLogo::Required => "required",
            IssuingPhysicalBundleFeaturesCardLogo::Unsupported => "unsupported",
        }
    }
}

impl AsRef<str> for IssuingPhysicalBundleFeaturesCardLogo {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingPhysicalBundleFeaturesCardLogo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for IssuingPhysicalBundleFeaturesCardLogo {
    fn default() -> Self {
        Self::Optional
    }
}

/// An enum representing the possible values of an `IssuingPhysicalBundleFeatures`'s `carrier_text` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingPhysicalBundleFeaturesCarrierText {
    Optional,
    Required,
    Unsupported,
}

impl IssuingPhysicalBundleFeaturesCarrierText {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingPhysicalBundleFeaturesCarrierText::Optional => "optional",
            IssuingPhysicalBundleFeaturesCarrierText::Required => "required",
            IssuingPhysicalBundleFeaturesCarrierText::Unsupported => "unsupported",
        }
    }
}

impl AsRef<str> for IssuingPhysicalBundleFeaturesCarrierText {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingPhysicalBundleFeaturesCarrierText {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for IssuingPhysicalBundleFeaturesCarrierText {
    fn default() -> Self {
        Self::Optional
    }
}

/// An enum representing the possible values of an `IssuingPhysicalBundleFeatures`'s `second_line` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingPhysicalBundleFeaturesSecondLine {
    Optional,
    Required,
    Unsupported,
}

impl IssuingPhysicalBundleFeaturesSecondLine {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingPhysicalBundleFeaturesSecondLine::Optional => "optional",
            IssuingPhysicalBundleFeaturesSecondLine::Required => "required",
            IssuingPhysicalBundleFeaturesSecondLine::Unsupported => "unsupported",
        }
    }
}

impl AsRef<str> for IssuingPhysicalBundleFeaturesSecondLine {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingPhysicalBundleFeaturesSecondLine {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for IssuingPhysicalBundleFeaturesSecondLine {
    fn default() -> Self {
        Self::Optional
    }
}

/// An enum representing the possible values of an `IssuingPhysicalBundle`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingPhysicalBundleStatus {
    Active,
    Inactive,
    Review,
}

impl IssuingPhysicalBundleStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingPhysicalBundleStatus::Active => "active",
            IssuingPhysicalBundleStatus::Inactive => "inactive",
            IssuingPhysicalBundleStatus::Review => "review",
        }
    }
}

impl AsRef<str> for IssuingPhysicalBundleStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingPhysicalBundleStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for IssuingPhysicalBundleStatus {
    fn default() -> Self {
        Self::Active
    }
}

/// An enum representing the possible values of an `IssuingPhysicalBundle`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingPhysicalBundleType {
    Custom,
    Standard,
}

impl IssuingPhysicalBundleType {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingPhysicalBundleType::Custom => "custom",
            IssuingPhysicalBundleType::Standard => "standard",
        }
    }
}

impl AsRef<str> for IssuingPhysicalBundleType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingPhysicalBundleType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for IssuingPhysicalBundleType {
    fn default() -> Self {
        Self::Custom
    }
}
