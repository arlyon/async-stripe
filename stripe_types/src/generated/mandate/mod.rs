/// A Mandate is a record of the permission a customer has given you to debit their payment method.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Mandate {
    pub customer_acceptance: stripe_types::customer_acceptance::CustomerAcceptance,
    /// Unique identifier for the object.
    pub id: stripe_types::mandate::MandateId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_use: Option<stripe_types::multi_use::MultiUse>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: MandateObject,
    /// The account (if any) for which the mandate is intended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,
    /// ID of the payment method associated with this mandate.
    pub payment_method: stripe_types::Expandable<stripe_types::payment_method::PaymentMethod>,
    pub payment_method_details: stripe_types::payment_method_details::PaymentMethodDetails,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_use: Option<stripe_types::single_use::SingleUse>,
    /// The status of the mandate, which indicates whether it can be used to initiate a payment.
    pub status: MandateStatus,
    /// The type of the mandate.
    #[serde(rename = "type")]
    pub type_: MandateType,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MandateObject {
    Mandate,
}

impl MandateObject {
    pub fn as_str(self) -> &'static str {
        use MandateObject::*;
        match self {
            Mandate => "mandate",
        }
    }
}

impl std::str::FromStr for MandateObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MandateObject::*;
        match s {
            "mandate" => Ok(Mandate),
            _ => Err(()),
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
impl serde::Serialize for MandateObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for MandateObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for MandateObject"))
    }
}
/// The status of the mandate, which indicates whether it can be used to initiate a payment.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MandateStatus {
    Active,
    Inactive,
    Pending,
}

impl MandateStatus {
    pub fn as_str(self) -> &'static str {
        use MandateStatus::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Pending => "pending",
        }
    }
}

impl std::str::FromStr for MandateStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MandateStatus::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "pending" => Ok(Pending),
            _ => Err(()),
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
impl serde::Serialize for MandateStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for MandateStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for MandateStatus"))
    }
}
/// The type of the mandate.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MandateType {
    MultiUse,
    SingleUse,
}

impl MandateType {
    pub fn as_str(self) -> &'static str {
        use MandateType::*;
        match self {
            MultiUse => "multi_use",
            SingleUse => "single_use",
        }
    }
}

impl std::str::FromStr for MandateType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MandateType::*;
        match s {
            "multi_use" => Ok(MultiUse),
            "single_use" => Ok(SingleUse),
            _ => Err(()),
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
impl serde::Serialize for MandateType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for MandateType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for MandateType"))
    }
}
impl stripe_types::Object for Mandate {
    type Id = stripe_types::mandate::MandateId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(MandateId, "mandate_");
