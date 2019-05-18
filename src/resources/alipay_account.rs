use crate::ids::{AlipayAccountId, CustomerId};
use crate::params::{Paginated, Metadata, Timestamp};
use crate::resources::Currency;
use serde_derive::{Deserialize, Serialize};

/// The structure of an Alipay account in a `PaymentSource`.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AlipayAccount {
    pub id: AlipayAccountId,

    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// The customer that the Alipay account belong to.
    pub customer: Option<CustomerId>, //.TODO: Expandable

    /// Always true for a deleted object.
    #[serde(default)]
    pub deleted: bool,

    /// Uniquely identifies the account and will be the same across all Alipay account objects that are
    /// linked to the same Alipay account.
    pub fingerprint: String,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object
    /// exists in test mode.
    pub livemode: bool,

    /// Set of key-value pairs that you can attach to an object. This can be useful for storing
    /// additional information about the object in a structured format.
    pub metadata: Metadata,

    /// If the Alipay account object is not reusable, the exact amount that you can create a charge for.
    pub payment_amount: u64,

    /// If the Alipay account object is not reusable, the exact currency that you can create a charge for.
    pub payment_currency: Currency,

    /// True if you can create multiple payments using this account. If the account is reusable, then
    /// you can freely choose the amount of each payment.
    pub reusable: bool,

    /// Whether this Alipay account object has ever been used for a payment.
    pub used: bool,

    // /// The username for the Alipay account.
    pub username: String,
}

impl Paginated for AlipayAccount {
    fn cursor(&self) -> &str {
        &self.id
    }
}
