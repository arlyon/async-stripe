// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::IssuingCardId;
use crate::params::{Expandable, Metadata, Object, Timestamp};
use crate::resources::{
    Address, CardBrand, Currency, IssuingCardholder, MerchantCategory, SpendingLimit,
};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "IssuingCard".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingCard {
    /// Unique identifier for the object.
    pub id: IssuingCardId,

    pub authorization_controls: IssuingCardAuthorizationControls,

    /// The brand of the card.
    pub brand: CardBrand,

    /// The [Cardholder](https://stripe.com/docs/api#issuing_cardholder_object) object to which the card belongs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder: Option<IssuingCardholder>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

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

    /// The name of the cardholder, printed on the card.
    pub name: String,

    /// The card this card replaces, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replacement_for: Option<Expandable<IssuingCard>>,

    /// Why the card that this card replaces (if any) needed to be replaced.
    ///
    /// One of `damage`, `expiration`, `loss`, or `theft`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replacement_reason: Option<IssuingCardReplacementReason>,

    /// Where and how the card will be shipped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<IssuingCardShipping>,

    /// One of `active`, `inactive`, `canceled`, `lost`, `stolen`, or `pending`.
    pub status: IssuingCardStatus,

    /// One of `virtual` or `physical`.
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

    /// The currency of the card.
    ///
    /// See [max_amount](https://stripe.com/docs/api#issuing_card_object-authorization_controls-max_amount).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// Maximum amount allowed per authorization on this card, in the currency of the card.
    ///
    /// Authorization amounts in a different currency will be converted to the card's currency when evaluating this control.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_amount: Option<i64>,

    /// Maximum count of approved authorizations on this card.
    ///
    /// Counts all authorizations retroactively.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_approvals: Option<i64>,

    /// Limit the spending with rules based on time intervals and categories.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spending_limits: Option<Vec<SpendingLimit>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingCardShipping {
    pub address: Address,

    /// The delivery service that shipped a physical product, such as Fedex, UPS, USPS, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,

    /// A unix timestamp representing a best estimate of when the card will be delivered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eta: Option<Timestamp>,

    /// Recipient name.
    pub name: String,

    /// The delivery status of the card.
    ///
    /// One of `pending`, `shipped`, `delivered`, `returned`, `failure`, or `canceled`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<IssuingCardShippingStatus>,

    /// A tracking number for a card shipment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<String>,

    /// A link to the shipping carrier's site where you can view detailed information about a card shipment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_url: Option<String>,

    /// One of `bulk` or `individual`.
    ///
    /// Bulk shipments will be grouped and mailed together, while individual ones will not.
    #[serde(rename = "type")]
    pub type_: IssuingCardShippingType,
}

/// An enum representing the possible values of an `IssuingCard`'s `replacement_reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardReplacementReason {
    Damage,
    Expiration,
    Loss,
    Theft,
}

/// An enum representing the possible values of an `IssuingCardShipping`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardShippingStatus {
    Canceled,
    Delivered,
    Failure,
    Pending,
    Returned,
    Shipped,
}

/// An enum representing the possible values of an `IssuingCardShipping`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardShippingType {
    Bulk,
    Individual,
}

/// An enum representing the possible values of an `IssuingCard`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardStatus {
    Active,
    Canceled,
    Inactive,
    Lost,
    Pending,
    Stolen,
}

/// An enum representing the possible values of an `IssuingCard`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardType {
    Physical,
    Virtual,
}
