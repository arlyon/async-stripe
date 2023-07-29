/// You can reverse some [ReceivedCredits](https://stripe.com/docs/api#received_credits) depending on their network and source flow.
///
/// Reversing a ReceivedCredit leads to the creation of a new object known as a CreditReversal.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CreditReversal {
    /// Amount (in cents) transferred.
    pub amount: i64,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The FinancialAccount to reverse funds from.
    pub financial_account: String,
    /// A [hosted transaction receipt](https://stripe.com/docs/treasury/moving-money/regulatory-receipts) URL that is provided when money movement is considered regulated under Stripe's money transmission licenses.
    pub hosted_regulatory_receipt_url: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_treasury::treasury::credit_reversal::TreasuryCreditReversalId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// The rails used to reverse the funds.
    pub network: CreditReversalNetwork,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: CreditReversalObject,
    /// The ReceivedCredit being reversed.
    pub received_credit: String,
    /// Status of the CreditReversal.
    pub status: CreditReversalStatus,
    pub status_transitions:
        stripe_treasury::treasury::received_credit::status_transitions::StatusTransitions,
    /// The Transaction associated with this object.
    pub transaction:
        Option<stripe_types::Expandable<stripe_treasury::treasury::transaction::Transaction>>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CreditReversal {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The rails used to reverse the funds.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreditReversalNetwork {
    Ach,
    Stripe,
}

impl CreditReversalNetwork {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ach => "ach",
            Self::Stripe => "stripe",
        }
    }
}

impl std::str::FromStr for CreditReversalNetwork {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ach" => Ok(Self::Ach),
            "stripe" => Ok(Self::Stripe),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreditReversalNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreditReversalNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CreditReversalNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CreditReversalNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CreditReversalNetwork"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CreditReversalNetwork {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<CreditReversalNetwork> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CreditReversalNetwork::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreditReversalObject {
    TreasuryCreditReversal,
}

impl CreditReversalObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TreasuryCreditReversal => "treasury.credit_reversal",
        }
    }
}

impl std::str::FromStr for CreditReversalObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "treasury.credit_reversal" => Ok(Self::TreasuryCreditReversal),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreditReversalObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreditReversalObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CreditReversalObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CreditReversalObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CreditReversalObject"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CreditReversalObject {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<CreditReversalObject> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CreditReversalObject::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Status of the CreditReversal.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreditReversalStatus {
    Canceled,
    Posted,
    Processing,
}

impl CreditReversalStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Canceled => "canceled",
            Self::Posted => "posted",
            Self::Processing => "processing",
        }
    }
}

impl std::str::FromStr for CreditReversalStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "canceled" => Ok(Self::Canceled),
            "posted" => Ok(Self::Posted),
            "processing" => Ok(Self::Processing),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreditReversalStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreditReversalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CreditReversalStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CreditReversalStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CreditReversalStatus"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CreditReversalStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<CreditReversalStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CreditReversalStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
impl stripe_types::Object for CreditReversal {
    type Id = stripe_treasury::treasury::credit_reversal::TreasuryCreditReversalId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(TreasuryCreditReversalId);
