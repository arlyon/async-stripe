/// `Application Fee Refund` objects allow you to refund an application fee that
/// has previously been created but not yet refunded.
///
/// Funds will be refunded to the Stripe account from which the fee was originally collected.  Related guide: [Refunding application fees](https://stripe.com/docs/connect/destination-charges#refunding-app-fee).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FeeRefund {
    /// Amount, in %s.
    pub amount: i64,
    /// Balance transaction that describes the impact on your account balance.
    pub balance_transaction: Option<stripe_types::Expandable<stripe_types::BalanceTransaction>>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// ID of the application fee that was refunded.
    pub fee: stripe_types::Expandable<stripe_types::PlatformFee>,
    /// Unique identifier for the object.
    pub id: stripe_types::fee_refund::FeeRefundId,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: FeeRefundObject,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum FeeRefundObject {
    FeeRefund,
}

impl FeeRefundObject {
    pub fn as_str(self) -> &'static str {
        use FeeRefundObject::*;
        match self {
            FeeRefund => "fee_refund",
        }
    }
}

impl std::str::FromStr for FeeRefundObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FeeRefundObject::*;
        match s {
            "fee_refund" => Ok(FeeRefund),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for FeeRefundObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FeeRefundObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for FeeRefundObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for FeeRefundObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FeeRefundObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for FeeRefundObject"))
    }
}
impl stripe_types::Object for FeeRefund {
    type Id = stripe_types::fee_refund::FeeRefundId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(FeeRefundId);
