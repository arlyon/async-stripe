/// You can reverse some [ReceivedCredits](https://stripe.com/docs/api#received_credits) depending on their network and source flow.
///
/// Reversing a ReceivedCredit leads to the creation of a new object known as a CreditReversal.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TreasuryReceivedCreditsResourceCreditReversal {
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
    pub id: stripe_treasury::treasury_received_credits_resource_credit_reversal::TreasuryCreditReversalId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// The rails used to reverse the funds.
    pub network: TreasuryReceivedCreditsResourceCreditReversalNetwork,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: TreasuryReceivedCreditsResourceCreditReversalObject,
    /// The ReceivedCredit being reversed.
    pub received_credit: String,
    /// Status of the CreditReversal.
    pub status: TreasuryReceivedCreditsResourceCreditReversalStatus,
    pub status_transitions: stripe_treasury::TreasuryReceivedCreditsResourceStatusTransitions,
    /// The Transaction associated with this object.
    pub transaction: Option<stripe_types::Expandable<stripe_treasury::TreasuryTransactionsResourceTransaction>>,
}
/// The rails used to reverse the funds.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryReceivedCreditsResourceCreditReversalNetwork {
    Ach,
    Stripe,
}

impl TreasuryReceivedCreditsResourceCreditReversalNetwork {
    pub fn as_str(self) -> &'static str {
        use TreasuryReceivedCreditsResourceCreditReversalNetwork::*;
        match self {
            Ach => "ach",
            Stripe => "stripe",
        }
    }
}

impl std::str::FromStr for TreasuryReceivedCreditsResourceCreditReversalNetwork {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryReceivedCreditsResourceCreditReversalNetwork::*;
        match s {
            "ach" => Ok(Ach),
            "stripe" => Ok(Stripe),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for TreasuryReceivedCreditsResourceCreditReversalNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryReceivedCreditsResourceCreditReversalNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryReceivedCreditsResourceCreditReversalNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryReceivedCreditsResourceCreditReversalNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryReceivedCreditsResourceCreditReversalNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryReceivedCreditsResourceCreditReversalNetwork"))
    }
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryReceivedCreditsResourceCreditReversalObject {
    TreasuryCreditReversal,
}

impl TreasuryReceivedCreditsResourceCreditReversalObject {
    pub fn as_str(self) -> &'static str {
        use TreasuryReceivedCreditsResourceCreditReversalObject::*;
        match self {
            TreasuryCreditReversal => "treasury.credit_reversal",
        }
    }
}

impl std::str::FromStr for TreasuryReceivedCreditsResourceCreditReversalObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryReceivedCreditsResourceCreditReversalObject::*;
        match s {
            "treasury.credit_reversal" => Ok(TreasuryCreditReversal),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for TreasuryReceivedCreditsResourceCreditReversalObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryReceivedCreditsResourceCreditReversalObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryReceivedCreditsResourceCreditReversalObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryReceivedCreditsResourceCreditReversalObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryReceivedCreditsResourceCreditReversalObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryReceivedCreditsResourceCreditReversalObject"))
    }
}
/// Status of the CreditReversal.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryReceivedCreditsResourceCreditReversalStatus {
    Canceled,
    Posted,
    Processing,
}

impl TreasuryReceivedCreditsResourceCreditReversalStatus {
    pub fn as_str(self) -> &'static str {
        use TreasuryReceivedCreditsResourceCreditReversalStatus::*;
        match self {
            Canceled => "canceled",
            Posted => "posted",
            Processing => "processing",
        }
    }
}

impl std::str::FromStr for TreasuryReceivedCreditsResourceCreditReversalStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryReceivedCreditsResourceCreditReversalStatus::*;
        match s {
            "canceled" => Ok(Canceled),
            "posted" => Ok(Posted),
            "processing" => Ok(Processing),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for TreasuryReceivedCreditsResourceCreditReversalStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryReceivedCreditsResourceCreditReversalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryReceivedCreditsResourceCreditReversalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryReceivedCreditsResourceCreditReversalStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryReceivedCreditsResourceCreditReversalStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryReceivedCreditsResourceCreditReversalStatus"))
    }
}
impl stripe_types::Object for TreasuryReceivedCreditsResourceCreditReversal {
    type Id = stripe_treasury::treasury_received_credits_resource_credit_reversal::TreasuryCreditReversalId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(TreasuryCreditReversalId);
pub mod requests;
