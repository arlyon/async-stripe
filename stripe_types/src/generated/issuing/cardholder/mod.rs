/// An Issuing `Cardholder` object represents an individual or business entity who is [issued](https://stripe.com/docs/issuing) cards.
///
/// Related guide: [How to create a Cardholder](https://stripe.com/docs/issuing/cards#create-cardholder).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Cardholder {
    pub billing: stripe_types::issuing::cardholder::billing::Billing,
    /// Additional information about a `company` cardholder.
    pub company: Option<stripe_types::issuing::cardholder::company::Company>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The cardholder's email address.
    pub email: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_types::issuing::cardholder::IssuingCardholderId,
    /// Additional information about an `individual` cardholder.
    pub individual: Option<stripe_types::issuing::cardholder::individual::Individual>,
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
    pub requirements: stripe_types::issuing::cardholder::requirements::Requirements,
    /// Rules that control spending across this cardholder's cards.
    ///
    /// Refer to our [documentation](https://stripe.com/docs/issuing/controls/spending-controls) for more details.
    pub spending_controls:
        Option<stripe_types::issuing::cardholder::spending_controls::SpendingControls>,
    /// Specifies whether to permit authorizations on this cardholder's cards.
    pub status: CardholderStatus,
    /// One of `individual` or `company`.
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
        match self {
            Self::IssuingCardholder => "issuing.cardholder",
        }
    }
}

impl std::str::FromStr for CardholderObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "issuing.cardholder" => Ok(Self::IssuingCardholder),

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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CardholderObject"))
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
        match self {
            Self::Active => "active",
            Self::Blocked => "blocked",
            Self::Inactive => "inactive",
        }
    }
}

impl std::str::FromStr for CardholderStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "blocked" => Ok(Self::Blocked),
            "inactive" => Ok(Self::Inactive),

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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CardholderStatus"))
    }
}
/// One of `individual` or `company`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CardholderType {
    Company,
    Individual,
}

impl CardholderType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Company => "company",
            Self::Individual => "individual",
        }
    }
}

impl std::str::FromStr for CardholderType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "company" => Ok(Self::Company),
            "individual" => Ok(Self::Individual),

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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CardholderType"))
    }
}
impl stripe_types::Object for Cardholder {
    type Id = stripe_types::issuing::cardholder::IssuingCardholderId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(IssuingCardholderId, "ich_");
pub mod billing;
pub use billing::Billing;
pub mod spending_controls;
pub use spending_controls::SpendingControls;
pub mod company;
pub use company::Company;
pub mod individual;
pub use individual::Individual;
pub mod requirements;
pub use requirements::Requirements;
