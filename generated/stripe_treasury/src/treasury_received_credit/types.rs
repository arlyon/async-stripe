/// ReceivedCredits represent funds sent to a [FinancialAccount](https://stripe.com/docs/api#financial_accounts) (for example, via ACH or wire).
/// These money movements are not initiated from the FinancialAccount.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TreasuryReceivedCredit {
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
    /// A ReceivedCredit might fail because the receiving FinancialAccount is closed or frozen.
pub failure_code: Option<TreasuryReceivedCreditFailureCode>,
    /// The FinancialAccount that received the funds.
pub financial_account: Option<String>,
        /// A [hosted transaction receipt](https://stripe.com/docs/treasury/moving-money/regulatory-receipts) URL that is provided when money movement is considered regulated under Stripe's money transmission licenses.
pub hosted_regulatory_receipt_url: Option<String>,
    /// Unique identifier for the object.
pub id: stripe_treasury::TreasuryReceivedCreditId,
pub initiating_payment_method_details: stripe_treasury::TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails,
pub linked_flows: stripe_treasury::TreasuryReceivedCreditsResourceLinkedFlows,
        /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
pub livemode: bool,
    /// The rails used to send the funds.
pub network: TreasuryReceivedCreditNetwork,
    /// Details describing when a ReceivedCredit may be reversed.
pub reversal_details: Option<stripe_treasury::TreasuryReceivedCreditsResourceReversalDetails>,
        /// Status of the ReceivedCredit.
    /// ReceivedCredits are created either `succeeded` (approved) or `failed` (declined).
    /// If a ReceivedCredit is declined, the failure reason can be found in the `failure_code` field.
pub status: stripe_treasury::TreasuryReceivedCreditStatus,
    /// The Transaction associated with this object.
pub transaction: Option<stripe_types::Expandable<stripe_treasury::TreasuryTransaction>>,

}
/// Reason for the failure.
/// A ReceivedCredit might fail because the receiving FinancialAccount is closed or frozen.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryReceivedCreditFailureCode {
    AccountClosed,
    AccountFrozen,
    Other,
}
impl TreasuryReceivedCreditFailureCode {
    pub fn as_str(self) -> &'static str {
        use TreasuryReceivedCreditFailureCode::*;
        match self {
            AccountClosed => "account_closed",
            AccountFrozen => "account_frozen",
            Other => "other",
        }
    }
}

impl std::str::FromStr for TreasuryReceivedCreditFailureCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryReceivedCreditFailureCode::*;
        match s {
            "account_closed" => Ok(AccountClosed),
            "account_frozen" => Ok(AccountFrozen),
            "other" => Ok(Other),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for TreasuryReceivedCreditFailureCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryReceivedCreditFailureCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryReceivedCreditFailureCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryReceivedCreditFailureCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for TreasuryReceivedCreditFailureCode")
        })
    }
}
/// The rails used to send the funds.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryReceivedCreditNetwork {
    Ach,
    Card,
    Stripe,
    UsDomesticWire,
}
impl TreasuryReceivedCreditNetwork {
    pub fn as_str(self) -> &'static str {
        use TreasuryReceivedCreditNetwork::*;
        match self {
            Ach => "ach",
            Card => "card",
            Stripe => "stripe",
            UsDomesticWire => "us_domestic_wire",
        }
    }
}

impl std::str::FromStr for TreasuryReceivedCreditNetwork {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryReceivedCreditNetwork::*;
        match s {
            "ach" => Ok(Ach),
            "card" => Ok(Card),
            "stripe" => Ok(Stripe),
            "us_domestic_wire" => Ok(UsDomesticWire),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for TreasuryReceivedCreditNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryReceivedCreditNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryReceivedCreditNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryReceivedCreditNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for TreasuryReceivedCreditNetwork")
        })
    }
}
impl stripe_types::Object for TreasuryReceivedCredit {
    type Id = stripe_treasury::TreasuryReceivedCreditId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(TreasuryReceivedCreditId);
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryReceivedCreditStatus {
    Failed,
    Succeeded,
}
impl TreasuryReceivedCreditStatus {
    pub fn as_str(self) -> &'static str {
        use TreasuryReceivedCreditStatus::*;
        match self {
            Failed => "failed",
            Succeeded => "succeeded",
        }
    }
}

impl std::str::FromStr for TreasuryReceivedCreditStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryReceivedCreditStatus::*;
        match s {
            "failed" => Ok(Failed),
            "succeeded" => Ok(Succeeded),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for TreasuryReceivedCreditStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryReceivedCreditStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryReceivedCreditStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryReceivedCreditStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for TreasuryReceivedCreditStatus"))
    }
}
