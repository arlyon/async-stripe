// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};
use stripe::{
    ids::RecipientId,
    params::{Expandable, List, Metadata, Object, Timestamp},
    resources::{Account, BankAccount, Card},
};

/// The resource representing a Stripe "TransferRecipient".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
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
    pub created: Timestamp,

    /// The default card to use for creating transfers to this recipient.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_card: Option<Expandable<Card>>,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
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
    pub type_: crate::RecipientType,
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
