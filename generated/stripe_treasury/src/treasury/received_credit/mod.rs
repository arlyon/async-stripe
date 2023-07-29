/// ReceivedCredits represent funds sent to a [FinancialAccount](https://stripe.com/docs/api#financial_accounts) (for example, via ACH or wire).
///
/// These money movements are not initiated from the FinancialAccount.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ReceivedCredit {
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
pub failure_code: Option<ReceivedCreditFailureCode>,
    /// The FinancialAccount that received the funds.
pub financial_account: Option<String>,
    /// A [hosted transaction receipt](https://stripe.com/docs/treasury/moving-money/regulatory-receipts) URL that is provided when money movement is considered regulated under Stripe's money transmission licenses.
pub hosted_regulatory_receipt_url: Option<String>,
    /// Unique identifier for the object.
pub id: stripe_treasury::treasury::received_credit::TreasuryReceivedCreditId,
pub initiating_payment_method_details: stripe_treasury::treasury::received_credit::initiating_payment_method_details::InitiatingPaymentMethodDetails,
pub linked_flows: stripe_treasury::treasury::received_credit::linked_flows::LinkedFlows,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
pub livemode: bool,
    /// The rails used to send the funds.
pub network: ReceivedCreditNetwork,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
pub object: ReceivedCreditObject,
    /// Details describing when a ReceivedCredit may be reversed.
pub reversal_details: Option<stripe_treasury::treasury::received_credit::reversal_details::ReversalDetails>,
    /// Status of the ReceivedCredit.
    ///
    /// ReceivedCredits are created either `succeeded` (approved) or `failed` (declined).
    /// If a ReceivedCredit is declined, the failure reason can be found in the `failure_code` field.
pub status: ReceivedCreditStatus,
    /// The Transaction associated with this object.
pub transaction: Option<stripe_types::Expandable<stripe_treasury::treasury::transaction::Transaction>>,

}
/// Reason for the failure.
///
/// A ReceivedCredit might fail because the receiving FinancialAccount is closed or frozen.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ReceivedCreditFailureCode {
    AccountClosed,
    AccountFrozen,
    Other,
}

impl ReceivedCreditFailureCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AccountClosed => "account_closed",
            Self::AccountFrozen => "account_frozen",
            Self::Other => "other",
        }
    }
}

impl std::str::FromStr for ReceivedCreditFailureCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "account_closed" => Ok(Self::AccountClosed),
            "account_frozen" => Ok(Self::AccountFrozen),
            "other" => Ok(Self::Other),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for ReceivedCreditFailureCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ReceivedCreditFailureCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for ReceivedCreditFailureCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ReceivedCreditFailureCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ReceivedCreditFailureCode"))
    }
}
/// The rails used to send the funds.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ReceivedCreditNetwork {
    Ach,
    Card,
    Stripe,
    UsDomesticWire,
}

impl ReceivedCreditNetwork {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ach => "ach",
            Self::Card => "card",
            Self::Stripe => "stripe",
            Self::UsDomesticWire => "us_domestic_wire",
        }
    }
}

impl std::str::FromStr for ReceivedCreditNetwork {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ach" => Ok(Self::Ach),
            "card" => Ok(Self::Card),
            "stripe" => Ok(Self::Stripe),
            "us_domestic_wire" => Ok(Self::UsDomesticWire),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for ReceivedCreditNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ReceivedCreditNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for ReceivedCreditNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ReceivedCreditNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ReceivedCreditNetwork"))
    }
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ReceivedCreditObject {
    TreasuryReceivedCredit,
}

impl ReceivedCreditObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TreasuryReceivedCredit => "treasury.received_credit",
        }
    }
}

impl std::str::FromStr for ReceivedCreditObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "treasury.received_credit" => Ok(Self::TreasuryReceivedCredit),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for ReceivedCreditObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ReceivedCreditObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for ReceivedCreditObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ReceivedCreditObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ReceivedCreditObject"))
    }
}
/// Status of the ReceivedCredit.
///
/// ReceivedCredits are created either `succeeded` (approved) or `failed` (declined).
/// If a ReceivedCredit is declined, the failure reason can be found in the `failure_code` field.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ReceivedCreditStatus {
    Failed,
    Succeeded,
}

impl ReceivedCreditStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Failed => "failed",
            Self::Succeeded => "succeeded",
        }
    }
}

impl std::str::FromStr for ReceivedCreditStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "failed" => Ok(Self::Failed),
            "succeeded" => Ok(Self::Succeeded),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for ReceivedCreditStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ReceivedCreditStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for ReceivedCreditStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ReceivedCreditStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ReceivedCreditStatus"))
    }
}
impl stripe_types::Object for ReceivedCredit {
    type Id = stripe_treasury::treasury::received_credit::TreasuryReceivedCreditId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(TreasuryReceivedCreditId);
pub mod linked_flows;
pub use linked_flows::LinkedFlows;
pub mod reversal_details;
pub use reversal_details::ReversalDetails;
pub mod status_transitions;
pub use status_transitions::StatusTransitions;
pub mod initiating_payment_method_details;
pub use initiating_payment_method_details::InitiatingPaymentMethodDetails;
