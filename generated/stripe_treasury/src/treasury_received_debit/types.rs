/// ReceivedDebits represent funds pulled from a [FinancialAccount](https://stripe.com/docs/api#financial_accounts).
/// These are not initiated from the FinancialAccount.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TreasuryReceivedDebit {
    /// Amount (in cents) transferred.
pub amount: i64,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
pub created: stripe_types::Timestamp,
        /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
pub currency: stripe_types::Currency,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
pub description: String,
        /// Reason for the failure.
    /// A ReceivedDebit might fail because the FinancialAccount doesn't have sufficient funds, is closed, or is frozen.
pub failure_code: Option<TreasuryReceivedDebitFailureCode>,
    /// The FinancialAccount that funds were pulled from.
pub financial_account: Option<String>,
        /// A [hosted transaction receipt](https://stripe.com/docs/treasury/moving-money/regulatory-receipts) URL that is provided when money movement is considered regulated under Stripe's money transmission licenses.
pub hosted_regulatory_receipt_url: Option<String>,
    /// Unique identifier for the object.
pub id: stripe_treasury::TreasuryReceivedDebitId,
#[serde(skip_serializing_if = "Option::is_none")]
pub initiating_payment_method_details: Option<stripe_treasury::TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails>,
pub linked_flows: stripe_treasury::TreasuryReceivedDebitsResourceLinkedFlows,
        /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
pub livemode: bool,
    /// The network used for the ReceivedDebit.
pub network: TreasuryReceivedDebitNetwork,
    /// Details describing when a ReceivedDebit might be reversed.
pub reversal_details: Option<stripe_treasury::TreasuryReceivedDebitsResourceReversalDetails>,
        /// Status of the ReceivedDebit.
    /// ReceivedDebits are created with a status of either `succeeded` (approved) or `failed` (declined).
    /// The failure reason can be found under the `failure_code`.
pub status: stripe_treasury::TreasuryReceivedDebitStatus,
    /// The Transaction associated with this object.
pub transaction: Option<stripe_types::Expandable<stripe_treasury::TreasuryTransaction>>,

}
/// Reason for the failure.
/// A ReceivedDebit might fail because the FinancialAccount doesn't have sufficient funds, is closed, or is frozen.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryReceivedDebitFailureCode {
    AccountClosed,
    AccountFrozen,
    InsufficientFunds,
    Other,
}
impl TreasuryReceivedDebitFailureCode {
    pub fn as_str(self) -> &'static str {
        use TreasuryReceivedDebitFailureCode::*;
        match self {
            AccountClosed => "account_closed",
            AccountFrozen => "account_frozen",
            InsufficientFunds => "insufficient_funds",
            Other => "other",
        }
    }
}

impl std::str::FromStr for TreasuryReceivedDebitFailureCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryReceivedDebitFailureCode::*;
        match s {
            "account_closed" => Ok(AccountClosed),
            "account_frozen" => Ok(AccountFrozen),
            "insufficient_funds" => Ok(InsufficientFunds),
            "other" => Ok(Other),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for TreasuryReceivedDebitFailureCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryReceivedDebitFailureCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryReceivedDebitFailureCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryReceivedDebitFailureCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for TreasuryReceivedDebitFailureCode")
        })
    }
}
/// The network used for the ReceivedDebit.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryReceivedDebitNetwork {
    Ach,
    Card,
    Stripe,
}
impl TreasuryReceivedDebitNetwork {
    pub fn as_str(self) -> &'static str {
        use TreasuryReceivedDebitNetwork::*;
        match self {
            Ach => "ach",
            Card => "card",
            Stripe => "stripe",
        }
    }
}

impl std::str::FromStr for TreasuryReceivedDebitNetwork {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryReceivedDebitNetwork::*;
        match s {
            "ach" => Ok(Ach),
            "card" => Ok(Card),
            "stripe" => Ok(Stripe),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for TreasuryReceivedDebitNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryReceivedDebitNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryReceivedDebitNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryReceivedDebitNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for TreasuryReceivedDebitNetwork"))
    }
}
impl stripe_types::Object for TreasuryReceivedDebit {
    type Id = stripe_treasury::TreasuryReceivedDebitId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(TreasuryReceivedDebitId);
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryReceivedDebitStatus {
    Failed,
    Succeeded,
}
impl TreasuryReceivedDebitStatus {
    pub fn as_str(self) -> &'static str {
        use TreasuryReceivedDebitStatus::*;
        match self {
            Failed => "failed",
            Succeeded => "succeeded",
        }
    }
}

impl std::str::FromStr for TreasuryReceivedDebitStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryReceivedDebitStatus::*;
        match s {
            "failed" => Ok(Failed),
            "succeeded" => Ok(Succeeded),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for TreasuryReceivedDebitStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryReceivedDebitStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryReceivedDebitStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryReceivedDebitStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for TreasuryReceivedDebitStatus"))
    }
}
