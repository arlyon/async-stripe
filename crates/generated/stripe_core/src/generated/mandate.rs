// ======================================
// This file was automatically generated.
// ======================================

use async_stripe_common::resources::Currency;
use async_stripe_core::{
    ids::MandateId,
    params::{Expand, Expandable, Object},
    BaseClient, Client,
};
use serde::{Deserialize, Serialize};

use crate::resources::{CustomerAcceptance, MandatePaymentMethodDetails, PaymentMethod};

/// The resource representing a Stripe "Mandate".
///
/// For more details see <https://stripe.com/docs/api/mandates/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Mandate {
    /// Unique identifier for the object.
    pub id: MandateId,

    pub customer_acceptance: CustomerAcceptance,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_use: Option<MandateMultiUse>,

    /// ID of the payment method associated with this mandate.
    pub payment_method: Expandable<PaymentMethod>,

    pub payment_method_details: MandatePaymentMethodDetails,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_use: Option<MandateSingleUse>,

    /// The status of the mandate, which indicates whether it can be used to initiate a payment.
    pub status: MandateStatus,

    /// The type of the mandate.
    #[serde(rename = "type")]
    pub type_: MandateType,
}

impl Mandate {
    /// Retrieves a Mandate object.
    pub fn retrieve<C: BaseClient>(
        client: &Client<C>,
        id: &MandateId,
        expand: &[&str],
    ) -> <C as BaseClient>::Response<Mandate> {
        client.get_query(&format!("/mandates/{}", id), &Expand { expand })
    }
}

impl Object for Mandate {
    type Id = MandateId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "mandate"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MandateMultiUse {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MandateSingleUse {
    /// On a single use mandate, the amount of the payment.
    pub amount: i64,

    /// On a single use mandate, the currency of the payment.
    pub currency: Currency,
}

/// An enum representing the possible values of an `Mandate`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MandateStatus {
    Active,
    Inactive,
    Pending,
}

impl MandateStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            MandateStatus::Active => "active",
            MandateStatus::Inactive => "inactive",
            MandateStatus::Pending => "pending",
        }
    }
}

impl AsRef<str> for MandateStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for MandateStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for MandateStatus {
    fn default() -> Self {
        Self::Active
    }
}

/// An enum representing the possible values of an `Mandate`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MandateType {
    MultiUse,
    SingleUse,
}

impl MandateType {
    pub fn as_str(self) -> &'static str {
        match self {
            MandateType::MultiUse => "multi_use",
            MandateType::SingleUse => "single_use",
        }
    }
}

impl AsRef<str> for MandateType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for MandateType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for MandateType {
    fn default() -> Self {
        Self::MultiUse
    }
}
