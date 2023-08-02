/// An Issuing `Cardholder` object represents an individual or business entity who is [issued](https://stripe.com/docs/issuing) cards.
///
/// Related guide: [How to create a cardholder](https://stripe.com/docs/issuing/cards#create-cardholder).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Cardholder {
    pub billing: stripe_types::billing::Billing,
    /// Additional information about a `company` cardholder.
    pub company: Option<stripe_types::company::Company>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The cardholder's email address.
    pub email: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_types::issuing::cardholder::IssuingCardholderId,
    /// Additional information about an `individual` cardholder.
    pub individual: Option<stripe_types::individual::Individual>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// The cardholder's name.
    ///
    /// This will be printed on cards issued to them.
    pub name: String,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: CardholderObject,
    /// The cardholder's phone number.
    ///
    /// This is required for all cardholders who will be creating EU cards.
    /// See the [3D Secure documentation](https://stripe.com/docs/issuing/3d-secure#when-is-3d-secure-applied) for more details.
    pub phone_number: Option<String>,
    /// The cardholder’s preferred locales (languages), ordered by preference.
    ///
    /// Locales can be `de`, `en`, `es`, `fr`, or `it`.  This changes the language of the [3D Secure flow](https://stripe.com/docs/issuing/3d-secure) and one-time password messages sent to the cardholder.
    pub preferred_locales: Option<Vec<CardholderPreferredLocales>>,
    pub requirements: stripe_types::requirements::Requirements,
    /// Rules that control spending across this cardholder's cards.
    ///
    /// Refer to our [documentation](https://stripe.com/docs/issuing/controls/spending-controls) for more details.
    pub spending_controls: Option<stripe_types::spending_controls::SpendingControls>,
    /// Specifies whether to permit authorizations on this cardholder's cards.
    pub status: CardholderStatus,
    /// One of `individual` or `company`.
    ///
    /// See [Choose a cardholder type](https://stripe.com/docs/issuing/other/choose-cardholder) for more details.
    #[serde(rename = "type")]
    pub type_: CardholderType,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CardholderObject {
    IssuingCardholder,
}

impl CardholderObject {
    pub fn as_str(self) -> &'static str {
        use CardholderObject::*;
        match self {
            IssuingCardholder => "issuing.cardholder",
        }
    }
}

impl std::str::FromStr for CardholderObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CardholderObject::*;
        match s {
            "issuing.cardholder" => Ok(IssuingCardholder),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CardholderObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CardholderObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CardholderObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CardholderObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CardholderObject"))
    }
}
/// The cardholder’s preferred locales (languages), ordered by preference.
///
/// Locales can be `de`, `en`, `es`, `fr`, or `it`.  This changes the language of the [3D Secure flow](https://stripe.com/docs/issuing/3d-secure) and one-time password messages sent to the cardholder.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CardholderPreferredLocales {
    De,
    En,
    Es,
    Fr,
    It,
}

impl CardholderPreferredLocales {
    pub fn as_str(self) -> &'static str {
        use CardholderPreferredLocales::*;
        match self {
            De => "de",
            En => "en",
            Es => "es",
            Fr => "fr",
            It => "it",
        }
    }
}

impl std::str::FromStr for CardholderPreferredLocales {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CardholderPreferredLocales::*;
        match s {
            "de" => Ok(De),
            "en" => Ok(En),
            "es" => Ok(Es),
            "fr" => Ok(Fr),
            "it" => Ok(It),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CardholderPreferredLocales {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CardholderPreferredLocales {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CardholderPreferredLocales {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CardholderPreferredLocales {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CardholderPreferredLocales"))
    }
}
/// Specifies whether to permit authorizations on this cardholder's cards.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CardholderStatus {
    Active,
    Blocked,
    Inactive,
}

impl CardholderStatus {
    pub fn as_str(self) -> &'static str {
        use CardholderStatus::*;
        match self {
            Active => "active",
            Blocked => "blocked",
            Inactive => "inactive",
        }
    }
}

impl std::str::FromStr for CardholderStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CardholderStatus::*;
        match s {
            "active" => Ok(Active),
            "blocked" => Ok(Blocked),
            "inactive" => Ok(Inactive),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CardholderStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CardholderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CardholderStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CardholderStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CardholderStatus"))
    }
}
/// One of `individual` or `company`.
///
/// See [Choose a cardholder type](https://stripe.com/docs/issuing/other/choose-cardholder) for more details.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CardholderType {
    Company,
    Individual,
}

impl CardholderType {
    pub fn as_str(self) -> &'static str {
        use CardholderType::*;
        match self {
            Company => "company",
            Individual => "individual",
        }
    }
}

impl std::str::FromStr for CardholderType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CardholderType::*;
        match s {
            "company" => Ok(Company),
            "individual" => Ok(Individual),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CardholderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CardholderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CardholderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CardholderType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for CardholderType"))
    }
}
impl stripe_types::Object for Cardholder {
    type Id = stripe_types::issuing::cardholder::IssuingCardholderId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(IssuingCardholderId, "ich_");
