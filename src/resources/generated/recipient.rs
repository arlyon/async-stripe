// ======================================
// This file was automatically generated.
// ======================================

use crate::config::{Client, Response};
use crate::ids::RecipientId;
use crate::params::{Deleted, Expand, Expandable, List, Metadata, Object, RangeQuery, Timestamp};
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
    pub deleted: bool,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub livemode: Option<bool>,

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
    pub fn list(client: &Client, params: ListRecipients<'_>) -> Response<List<Recipient>> {
        client.get_query("/recipients", &params)
    }

    /// Creates a new `Recipient` object and verifies the recipient’s identity.
    /// Also verifies the recipient’s bank account information or debit card, if either is provided.
    pub fn create(client: &Client, params: CreateRecipient<'_>) -> Response<Recipient> {
        client.post_form("/recipients", &params)
    }

    /// Retrieves the details of an existing recipient.
    ///
    /// You need only supply the unique recipient identifier that was returned upon recipient creation.
    pub fn retrieve(client: &Client, id: &RecipientId, expand: &[&str]) -> Response<Recipient> {
        client.get_query(&format!("/recipients/{}", id), &Expand { expand })
    }

    /// Updates the specified recipient by setting the values of the parameters passed.
    /// Any parameters not provided will be left unchanged.
    ///
    /// If you update the name or tax ID, the identity verification will automatically be rerun.
    /// If you update the bank account, the bank account validation will automatically be rerun.
    pub fn update(
        client: &Client,
        id: &RecipientId,
        params: UpdateRecipient<'_>,
    ) -> Response<Recipient> {
        client.post_form(&format!("/recipients/{}", id), &params)
    }

    /// Permanently deletes a recipient.
    ///
    /// It cannot be undone.
    pub fn delete(client: &Client, id: &RecipientId) -> Response<Deleted<RecipientId>> {
        client.delete(&format!("/recipients/{}", id))
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

/// The parameters for `Recipient::create`.
#[derive(Clone, Debug, Serialize)]
pub struct CreateRecipient<'a> {
    /// An arbitrary string which you can attach to a `Recipient` object.
    ///
    /// It is displayed alongside the recipient in the web interface.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,

    /// The recipient's email address.
    ///
    /// It is displayed alongside the recipient in the web interface, and can be useful for searching and tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The recipient's full, legal name.
    ///
    /// For type `individual`, should be in the format `First Last`, `First Middle Last`, or `First M Last` (no prefixes or suffixes).
    /// For `corporation`, the full, incorporated name.
    pub name: &'a str,

    /// The recipient's tax ID, as a string.
    ///
    /// For type `individual`, the full SSN; for type `corporation`, the full EIN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<&'a str>,

    /// Type of the recipient: either `individual` or `corporation`.
    #[serde(rename = "type")]
    pub type_: RecipientType,
}

impl<'a> CreateRecipient<'a> {
    pub fn new(name: &'a str, type_: RecipientType) -> Self {
        CreateRecipient {
            description: Default::default(),
            email: Default::default(),
            expand: Default::default(),
            metadata: Default::default(),
            name,
            tax_id: Default::default(),
            type_,
        }
    }
}

/// The parameters for `Recipient::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListRecipients<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<RecipientId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<RecipientId>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<RecipientType>,

    /// Only return recipients that are verified or unverified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
}

impl<'a> ListRecipients<'a> {
    pub fn new() -> Self {
        ListRecipients {
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

/// The parameters for `Recipient::update`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct UpdateRecipient<'a> {
    /// ID of the card to set as the recipient's new default for payouts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_card: Option<&'a str>,

    /// An arbitrary string which you can attach to a `Recipient` object.
    ///
    /// It is displayed alongside the recipient in the web interface.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,

    /// The recipient's email address.
    ///
    /// It is displayed alongside the recipient in the web interface, and can be useful for searching and tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The recipient's full, legal name.
    ///
    /// For type `individual`, should be in the format `First Last`, `First Middle Last`, or `First M Last` (no prefixes or suffixes).
    /// For `corporation`, the full, incorporated name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,

    /// The recipient's tax ID, as a string.
    ///
    /// For type `individual`, the full SSN; for type `corporation`, the full EIN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<&'a str>,
}

impl<'a> UpdateRecipient<'a> {
    pub fn new() -> Self {
        UpdateRecipient {
            default_card: Default::default(),
            description: Default::default(),
            email: Default::default(),
            expand: Default::default(),
            metadata: Default::default(),
            name: Default::default(),
            tax_id: Default::default(),
        }
    }
}

/// An enum representing the possible values of an `ListRecipients`'s `type_` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RecipientType {
    Corporation,
    Individual,
}

impl RecipientType {
    pub fn as_str(self) -> &'static str {
        match self {
            RecipientType::Corporation => "corporation",
            RecipientType::Individual => "individual",
        }
    }
}

impl AsRef<str> for RecipientType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for RecipientType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
