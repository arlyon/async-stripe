/// A Mandate is a record of the permission a customer has given you to debit their payment method.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Mandate {
    pub customer_acceptance: stripe_core::mandate::customer_acceptance::CustomerAcceptance,
    /// Unique identifier for the object.
    pub id: stripe_core::mandate::MandateId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_use: Option<stripe_core::mandate::multi_use::MultiUse>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: MandateObject,
    /// ID of the payment method associated with this mandate.
    pub payment_method: stripe_types::Expandable<stripe_core::payment_method::PaymentMethod>,
    pub payment_method_details: stripe_core::mandate::payment_method_details::PaymentMethodDetails,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_use: Option<stripe_core::mandate::single_use::SingleUse>,
    /// The status of the mandate, which indicates whether it can be used to initiate a payment.
    pub status: MandateStatus,
    /// The type of the mandate.
    #[serde(rename = "type")]
    pub type_: MandateType,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Mandate {
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
pub enum MandateObject {
    Mandate,
}

impl MandateObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Mandate => "mandate",
        }
    }
}

impl AsRef<str> for MandateObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for MandateObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The status of the mandate, which indicates whether it can be used to initiate a payment.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum MandateStatus {
    Active,
    Inactive,
    Pending,
}

impl MandateStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::Pending => "pending",
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
/// The type of the mandate.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum MandateType {
    MultiUse,
    SingleUse,
}

impl MandateType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MultiUse => "multi_use",
            Self::SingleUse => "single_use",
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
impl stripe_types::Object for Mandate {
    type Id = stripe_core::mandate::MandateId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(MandateId, "mandate_");
pub mod customer_acceptance;
pub mod requests;
pub use customer_acceptance::CustomerAcceptance;
pub mod au_becs_debit;
pub use au_becs_debit::AuBecsDebit;
pub mod bacs_debit;
pub use bacs_debit::BacsDebit;
pub mod multi_use;
pub use multi_use::MultiUse;
pub mod payment_method_details;
pub use payment_method_details::PaymentMethodDetails;
pub mod single_use;
pub use single_use::SingleUse;
