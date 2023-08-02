/// You can [create physical or virtual cards](https://stripe.com/docs/issuing/cards) that are issued to cardholders.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Card {
    /// The brand of the card.
    pub brand: String,
    /// The reason why the card was canceled.
    pub cancellation_reason: Option<CardCancellationReason>,
    pub cardholder: stripe_types::issuing::cardholder::Cardholder,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Supported currencies are `usd` in the US, `eur` in the EU, and `gbp` in the UK.
    pub currency: stripe_types::Currency,
    /// The card's CVC.
    ///
    /// For security reasons, this is only available for virtual cards, and will be omitted unless you explicitly request it with [the `expand` parameter](https://stripe.com/docs/api/expanding_objects).
    /// Additionally, it's only available via the ["Retrieve a card" endpoint](https://stripe.com/docs/api/issuing/cards/retrieve), not via "List all cards" or any other endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc: Option<String>,
    /// The expiration month of the card.
    pub exp_month: i64,
    /// The expiration year of the card.
    pub exp_year: i64,
    /// The financial account this card is attached to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_account: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_types::issuing::card::IssuingCardId,
    /// The last 4 digits of the card number.
    pub last4: String,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// The full unredacted card number.
    ///
    /// For security reasons, this is only available for virtual cards, and will be omitted unless you explicitly request it with [the `expand` parameter](https://stripe.com/docs/api/expanding_objects).
    /// Additionally, it's only available via the ["Retrieve a card" endpoint](https://stripe.com/docs/api/issuing/cards/retrieve), not via "List all cards" or any other endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: CardObject,
    /// The latest card that replaces this card, if any.
    pub replaced_by: Option<stripe_types::Expandable<stripe_types::issuing::card::Card>>,
    /// The card this card replaces, if any.
    pub replacement_for: Option<stripe_types::Expandable<stripe_types::issuing::card::Card>>,
    /// The reason why the previous card needed to be replaced.
    pub replacement_reason: Option<CardReplacementReason>,
    /// Where and how the card will be shipped.
    pub shipping: Option<stripe_types::shipping::Shipping>,
    pub spending_controls: stripe_types::spending_controls::SpendingControls,
    /// Whether authorizations can be approved on this card.
    ///
    /// May be blocked from activating cards depending on past-due Cardholder requirements.
    /// Defaults to `inactive`.
    pub status: CardStatus,
    /// The type of the card.
    #[serde(rename = "type")]
    pub type_: CardType,
    /// Information relating to digital wallets (like Apple Pay and Google Pay).
    pub wallets: Option<stripe_types::wallets::Wallets>,
}
/// The reason why the card was canceled.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CardCancellationReason {
    DesignRejected,
    Lost,
    Stolen,
}

impl CardCancellationReason {
    pub fn as_str(self) -> &'static str {
        use CardCancellationReason::*;
        match self {
            DesignRejected => "design_rejected",
            Lost => "lost",
            Stolen => "stolen",
        }
    }
}

impl std::str::FromStr for CardCancellationReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CardCancellationReason::*;
        match s {
            "design_rejected" => Ok(DesignRejected),
            "lost" => Ok(Lost),
            "stolen" => Ok(Stolen),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CardCancellationReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CardCancellationReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CardCancellationReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CardCancellationReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CardCancellationReason"))
    }
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CardObject {
    IssuingCard,
}

impl CardObject {
    pub fn as_str(self) -> &'static str {
        use CardObject::*;
        match self {
            IssuingCard => "issuing.card",
        }
    }
}

impl std::str::FromStr for CardObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CardObject::*;
        match s {
            "issuing.card" => Ok(IssuingCard),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CardObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CardObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CardObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CardObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for CardObject"))
    }
}
/// The reason why the previous card needed to be replaced.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CardReplacementReason {
    Damaged,
    Expired,
    Lost,
    Stolen,
}

impl CardReplacementReason {
    pub fn as_str(self) -> &'static str {
        use CardReplacementReason::*;
        match self {
            Damaged => "damaged",
            Expired => "expired",
            Lost => "lost",
            Stolen => "stolen",
        }
    }
}

impl std::str::FromStr for CardReplacementReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CardReplacementReason::*;
        match s {
            "damaged" => Ok(Damaged),
            "expired" => Ok(Expired),
            "lost" => Ok(Lost),
            "stolen" => Ok(Stolen),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CardReplacementReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CardReplacementReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CardReplacementReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CardReplacementReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CardReplacementReason"))
    }
}
/// Whether authorizations can be approved on this card.
///
/// May be blocked from activating cards depending on past-due Cardholder requirements.
/// Defaults to `inactive`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CardStatus {
    Active,
    Canceled,
    Inactive,
}

impl CardStatus {
    pub fn as_str(self) -> &'static str {
        use CardStatus::*;
        match self {
            Active => "active",
            Canceled => "canceled",
            Inactive => "inactive",
        }
    }
}

impl std::str::FromStr for CardStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CardStatus::*;
        match s {
            "active" => Ok(Active),
            "canceled" => Ok(Canceled),
            "inactive" => Ok(Inactive),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CardStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CardStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CardStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CardStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for CardStatus"))
    }
}
/// The type of the card.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CardType {
    Physical,
    Virtual,
}

impl CardType {
    pub fn as_str(self) -> &'static str {
        use CardType::*;
        match self {
            Physical => "physical",
            Virtual => "virtual",
        }
    }
}

impl std::str::FromStr for CardType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CardType::*;
        match s {
            "physical" => Ok(Physical),
            "virtual" => Ok(Virtual),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CardType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CardType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CardType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CardType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for CardType"))
    }
}
impl stripe_types::Object for Card {
    type Id = stripe_types::issuing::card::IssuingCardId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(IssuingCardId, "ic_");
