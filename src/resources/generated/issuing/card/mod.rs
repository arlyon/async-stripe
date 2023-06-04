/// You can [create physical or virtual cards](https://stripe.com/docs/issuing/cards) that are issued to cardholders.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Card {
    /// The brand of the card.
    pub brand: String,
    /// The reason why the card was canceled.
    pub cancellation_reason: Option<CardCancellationReason>,
    pub cardholder: crate::issuing::cardholder::Cardholder,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Supported currencies are `usd` in the US, `eur` in the EU, and `gbp` in the UK.
    pub currency: crate::Currency,
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
    pub id: crate::issuing::card::IssuingCardId,
    /// The last 4 digits of the card number.
    pub last4: String,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: crate::Metadata,
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
    pub replaced_by: Option<crate::Expandable<crate::issuing::card::Card>>,
    /// The card this card replaces, if any.
    pub replacement_for: Option<crate::Expandable<crate::issuing::card::Card>>,
    /// The reason why the previous card needed to be replaced.
    pub replacement_reason: Option<CardReplacementReason>,
    /// Where and how the card will be shipped.
    pub shipping: Option<crate::issuing::card::shipping::Shipping>,
    pub spending_controls: crate::issuing::card::spending_controls::SpendingControls,
    /// Whether authorizations can be approved on this card.
    pub status: CardStatus,
    /// The type of the card.
    #[serde(rename = "type")]
    pub type_: CardType,
    /// Information relating to digital wallets (like Apple Pay and Google Pay).
    pub wallets: Option<crate::issuing::card::wallets::Wallets>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Card {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The reason why the card was canceled.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum CardCancellationReason {
    DesignRejected,
    Lost,
    Stolen,
}

impl CardCancellationReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DesignRejected => "design_rejected",
            Self::Lost => "lost",
            Self::Stolen => "stolen",
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
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum CardObject {
    #[serde(rename = "issuing.card")]
    IssuingCard,
}

impl CardObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::IssuingCard => "issuing.card",
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
/// The reason why the previous card needed to be replaced.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum CardReplacementReason {
    Damaged,
    Expired,
    Lost,
    Stolen,
}

impl CardReplacementReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Damaged => "damaged",
            Self::Expired => "expired",
            Self::Lost => "lost",
            Self::Stolen => "stolen",
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
/// Whether authorizations can be approved on this card.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum CardStatus {
    Active,
    Canceled,
    Inactive,
}

impl CardStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Canceled => "canceled",
            Self::Inactive => "inactive",
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
/// The type of the card.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum CardType {
    Physical,
    Virtual,
}

impl CardType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Physical => "physical",
            Self::Virtual => "virtual",
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
impl crate::Object for Card {
    type Id = crate::issuing::card::IssuingCardId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
crate::def_id!(IssuingCardId, "ic_");
pub mod requests;
pub mod spending_controls;
pub use spending_controls::SpendingControls;
pub mod shipping;
pub use shipping::Shipping;
pub mod wallets;
pub use wallets::Wallets;
