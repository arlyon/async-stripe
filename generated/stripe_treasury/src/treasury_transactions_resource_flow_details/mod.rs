#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TreasuryTransactionsResourceFlowDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_reversal: Option<stripe_treasury::TreasuryReceivedCreditsResourceCreditReversal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debit_reversal: Option<stripe_treasury::TreasuryReceivedDebitsResourceDebitReversal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_transfer: Option<stripe_treasury::TreasuryInboundTransfersResourceInboundTransfer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing_authorization: Option<stripe_types::IssuingAuthorization>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_payment: Option<stripe_treasury::TreasuryOutboundPaymentsResourceOutboundPayment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_transfer: Option<stripe_treasury::TreasuryOutboundTransfersResourceOutboundTransfer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_credit: Option<stripe_treasury::TreasuryReceivedCreditsResourceReceivedCredit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_debit: Option<stripe_treasury::TreasuryReceivedDebitsResourceReceivedDebit>,
    /// Type of the flow that created the Transaction.
    ///
    /// Set to the same value as `flow_type`.
    #[serde(rename = "type")]
    pub type_: TreasuryTransactionsResourceFlowDetailsType,
}
/// Type of the flow that created the Transaction.
///
/// Set to the same value as `flow_type`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryTransactionsResourceFlowDetailsType {
    CreditReversal,
    DebitReversal,
    InboundTransfer,
    IssuingAuthorization,
    Other,
    OutboundPayment,
    OutboundTransfer,
    ReceivedCredit,
    ReceivedDebit,
}

impl TreasuryTransactionsResourceFlowDetailsType {
    pub fn as_str(self) -> &'static str {
        use TreasuryTransactionsResourceFlowDetailsType::*;
        match self {
            CreditReversal => "credit_reversal",
            DebitReversal => "debit_reversal",
            InboundTransfer => "inbound_transfer",
            IssuingAuthorization => "issuing_authorization",
            Other => "other",
            OutboundPayment => "outbound_payment",
            OutboundTransfer => "outbound_transfer",
            ReceivedCredit => "received_credit",
            ReceivedDebit => "received_debit",
        }
    }
}

impl std::str::FromStr for TreasuryTransactionsResourceFlowDetailsType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryTransactionsResourceFlowDetailsType::*;
        match s {
            "credit_reversal" => Ok(CreditReversal),
            "debit_reversal" => Ok(DebitReversal),
            "inbound_transfer" => Ok(InboundTransfer),
            "issuing_authorization" => Ok(IssuingAuthorization),
            "other" => Ok(Other),
            "outbound_payment" => Ok(OutboundPayment),
            "outbound_transfer" => Ok(OutboundTransfer),
            "received_credit" => Ok(ReceivedCredit),
            "received_debit" => Ok(ReceivedDebit),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for TreasuryTransactionsResourceFlowDetailsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryTransactionsResourceFlowDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryTransactionsResourceFlowDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryTransactionsResourceFlowDetailsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryTransactionsResourceFlowDetailsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryTransactionsResourceFlowDetailsType"))
    }
}
