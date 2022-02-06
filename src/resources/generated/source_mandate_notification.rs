// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::ids::SourceMandateNotificationId;
use crate::params::{Object, Timestamp};
use crate::resources::Source;

/// The resource representing a Stripe "SourceMandateNotification".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceMandateNotification {
    /// Unique identifier for the object.
    pub id: SourceMandateNotificationId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<Box<SourceMandateNotificationAcssDebitData>>,

    /// A positive integer in the smallest currency unit (that is, 100 cents for $1.00, or 1 for ¥1, Japanese Yen being a zero-decimal currency) representing the amount associated with the mandate notification.
    ///
    /// The amount is expressed in the currency of the underlying source.
    /// Required if the notification type is `debit_initiated`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Box<i64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<Box<SourceMandateNotificationBacsDebitData>>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The reason of the mandate notification.
    ///
    /// Valid reasons are `mandate_confirmed` or `debit_initiated`.
    pub reason: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<Box<SourceMandateNotificationSepaDebitData>>,

    pub source: Source,

    /// The status of the mandate notification.
    ///
    /// Valid statuses are `pending` or `submitted`.
    pub status: String,

    /// The type of source this mandate notification is attached to.
    ///
    /// Should be the source type identifier code for the payment method, such as `three_d_secure`.
    #[serde(rename = "type")]
    pub type_: String,
}

impl Object for SourceMandateNotification {
    type Id = SourceMandateNotificationId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "source_mandate_notification"
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceMandateNotificationAcssDebitData {
    /// The statement descriptor associate with the debit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceMandateNotificationBacsDebitData {
    /// Last 4 digits of the account number associated with the debit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceMandateNotificationSepaDebitData {
    /// SEPA creditor ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creditor_identifier: Option<Box<String>>,

    /// Last 4 digits of the account number associated with the debit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<Box<String>>,

    /// Mandate reference associated with the debit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_reference: Option<Box<String>>,
}
