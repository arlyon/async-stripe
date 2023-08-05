/// ReceivedCredits represent funds sent to a [FinancialAccount](https://stripe.com/docs/api#financial_accounts) (for example, via ACH or wire).
///
/// These money movements are not initiated from the FinancialAccount.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TreasuryReceivedCreditsResourceReceivedCredit {
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
    /// A ReceivedCredit might fail because the receiving FinancialAccount is closed or frozen.
    pub failure_code: Option<TreasuryReceivedCreditsResourceReceivedCreditFailureCode>,
    /// The FinancialAccount that received the funds.
    pub financial_account: Option<String>,
    /// A [hosted transaction receipt](https://stripe.com/docs/treasury/moving-money/regulatory-receipts) URL that is provided when money movement is considered regulated under Stripe's money transmission licenses.
    pub hosted_regulatory_receipt_url: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_treasury::treasury_received_credits_resource_received_credit::TreasuryReceivedCreditId,
    pub initiating_payment_method_details: stripe_treasury::TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails,
    pub linked_flows: stripe_treasury::TreasuryReceivedCreditsResourceLinkedFlows,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The rails used to send the funds.
    pub network: TreasuryReceivedCreditsResourceReceivedCreditNetwork,
    /// Details describing when a ReceivedCredit may be reversed.
    pub reversal_details: Option<stripe_treasury::TreasuryReceivedCreditsResourceReversalDetails>,
    /// Status of the ReceivedCredit.
    ///
    /// ReceivedCredits are created either `succeeded` (approved) or `failed` (declined).
    /// If a ReceivedCredit is declined, the failure reason can be found in the `failure_code` field.
    pub status: TreasuryReceivedCreditsResourceReceivedCreditStatus,
    /// The Transaction associated with this object.
    pub transaction: Option<stripe_types::Expandable<stripe_treasury::TreasuryTransactionsResourceTransaction>>,
}
/// Reason for the failure.
///
/// A ReceivedCredit might fail because the receiving FinancialAccount is closed or frozen.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryReceivedCreditsResourceReceivedCreditFailureCode {
    AccountClosed,
    AccountFrozen,
    Other,
}

impl TreasuryReceivedCreditsResourceReceivedCreditFailureCode {
    pub fn as_str(self) -> &'static str {
        use TreasuryReceivedCreditsResourceReceivedCreditFailureCode::*;
        match self {
            AccountClosed => "account_closed",
            AccountFrozen => "account_frozen",
            Other => "other",
        }
    }
}

impl std::str::FromStr for TreasuryReceivedCreditsResourceReceivedCreditFailureCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryReceivedCreditsResourceReceivedCreditFailureCode::*;
        match s {
            "account_closed" => Ok(AccountClosed),
            "account_frozen" => Ok(AccountFrozen),
            "other" => Ok(Other),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for TreasuryReceivedCreditsResourceReceivedCreditFailureCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryReceivedCreditsResourceReceivedCreditFailureCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryReceivedCreditsResourceReceivedCreditFailureCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryReceivedCreditsResourceReceivedCreditFailureCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryReceivedCreditsResourceReceivedCreditFailureCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryReceivedCreditsResourceReceivedCreditFailureCode"))
    }
}
/// The rails used to send the funds.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryReceivedCreditsResourceReceivedCreditNetwork {
    Ach,
    Card,
    Stripe,
    UsDomesticWire,
}

impl TreasuryReceivedCreditsResourceReceivedCreditNetwork {
    pub fn as_str(self) -> &'static str {
        use TreasuryReceivedCreditsResourceReceivedCreditNetwork::*;
        match self {
            Ach => "ach",
            Card => "card",
            Stripe => "stripe",
            UsDomesticWire => "us_domestic_wire",
        }
    }
}

impl std::str::FromStr for TreasuryReceivedCreditsResourceReceivedCreditNetwork {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryReceivedCreditsResourceReceivedCreditNetwork::*;
        match s {
            "ach" => Ok(Ach),
            "card" => Ok(Card),
            "stripe" => Ok(Stripe),
            "us_domestic_wire" => Ok(UsDomesticWire),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for TreasuryReceivedCreditsResourceReceivedCreditNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryReceivedCreditsResourceReceivedCreditNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryReceivedCreditsResourceReceivedCreditNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryReceivedCreditsResourceReceivedCreditNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryReceivedCreditsResourceReceivedCreditNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryReceivedCreditsResourceReceivedCreditNetwork"))
    }
}
/// Status of the ReceivedCredit.
///
/// ReceivedCredits are created either `succeeded` (approved) or `failed` (declined).
/// If a ReceivedCredit is declined, the failure reason can be found in the `failure_code` field.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryReceivedCreditsResourceReceivedCreditStatus {
    Failed,
    Succeeded,
}

impl TreasuryReceivedCreditsResourceReceivedCreditStatus {
    pub fn as_str(self) -> &'static str {
        use TreasuryReceivedCreditsResourceReceivedCreditStatus::*;
        match self {
            Failed => "failed",
            Succeeded => "succeeded",
        }
    }
}

impl std::str::FromStr for TreasuryReceivedCreditsResourceReceivedCreditStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryReceivedCreditsResourceReceivedCreditStatus::*;
        match s {
            "failed" => Ok(Failed),
            "succeeded" => Ok(Succeeded),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for TreasuryReceivedCreditsResourceReceivedCreditStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryReceivedCreditsResourceReceivedCreditStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryReceivedCreditsResourceReceivedCreditStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryReceivedCreditsResourceReceivedCreditStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryReceivedCreditsResourceReceivedCreditStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryReceivedCreditsResourceReceivedCreditStatus"))
    }
}
impl stripe_types::Object for TreasuryReceivedCreditsResourceReceivedCredit {
    type Id = stripe_treasury::treasury_received_credits_resource_received_credit::TreasuryReceivedCreditId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(TreasuryReceivedCreditId);
#[cfg(feature = "treasury_received_credits_resource_received_credit")]
mod requests;
#[cfg(feature = "treasury_received_credits_resource_received_credit")]
pub use requests::*;
