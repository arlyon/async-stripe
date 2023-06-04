/// An Issuing `Cardholder` object represents an individual or business entity who is [issued](https://stripe.com/docs/issuing) cards.
///
/// Related guide: [How to create a Cardholder](https://stripe.com/docs/issuing/cards#create-cardholder).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Cardholder {
    pub billing: crate::issuing::cardholder::billing::Billing,
    /// Additional information about a `company` cardholder.
    pub company: Option<crate::issuing::cardholder::company::Company>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::Timestamp,
    /// The cardholder's email address.
    pub email: Option<String>,
    /// Unique identifier for the object.
    pub id: crate::issuing::cardholder::IssuingCardholderId,
    /// Additional information about an `individual` cardholder.
    pub individual: Option<crate::issuing::cardholder::individual::Individual>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: crate::Metadata,
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
    pub requirements: crate::issuing::cardholder::requirements::Requirements,
    /// Rules that control spending across this cardholder's cards.
    ///
    /// Refer to our [documentation](https://stripe.com/docs/issuing/controls/spending-controls) for more details.
    pub spending_controls: Option<crate::issuing::cardholder::spending_controls::SpendingControls>,
    /// Specifies whether to permit authorizations on this cardholder's cards.
    pub status: CardholderStatus,
    /// One of `individual` or `company`.
    #[serde(rename = "type")]
    pub type_: CardholderType,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Cardholder {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum CardholderObject {
    #[serde(rename = "issuing.cardholder")]
    IssuingCardholder,
}

impl CardholderObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::IssuingCardholder => "issuing.cardholder",
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
/// Specifies whether to permit authorizations on this cardholder's cards.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
/// One of `individual` or `company`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
impl crate::Object for Cardholder {
    type Id = crate::issuing::cardholder::IssuingCardholderId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
crate::def_id!(IssuingCardholderId, "ich_");
pub mod billing;
pub mod requests;
pub use billing::Billing;
pub mod spending_controls;
pub use spending_controls::SpendingControls;
pub mod company;
pub use company::Company;
pub mod individual;
pub use individual::Individual;
pub mod requirements;
pub use requirements::Requirements;