/// You can [create physical or virtual cards](https://stripe.com/docs/issuing/cards) that are issued to cardholders.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IssuingCard {
    /// The brand of the card.
    pub brand: String,
    /// The reason why the card was canceled.
    pub cancellation_reason: Option<IssuingCardCancellationReason>,
    pub cardholder: stripe_types::IssuingCardholder,
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
    pub id: stripe_types::issuing_card::IssuingCardId,
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
    /// The latest card that replaces this card, if any.
    pub replaced_by: Option<stripe_types::Expandable<stripe_types::IssuingCard>>,
    /// The card this card replaces, if any.
    pub replacement_for: Option<stripe_types::Expandable<stripe_types::IssuingCard>>,
    /// The reason why the previous card needed to be replaced.
    pub replacement_reason: Option<IssuingCardReplacementReason>,
    /// Where and how the card will be shipped.
    pub shipping: Option<stripe_types::IssuingCardShipping>,
    pub spending_controls: stripe_types::IssuingCardAuthorizationControls,
    /// Whether authorizations can be approved on this card.
    ///
    /// May be blocked from activating cards depending on past-due Cardholder requirements.
    /// Defaults to `inactive`.
    pub status: IssuingCardStatus,
    /// The type of the card.
    #[serde(rename = "type")]
    pub type_: IssuingCardType,
    /// Information relating to digital wallets (like Apple Pay and Google Pay).
    pub wallets: Option<stripe_types::IssuingCardWallets>,
}
/// The reason why the card was canceled.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingCardCancellationReason {
    DesignRejected,
    Lost,
    Stolen,
}

impl IssuingCardCancellationReason {
    pub fn as_str(self) -> &'static str {
        use IssuingCardCancellationReason::*;
        match self {
            DesignRejected => "design_rejected",
            Lost => "lost",
            Stolen => "stolen",
        }
    }
}

impl std::str::FromStr for IssuingCardCancellationReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingCardCancellationReason::*;
        match s {
            "design_rejected" => Ok(DesignRejected),
            "lost" => Ok(Lost),
            "stolen" => Ok(Stolen),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for IssuingCardCancellationReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingCardCancellationReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingCardCancellationReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingCardCancellationReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingCardCancellationReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for IssuingCardCancellationReason")
        })
    }
}
/// The reason why the previous card needed to be replaced.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingCardReplacementReason {
    Damaged,
    Expired,
    Lost,
    Stolen,
}

impl IssuingCardReplacementReason {
    pub fn as_str(self) -> &'static str {
        use IssuingCardReplacementReason::*;
        match self {
            Damaged => "damaged",
            Expired => "expired",
            Lost => "lost",
            Stolen => "stolen",
        }
    }
}

impl std::str::FromStr for IssuingCardReplacementReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingCardReplacementReason::*;
        match s {
            "damaged" => Ok(Damaged),
            "expired" => Ok(Expired),
            "lost" => Ok(Lost),
            "stolen" => Ok(Stolen),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for IssuingCardReplacementReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingCardReplacementReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingCardReplacementReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingCardReplacementReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingCardReplacementReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for IssuingCardReplacementReason"))
    }
}
/// Whether authorizations can be approved on this card.
///
/// May be blocked from activating cards depending on past-due Cardholder requirements.
/// Defaults to `inactive`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingCardStatus {
    Active,
    Canceled,
    Inactive,
}

impl IssuingCardStatus {
    pub fn as_str(self) -> &'static str {
        use IssuingCardStatus::*;
        match self {
            Active => "active",
            Canceled => "canceled",
            Inactive => "inactive",
        }
    }
}

impl std::str::FromStr for IssuingCardStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingCardStatus::*;
        match s {
            "active" => Ok(Active),
            "canceled" => Ok(Canceled),
            "inactive" => Ok(Inactive),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for IssuingCardStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingCardStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingCardStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingCardStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingCardStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for IssuingCardStatus"))
    }
}
/// The type of the card.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingCardType {
    Physical,
    Virtual,
}

impl IssuingCardType {
    pub fn as_str(self) -> &'static str {
        use IssuingCardType::*;
        match self {
            Physical => "physical",
            Virtual => "virtual",
        }
    }
}

impl std::str::FromStr for IssuingCardType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingCardType::*;
        match s {
            "physical" => Ok(Physical),
            "virtual" => Ok(Virtual),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for IssuingCardType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingCardType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingCardType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for IssuingCardType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IssuingCardType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for IssuingCardType"))
    }
}
impl stripe_types::Object for IssuingCard {
    type Id = stripe_types::issuing_card::IssuingCardId;
    fn id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}
stripe_types::def_id!(IssuingCardId, "ic_");
