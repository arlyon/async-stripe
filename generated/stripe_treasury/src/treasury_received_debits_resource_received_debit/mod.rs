/// ReceivedDebits represent funds pulled from a [FinancialAccount](https://stripe.com/docs/api#financial_accounts).
///
/// These are not initiated from the FinancialAccount.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TreasuryReceivedDebitsResourceReceivedDebit {
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
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: String,
    /// Reason for the failure.
    ///
    /// A ReceivedDebit might fail because the FinancialAccount doesn't have sufficient funds, is closed, or is frozen.
    pub failure_code: Option<TreasuryReceivedDebitsResourceReceivedDebitFailureCode>,
    /// The FinancialAccount that funds were pulled from.
    pub financial_account: Option<String>,
    /// A [hosted transaction receipt](https://stripe.com/docs/treasury/moving-money/regulatory-receipts) URL that is provided when money movement is considered regulated under Stripe's money transmission licenses.
    pub hosted_regulatory_receipt_url: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_treasury::treasury_received_debits_resource_received_debit::TreasuryReceivedDebitId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiating_payment_method_details: Option<stripe_treasury::TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails>,
    pub linked_flows: stripe_treasury::TreasuryReceivedDebitsResourceLinkedFlows,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The network used for the ReceivedDebit.
    pub network: TreasuryReceivedDebitsResourceReceivedDebitNetwork,
    /// Details describing when a ReceivedDebit might be reversed.
    pub reversal_details: Option<stripe_treasury::TreasuryReceivedDebitsResourceReversalDetails>,
    /// Status of the ReceivedDebit.
    ///
    /// ReceivedDebits are created with a status of either `succeeded` (approved) or `failed` (declined).
    /// The failure reason can be found under the `failure_code`.
    pub status: TreasuryReceivedDebitsResourceReceivedDebitStatus,
    /// The Transaction associated with this object.
    pub transaction: Option<stripe_types::Expandable<stripe_treasury::TreasuryTransactionsResourceTransaction>>,
}
/// Reason for the failure.
///
/// A ReceivedDebit might fail because the FinancialAccount doesn't have sufficient funds, is closed, or is frozen.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryReceivedDebitsResourceReceivedDebitFailureCode {
    AccountClosed,
    AccountFrozen,
    InsufficientFunds,
    Other,
}

impl TreasuryReceivedDebitsResourceReceivedDebitFailureCode {
    pub fn as_str(self) -> &'static str {
        use TreasuryReceivedDebitsResourceReceivedDebitFailureCode::*;
        match self {
            AccountClosed => "account_closed",
            AccountFrozen => "account_frozen",
            InsufficientFunds => "insufficient_funds",
            Other => "other",
        }
    }
}

impl std::str::FromStr for TreasuryReceivedDebitsResourceReceivedDebitFailureCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryReceivedDebitsResourceReceivedDebitFailureCode::*;
        match s {
            "account_closed" => Ok(AccountClosed),
            "account_frozen" => Ok(AccountFrozen),
            "insufficient_funds" => Ok(InsufficientFunds),
            "other" => Ok(Other),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for TreasuryReceivedDebitsResourceReceivedDebitFailureCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryReceivedDebitsResourceReceivedDebitFailureCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryReceivedDebitsResourceReceivedDebitFailureCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryReceivedDebitsResourceReceivedDebitFailureCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryReceivedDebitsResourceReceivedDebitFailureCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryReceivedDebitsResourceReceivedDebitFailureCode"))
    }
}
/// The network used for the ReceivedDebit.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryReceivedDebitsResourceReceivedDebitNetwork {
    Ach,
    Card,
    Stripe,
}

impl TreasuryReceivedDebitsResourceReceivedDebitNetwork {
    pub fn as_str(self) -> &'static str {
        use TreasuryReceivedDebitsResourceReceivedDebitNetwork::*;
        match self {
            Ach => "ach",
            Card => "card",
            Stripe => "stripe",
        }
    }
}

impl std::str::FromStr for TreasuryReceivedDebitsResourceReceivedDebitNetwork {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryReceivedDebitsResourceReceivedDebitNetwork::*;
        match s {
            "ach" => Ok(Ach),
            "card" => Ok(Card),
            "stripe" => Ok(Stripe),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for TreasuryReceivedDebitsResourceReceivedDebitNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryReceivedDebitsResourceReceivedDebitNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryReceivedDebitsResourceReceivedDebitNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryReceivedDebitsResourceReceivedDebitNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryReceivedDebitsResourceReceivedDebitNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryReceivedDebitsResourceReceivedDebitNetwork"))
    }
}
/// Status of the ReceivedDebit.
///
/// ReceivedDebits are created with a status of either `succeeded` (approved) or `failed` (declined).
/// The failure reason can be found under the `failure_code`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryReceivedDebitsResourceReceivedDebitStatus {
    Failed,
    Succeeded,
}

impl TreasuryReceivedDebitsResourceReceivedDebitStatus {
    pub fn as_str(self) -> &'static str {
        use TreasuryReceivedDebitsResourceReceivedDebitStatus::*;
        match self {
            Failed => "failed",
            Succeeded => "succeeded",
        }
    }
}

impl std::str::FromStr for TreasuryReceivedDebitsResourceReceivedDebitStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryReceivedDebitsResourceReceivedDebitStatus::*;
        match s {
            "failed" => Ok(Failed),
            "succeeded" => Ok(Succeeded),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for TreasuryReceivedDebitsResourceReceivedDebitStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryReceivedDebitsResourceReceivedDebitStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryReceivedDebitsResourceReceivedDebitStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryReceivedDebitsResourceReceivedDebitStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryReceivedDebitsResourceReceivedDebitStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryReceivedDebitsResourceReceivedDebitStatus"))
    }
}
impl stripe_types::Object for TreasuryReceivedDebitsResourceReceivedDebit {
    type Id = stripe_treasury::treasury_received_debits_resource_received_debit::TreasuryReceivedDebitId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(TreasuryReceivedDebitId);
#[cfg(feature = "treasury_received_debits_resource_received_debit")]
mod requests;
#[cfg(feature = "treasury_received_debits_resource_received_debit")]
pub use requests::*;
