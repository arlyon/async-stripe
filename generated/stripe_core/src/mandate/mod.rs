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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for MandateObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mandate" => Ok(Self::Mandate),

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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for MandateObject"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for MandateObject {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<MandateObject> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(MandateObject::from_str(s)?);
        Ok(())
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
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::Pending => "pending",
        }
    }
}

impl std::str::FromStr for MandateStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            "pending" => Ok(Self::Pending),

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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for MandateStatus"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for MandateStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<MandateStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(MandateStatus::from_str(s)?);
        Ok(())
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
        match self {
            Self::MultiUse => "multi_use",
            Self::SingleUse => "single_use",
        }
    }
}

impl std::str::FromStr for MandateType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "multi_use" => Ok(Self::MultiUse),
            "single_use" => Ok(Self::SingleUse),

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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for MandateType"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for MandateType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<MandateType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(MandateType::from_str(s)?);
        Ok(())
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
