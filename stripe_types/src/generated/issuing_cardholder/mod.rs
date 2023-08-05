/// An Issuing `Cardholder` object represents an individual or business entity who is [issued](https://stripe.com/docs/issuing) cards.
///
/// Related guide: [How to create a cardholder](https://stripe.com/docs/issuing/cards#create-cardholder).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IssuingCardholder {
    pub billing: stripe_types::IssuingCardholderAddress,
    /// Additional information about a `company` cardholder.
    pub company: Option<stripe_types::IssuingCardholderCompany>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The cardholder's email address.
    pub email: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_types::issuing_cardholder::IssuingCardholderId,
    /// Additional information about an `individual` cardholder.
    pub individual: Option<stripe_types::IssuingCardholderIndividual>,
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
    /// The cardholder's phone number.
    ///
    /// This is required for all cardholders who will be creating EU cards.
    /// See the [3D Secure documentation](https://stripe.com/docs/issuing/3d-secure#when-is-3d-secure-applied) for more details.
    pub phone_number: Option<String>,
    /// The cardholder’s preferred locales (languages), ordered by preference.
    ///
    /// Locales can be `de`, `en`, `es`, `fr`, or `it`.  This changes the language of the [3D Secure flow](https://stripe.com/docs/issuing/3d-secure) and one-time password messages sent to the cardholder.
    pub preferred_locales: Option<Vec<IssuingCardholderPreferredLocales>>,
    pub requirements: stripe_types::IssuingCardholderRequirements,
    /// Rules that control spending across this cardholder's cards.
    ///
    /// Refer to our [documentation](https://stripe.com/docs/issuing/controls/spending-controls) for more details.
    pub spending_controls: Option<stripe_types::IssuingCardholderAuthorizationControls>,
    /// Specifies whether to permit authorizations on this cardholder's cards.
    pub status: IssuingCardholderStatus,
    /// One of `individual` or `company`.
    ///
    /// See [Choose a cardholder type](https://stripe.com/docs/issuing/other/choose-cardholder) for more details.
    #[serde(rename = "type")]
    pub type_: IssuingCardholderType,
}
/// The cardholder’s preferred locales (languages), ordered by preference.
///
/// Locales can be `de`, `en`, `es`, `fr`, or `it`.  This changes the language of the [3D Secure flow](https://stripe.com/docs/issuing/3d-secure) and one-time password messages sent to the cardholder.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingCardholderPreferredLocales {
    De,
    En,
    Es,
    Fr,
    It,
}

impl IssuingCardholderPreferredLocales {
    pub fn as_str(self) -> &'static str {
        use IssuingCardholderPreferredLocales::*;
        match self {
            De => "de",
            En => "en",
            Es => "es",
            Fr => "fr",
            It => "it",
        }
    }
}

impl std::str::FromStr for IssuingCardholderPreferredLocales {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingCardholderPreferredLocales::*;
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

impl AsRef<str> for IssuingCardholderPreferredLocales {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingCardholderPreferredLocales {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingCardholderPreferredLocales {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingCardholderPreferredLocales {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingCardholderPreferredLocales {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for IssuingCardholderPreferredLocales")
        })
    }
}
/// Specifies whether to permit authorizations on this cardholder's cards.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingCardholderStatus {
    Active,
    Blocked,
    Inactive,
}

impl IssuingCardholderStatus {
    pub fn as_str(self) -> &'static str {
        use IssuingCardholderStatus::*;
        match self {
            Active => "active",
            Blocked => "blocked",
            Inactive => "inactive",
        }
    }
}

impl std::str::FromStr for IssuingCardholderStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingCardholderStatus::*;
        match s {
            "active" => Ok(Active),
            "blocked" => Ok(Blocked),
            "inactive" => Ok(Inactive),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for IssuingCardholderStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingCardholderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingCardholderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingCardholderStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingCardholderStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for IssuingCardholderStatus"))
    }
}
/// One of `individual` or `company`.
///
/// See [Choose a cardholder type](https://stripe.com/docs/issuing/other/choose-cardholder) for more details.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingCardholderType {
    Company,
    Individual,
}

impl IssuingCardholderType {
    pub fn as_str(self) -> &'static str {
        use IssuingCardholderType::*;
        match self {
            Company => "company",
            Individual => "individual",
        }
    }
}

impl std::str::FromStr for IssuingCardholderType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingCardholderType::*;
        match s {
            "company" => Ok(Company),
            "individual" => Ok(Individual),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for IssuingCardholderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingCardholderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingCardholderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingCardholderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingCardholderType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for IssuingCardholderType"))
    }
}
impl stripe_types::Object for IssuingCardholder {
    type Id = stripe_types::issuing_cardholder::IssuingCardholderId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(IssuingCardholderId, "ich_");
