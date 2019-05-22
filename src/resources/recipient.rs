// ======================================
// This file was automatically generated.
// ======================================

use crate::config::{Client, Response};
use crate::ids::RecipientId;
use crate::params::{Expand, Expandable, List, Metadata, Object, RangeQuery, Timestamp};
use crate::resources::{Account, BankAccount, Card};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "TransferRecipient".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Recipient {
    /// Unique identifier for the object.
    pub id: RecipientId,

    /// Hash describing the current account on the recipient, if there is one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_account: Option<BankAccount>,

    #[serde(default)]
    pub cards: List<Card>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<Timestamp>,

    /// The default card to use for creating transfers to this recipient.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_card: Option<Expandable<Card>>,

    // Always true for a deleted object
    #[serde(default)]
    deleted: bool,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    #[serde(default)]
    pub livemode: bool,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(default)]
    pub metadata: Metadata,

    /// The ID of the [Custom account](https://stripe.com/docs/connect/custom-accounts) this recipient was migrated to.
    ///
    /// If set, the recipient can no longer be updated, nor can transfers be made to it: use the Custom account instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrated_to: Option<Expandable<Account>>,

    /// Full, legal name of the recipient.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rolled_back_from: Option<Expandable<Account>>,

    /// Type of the recipient, one of `individual` or `corporation`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<RecipientType>,
}

impl Recipient {
    /// Returns a list of your recipients.
    ///
    /// The recipients are returned sorted by creation date, with the most recently created recipients appearing first.
    pub fn list(client: &Client, params: RecipientListParams<'_>) -> Response<List<Recipient>> {
        client.get_query("/recipients", &params)
    }
}

impl Object for Recipient {
    type Id = RecipientId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "recipient"
    }
}

/// The parameters for `Recipient::list`.
#[derive(Clone, Debug, Serialize)]
pub struct RecipientListParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<RangeQuery<Timestamp>>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a RecipientId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u64>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a RecipientId>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<RecipientType>,

    /// Only return recipients that are verified or unverified.
    #[serde(skip_serializing_if = "Option::is_none")]
    verified: Option<bool>,
}

impl<'a> RecipientListParams<'a> {
    pub fn new() -> Self {
        RecipientListParams {
            created: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
            type_: Default::default(),
            verified: Default::default(),
        }
    }
}

/// An enum representing the possible values of an `Recipient`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RecipientType {
    Corporation,
    Individual,
}
