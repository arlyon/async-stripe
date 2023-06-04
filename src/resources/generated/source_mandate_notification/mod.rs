/// Source mandate notifications should be created when a notification related to
/// a source mandate must be sent to the payer.
///
/// They will trigger a webhook or deliver an email to the customer.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SourceMandateNotification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<crate::source_mandate_notification::acss_debit_data::AcssDebitData>,
    /// A positive integer in the smallest currency unit (that is, 100 cents for $1.00, or 1 for ¥1, Japanese Yen being a zero-decimal currency) representing the amount associated with the mandate notification.
    ///
    /// The amount is expressed in the currency of the underlying source.
    /// Required if the notification type is `debit_initiated`.
    pub amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<crate::source_mandate_notification::bacs_debit_data::BacsDebitData>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::Timestamp,
    /// Unique identifier for the object.
    pub id: crate::source_mandate_notification::SourceMandateNotificationId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: SourceMandateNotificationObject,
    /// The reason of the mandate notification.
    ///
    /// Valid reasons are `mandate_confirmed` or `debit_initiated`.
    pub reason: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<crate::source_mandate_notification::sepa_debit_data::SepaDebitData>,
    pub source: crate::source::Source,
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
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SourceMandateNotification {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum SourceMandateNotificationObject {
    SourceMandateNotification,
}

impl SourceMandateNotificationObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SourceMandateNotification => "source_mandate_notification",
        }
    }
}

impl AsRef<str> for SourceMandateNotificationObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SourceMandateNotificationObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl crate::Object for SourceMandateNotification {
    type Id = crate::source_mandate_notification::SourceMandateNotificationId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
crate::def_id!(SourceMandateNotificationId);
pub mod acss_debit_data;
pub use acss_debit_data::AcssDebitData;
pub mod bacs_debit_data;
pub use bacs_debit_data::BacsDebitData;
pub mod sepa_debit_data;
pub use sepa_debit_data::SepaDebitData;
