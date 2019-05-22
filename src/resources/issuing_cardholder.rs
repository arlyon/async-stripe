// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::IssuingCardholderId;
use crate::params::{Metadata, Object, Timestamp};
use crate::resources::{Address, MerchantCategory, SpendingLimit};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "IssuingCardholder".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingCardholder {
    /// Unique identifier for the object.
    pub id: IssuingCardholderId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_controls: Option<IssuingCardholderAuthorizationControls>,

    pub billing: IssuingCardholderAddress,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// The cardholder's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Whether or not this cardholder is the default cardholder.
    pub is_default: bool,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    /// The cardholder's name.
    ///
    /// This will be printed on cards issued to them.
    pub name: String,

    /// The cardholder's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,

    /// One of `active`, `inactive`, `blocked`, or `pending`.
    pub status: IssuingCardholderStatus,

    /// One of `individual` or `business_entity`.
    #[serde(rename = "type")]
    pub type_: IssuingCardholderType,
}

impl Object for IssuingCardholder {
    type Id = IssuingCardholderId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "issuing.cardholder"
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingCardholderAddress {
    pub address: Address,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingCardholderAuthorizationControls {
    /// Array of strings containing [categories](https://stripe.com/docs/api#issuing_authorization_object-merchant_data-category) of authorizations permitted on this card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_categories: Option<Vec<MerchantCategory>>,

    /// Array of strings containing [categories](https://stripe.com/docs/api#issuing_authorization_object-merchant_data-category) of authorizations to always decline on this card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_categories: Option<Vec<MerchantCategory>>,

    /// Limit the spending with rules based on time intervals and categories.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spending_limits: Option<Vec<SpendingLimit>>,
}

/// An enum representing the possible values of an `IssuingCardholder`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardholderStatus {
    Active,
    Blocked,
    Inactive,
    Pending,
}

/// An enum representing the possible values of an `IssuingCardholder`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardholderType {
    BusinessEntity,
    Individual,
}
