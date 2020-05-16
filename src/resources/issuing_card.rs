// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::IssuingCardId;
use crate::params::{Expandable, Metadata, Object, Timestamp};
use crate::resources::{
    Address, CardBrand, Currency, IssuingCardShippingStatus, IssuingCardShippingType,
    IssuingCardType, IssuingCardholder, MerchantCategory, SpendingLimit,
};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "IssuingCard".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingCard {
    /// Unique identifier for the object.
    pub id: IssuingCardId,

    /// The brand of the card.
    pub brand: CardBrand,

    /// The reason why the card was canceled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<IssuingCardCancellationReason>,

    pub cardholder: IssuingCardholder,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

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

    /// The last 4 digits of the card number.
    pub last4: String,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    /// The full unredacted card number.
    ///
    /// For security reasons, this is only available for virtual cards, and will be omitted unless you explicitly request it with [the `expand` parameter](https://stripe.com/docs/api/expanding_objects).
    /// Additionally, it's only available via the ["Retrieve a card" endpoint](https://stripe.com/docs/api/issuing/cards/retrieve), not via "List all cards" or any other endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,

    /// The latest card that replaces this card, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaced_by: Option<Expandable<IssuingCard>>,

    /// The card this card replaces, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replacement_for: Option<Expandable<IssuingCard>>,

    /// The reason why the previous card needed to be replaced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replacement_reason: Option<IssuingCardReplacementReason>,

    /// Where and how the card will be shipped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<IssuingCardShipping>,

    pub spending_controls: IssuingCardAuthorizationControls,

    /// Whether authorizations can be approved on this card.
    pub status: IssuingCardStatus,

    /// The type of the card.
    #[serde(rename = "type")]
    pub type_: IssuingCardType,
}

impl Object for IssuingCard {
    type Id = IssuingCardId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "issuing.card"
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingCardAuthorizationControls {
    /// Array of strings containing [categories](https://stripe.com/docs/api#issuing_authorization_object-merchant_data-category) of authorizations permitted on this card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_categories: Option<Vec<MerchantCategory>>,

    /// Array of strings containing [categories](https://stripe.com/docs/api#issuing_authorization_object-merchant_data-category) of authorizations to always decline on this card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_categories: Option<Vec<MerchantCategory>>,

    /// Limit the spending with rules based on time intervals and categories.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spending_limits: Option<Vec<SpendingLimit>>,

    /// Currency for the amounts within spending_limits.
    ///
    /// Locked to the currency of the card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spending_limits_currency: Option<Currency>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingCardShipping {
    pub address: Address,

    /// The delivery company that shipped a card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<IssuingCardShippingCarrier>,

    /// A unix timestamp representing a best estimate of when the card will be delivered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eta: Option<Timestamp>,

    /// Recipient name.
    pub name: String,

    /// Shipment service, such as `standard` or `express`.
    pub service: IssuingCardShippingService,

    /// The delivery status of the card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<IssuingCardShippingStatus>,

    /// A tracking number for a card shipment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<String>,

    /// A link to the shipping carrier's site where you can view detailed information about a card shipment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_url: Option<String>,

    /// Packaging options.
    #[serde(rename = "type")]
    pub type_: IssuingCardShippingType,
}

/// An enum representing the possible values of an `IssuingCard`'s `cancellation_reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardCancellationReason {
    Lost,
    Stolen,
}

impl IssuingCardCancellationReason {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingCardCancellationReason::Lost => "lost",
            IssuingCardCancellationReason::Stolen => "stolen",
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
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `IssuingCard`'s `replacement_reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardReplacementReason {
    Damaged,
    Expired,
    Lost,
    Stolen,
}

impl IssuingCardReplacementReason {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingCardReplacementReason::Damaged => "damaged",
            IssuingCardReplacementReason::Expired => "expired",
            IssuingCardReplacementReason::Lost => "lost",
            IssuingCardReplacementReason::Stolen => "stolen",
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
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `IssuingCardShipping`'s `carrier` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardShippingCarrier {
    Fedex,
    Usps,
}

impl IssuingCardShippingCarrier {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingCardShippingCarrier::Fedex => "fedex",
            IssuingCardShippingCarrier::Usps => "usps",
        }
    }
}

impl AsRef<str> for IssuingCardShippingCarrier {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingCardShippingCarrier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `IssuingCardShipping`'s `service` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardShippingService {
    Express,
    Priority,
    Standard,
}

impl IssuingCardShippingService {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingCardShippingService::Express => "express",
            IssuingCardShippingService::Priority => "priority",
            IssuingCardShippingService::Standard => "standard",
        }
    }
}

impl AsRef<str> for IssuingCardShippingService {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingCardShippingService {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `IssuingCard`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardStatus {
    Active,
    Canceled,
    Inactive,
}

impl IssuingCardStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingCardStatus::Active => "active",
            IssuingCardStatus::Canceled => "canceled",
            IssuingCardStatus::Inactive => "inactive",
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
        self.as_str().fmt(f)
    }
}
